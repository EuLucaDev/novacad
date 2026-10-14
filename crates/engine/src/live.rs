//! Colaboração em tempo real (Nova Live) no motor do Nova CAD.
//!
//! - Comandos que entram no desfazer viram [`LiveChange::Op`] com a seleção (handles) de quem
//!   editou, para o colega rodar o mesmo comando nas mesmas entidades.
//! - Qualquer outra mudança do desenho (desfazer, refazer, grips, edições sem comando) vira
//!   [`LiveChange::Replace`]: o desenho mudou sem um comando replicável.
//! - Zoom, rolagem, vistas e seleção são pessoais. O documento viaja como DXF (o formato nativo;
//!   a gravação não inclui data e hora, então a impressão digital é estável).

use std::sync::Arc;

use novacad_doc::{Drawing, Handle};
use serde_json::{Value, json};

use crate::{EngineError, Result, Session};

#[derive(Clone, Debug, PartialEq)]
pub enum LiveChange {
    Op { cmd: String, params: Value, ctx: Value },
    Replace,
}

#[derive(Default)]
pub struct LiveState {
    pub doc: Option<u64>,
    pub outbox: Vec<LiveChange>,
    pub(crate) applying: bool,
    /// O desenho já informado à sessão (o documento só muda de objeto quando o conteúdo muda).
    pub(crate) seen: Option<Arc<Drawing>>,
}

const NAME: &str = "compartilhado.dxf";

fn err(e: impl std::fmt::Display) -> EngineError {
    EngineError::Other(format!("colaboração: {e}"))
}

/// Impressão digital estável (FNV-1a 64).
pub fn fnv64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

// Only audited document edits may execute on a peer. Other local edits travel as data.
fn remote_command_allowed(cmd: &str) -> bool {
    matches!(cmd, "line" | "circle")
}

impl Session {
    fn live_index(&self) -> Result<usize> {
        let uid = self.live.doc.ok_or_else(|| err("nenhuma sessão"))?;
        self.docs.iter().position(|d| d.uid == uid).ok_or_else(|| err("o desenho compartilhado foi fechado"))
    }

    pub fn live_start(&mut self) -> Result<()> {
        let st = self.state()?;
        self.live = LiveState { doc: Some(st.uid), seen: Some(st.doc.clone()), ..LiveState::default() };
        Ok(())
    }

    pub fn live_stop(&mut self) {
        self.live = LiveState::default();
    }

    pub fn live_active(&self) -> bool {
        self.live.doc.is_some()
    }

    pub fn live_take(&mut self) -> Vec<LiveChange> {
        if let Ok(i) = self.live_index()
            && let Some(st) = self.docs.get(i)
            && !self.live.seen.as_ref().is_some_and(|s| Arc::ptr_eq(s, &st.doc))
        {
            self.live.seen = Some(st.doc.clone());
            self.live.outbox.push(LiveChange::Replace);
        }
        std::mem::take(&mut self.live.outbox)
    }

    /// Um comando interativo em andamento (pedindo pontos): o desenho inteiro espera ele terminar.
    pub fn live_busy(&self) -> bool {
        self.running.is_some()
    }

    pub fn live_snapshot(&self) -> Result<Vec<u8>> {
        let st = self.docs.get(self.live_index()?).ok_or_else(|| err("sem desenho"))?;
        novacad_io::write(&st.doc, NAME).map_err(err)
    }

    pub fn live_digest(&self) -> Result<u64> {
        Ok(fnv64(&self.live_snapshot()?))
    }

    pub fn live_load(&mut self, bytes: &[u8]) -> Result<()> {
        let i = self.live_index()?;
        let d = novacad_io::read(bytes, NAME).map_err(err)?;
        let st = self.docs.get_mut(i).ok_or_else(|| err("sem desenho"))?;
        st.doc = Arc::new(d);
        st.revision += 1;
        let doc = st.doc.clone();
        st.selection.retain(|h| doc.entity(*h).is_some());
        st.previous_selection.retain(|h| doc.entity(*h).is_some());
        self.live.seen = Some(doc);
        Ok(())
    }

    /// Roda o comando de um colega com a seleção dele; desfazer e seleção locais ficam como estavam.
    pub fn live_apply(&mut self, cmd: &str, params: &Value, ctx: &Value) -> Result<()> {
        if !remote_command_allowed(cmd) {
            return Err(err("comando não permitido na colaboração"));
        }
        let i = self.live_index()?;
        let prev = self.active;
        self.active = i;
        let (local_sel, undo_len) = {
            let st = self.state()?;
            (st.selection.clone(), st.undo.len())
        };
        if let Some(sel) = ctx.get("selection").and_then(|s| serde_json::from_value::<Vec<Handle>>(s.clone()).ok()) {
            self.state_mut()?.selection = sel;
        }
        self.live.applying = true;
        let r = self.execute(cmd, params);
        self.live.applying = false;
        if let Ok(st) = self.state_mut() {
            st.undo.truncate(undo_len);
            let doc = st.doc.clone();
            st.selection = local_sel.into_iter().filter(|h| doc.entity(*h).is_some()).collect();
            self.live.seen = Some(doc);
        }
        if prev < self.docs.len() {
            self.active = prev;
        }
        r.map(|_| ())
    }

    /// Chamado por `execute` depois de cada comando.
    pub(crate) fn live_record(&mut self, id: &str, params: &Value, undoable: bool, ctx: Value) {
        if self.live.applying {
            return;
        }
        let Ok(i) = self.live_index() else { return };
        let Some(st) = self.docs.get(i) else { return };
        if self.live.seen.as_ref().is_some_and(|s| Arc::ptr_eq(s, &st.doc)) {
            return;
        }
        if undoable && remote_command_allowed(id) && !matches!(id, "undo" | "u" | "redo") {
            let doc = st.doc.clone();
            self.live.outbox.push(LiveChange::Op { cmd: id.to_string(), params: params.clone(), ctx });
            self.live.seen = Some(doc);
        }
        // Não replicável: `live_take` manda o desenho inteiro.
    }
}

pub(crate) fn context(selection: &[Handle]) -> Value {
    json!({ "selection": selection })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
    use super::*;

    #[test]
    fn remote_commands_cannot_open_export_or_execute_code() {
        let (_, mut b) = pair();
        let before = b.live_digest().unwrap();
        for command in ["file.open", "file.saveAs", "file.export", "session.open", "script.run", "app.quit", "lisp.eval"] {
            assert!(b.live_apply(command, &serde_json::json!({"path": "/tmp/not-authorized"}), &serde_json::json!({})).is_err());
            assert_eq!(before, b.live_digest().unwrap());
        }
    }

    fn pair() -> (Session, Session) {
        let mut a = Session::new();
        a.live_start().unwrap();
        let mut b = Session::new();
        b.live_start().unwrap();
        let snap = a.live_snapshot().unwrap();
        b.live_load(&snap).unwrap();
        (a, b)
    }

    fn sync(a: &mut Session, b: &mut Session) {
        for change in a.live_take() {
            match change {
                LiveChange::Op { cmd, params, ctx } => b.live_apply(&cmd, &params, &ctx).unwrap(),
                LiveChange::Replace => {
                    let snap = a.live_snapshot().unwrap();
                    b.live_load(&snap).unwrap();
                }
            }
        }
        assert!(b.live_take().is_empty(), "o que veio de A não volta para A");
    }

    #[test]
    fn entities_replicate_and_undo_travels_as_a_drawing() {
        let (mut a, mut b) = pair();
        a.execute("line", &json!({"points": [[0, 0], [100, 50]]})).unwrap();
        a.execute("circle", &json!({"center": [50, 50], "radius": 25})).unwrap();
        sync(&mut a, &mut b);
        assert_eq!(a.live_digest().unwrap(), b.live_digest().unwrap());
        assert!(b.state().unwrap().undo.is_empty());

        a.execute("undo", &json!({})).unwrap();
        sync(&mut a, &mut b);
        assert_eq!(a.live_digest().unwrap(), b.live_digest().unwrap());

        b.execute("line", &json!({"points": [[10, 10], [20, 30]]})).unwrap();
        sync(&mut b, &mut a);
        assert_eq!(a.live_digest().unwrap(), b.live_digest().unwrap());
    }
}
