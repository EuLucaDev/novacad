# Asset attribution

Every non-code asset in this repository (images, icons, fonts, example drawings, presets) is listed
here with its author, source and licence. `cargo xtask assets` (part of `cargo xtask ci`) fails if
an asset file is missing from this table.

**Policy (mandatory):** Nova CAD contains **no Autodesk, Adobe or Avid iconography, images,
artwork, fonts, hatch patterns, linetypes, templates or presets.** Every asset is original work by
Nova CAD contributors or third-party material under an open licence (OSI open source, public domain
/ CC0, or Creative Commons that allows redistribution). Screenshots of Autodesk software are never
committed. Font files are not added here: shared fonts live in
[EuLucaDev/novafoods-fonts](https://github.com/EuLucaDev/novafoods-fonts), an optional build input. The one
exception is the Nova Foods brand in `docs/brand/`: Nova Foods trademarks, not open source, used under
`docs/brand/LICENSE-brand.txt`.

Generated-in-code assets are original and have no file to list:
- the UI icon set (`crates/ui-egui/src/icons.rs`);
- the "Nova CAD Stroke" single-stroke drafting font (`crates/fonts/src/stroke.rs`);
- the standard linetype and hatch-pattern libraries (`crates/doc/src/library.rs`; industry-common
  names, our own dash/spacing values);
- the colour index palette, generated from its structure (`crates/color/src/lib.rs`);
- the sample drawings (`crates/engine/src/sample.rs`).

| Asset | Author | Source | Licence | Notes |
|---|---|---|---|---|
| `assets/app-icon/hicolor/128x128/apps/br.com.novafoods.novacad.png` | Nova CAD contributors | generated from `assets/app-icon/novacad-1024.png` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/hicolor/16x16/apps/br.com.novafoods.novacad.png` | Nova CAD contributors | generated from `assets/app-icon/novacad-1024.png` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/hicolor/24x24/apps/br.com.novafoods.novacad.png` | Nova CAD contributors | generated from `assets/app-icon/novacad-1024.png` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/hicolor/256x256/apps/br.com.novafoods.novacad.png` | Nova CAD contributors | generated from `assets/app-icon/novacad-1024.png` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/hicolor/32x32/apps/br.com.novafoods.novacad.png` | Nova CAD contributors | generated from `assets/app-icon/novacad-1024.png` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/hicolor/48x48/apps/br.com.novafoods.novacad.png` | Nova CAD contributors | generated from `assets/app-icon/novacad-1024.png` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/hicolor/512x512/apps/br.com.novafoods.novacad.png` | Nova CAD contributors | generated from `assets/app-icon/novacad-1024.png` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/hicolor/64x64/apps/br.com.novafoods.novacad.png` | Nova CAD contributors | generated from `assets/app-icon/novacad-1024.png` | MIT OR Apache-2.0 | Linux hicolor icon |
| `assets/app-icon/novacad.icns` | Nova CAD contributors | generated (packaging/icons.sh) | MIT OR Apache-2.0 | macOS icon |
| `assets/app-icon/novacad.ico` | Nova CAD contributors | generated (packaging/icons.sh) | MIT OR Apache-2.0 | Windows icon |
| `assets/app-icon/novacad-64.png` | Nova CAD contributors | generated | MIT OR Apache-2.0 | |
| `assets/app-icon/novacad-256.png` | Nova CAD contributors | generated | MIT OR Apache-2.0 | |
| `assets/app-icon/novacad-1024.png` | Nova CAD contributors | generated | MIT OR Apache-2.0 | |
| `assets/app-icon/novacad-macos-512.png` | Nova CAD contributors | generated (macOS margin variant) | MIT OR Apache-2.0 | |
| `packaging/macos/dmg/background.svg` | @XusBadia | original work for this repository (2026-10-08), "Nova CAD macOS DMG window background"; derived from the Nova CAD app icon (`assets/app-icon/`, referenced via `<image>`, not copied); text is outlined Inter / JetBrains Mono glyph paths (OFL fonts the project already uses); no third-party material modified | MIT OR Apache-2.0 (same as the code: `LICENSE-MIT`, `LICENSE-APACHE`) | Finder window background of the macOS DMG; `packaging/macos/dmg/generate.py` renders it to `background.tiff` (see `packaging/macos/dmg/README.md`) |
| `packaging/macos/dmg/background.tiff` | @XusBadia | Rendered from `packaging/macos/dmg/background.svg` by `packaging/macos/dmg/generate.py` | MIT OR Apache-2.0 (same as the code: `LICENSE-MIT`, `LICENSE-APACHE`) | 1x + 2x HiDPI TIFF for the macOS DMG window; includes the app icon (see its row) and LittleCMS's built-in sRGB profile ("No copyright, use freely") |
| `examples/apartment.dxf` | Nova CAD contributors | sample drawing made with novacad-cli commands | MIT OR Apache-2.0 | Original |
| `docs/images/ui-apartment.png` | Nova CAD contributors | screenshot of Nova CAD itself (examples/apartment.dxf) | MIT OR Apache-2.0 | Original; no Autodesk UI |
| `docs/images/ui-layout.png` | Nova CAD contributors | screenshot of Nova CAD itself (a layout of examples/apartment.dxf) | MIT OR Apache-2.0 | Original; no Autodesk UI |
| `docs/images/ui-bracket.png` | Nova CAD contributors | screenshot of Nova CAD itself (sample drawing generated in code) | MIT OR Apache-2.0 | Original; no Autodesk UI |
| `docs/brand/novafoods-logo-white.png` | Nova Foods Team | EuLucaDev/craftrules `assets/brand/` | Nova Foods brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |
| `docs/brand/novafoods-logo-white.svg` | Nova Foods Team | EuLucaDev/craftrules `assets/brand/` | Nova Foods brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |
| `docs/brand/novafoods-logo.png` | Nova Foods Team | EuLucaDev/craftrules `assets/brand/` | Nova Foods brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |
| `docs/brand/novafoods-logo.svg` | Nova Foods Team | EuLucaDev/craftrules `assets/brand/` | Nova Foods brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |
| `docs/brand/novafoods-mark-black.png` | Nova Foods Team | EuLucaDev/craftrules `assets/brand/` | Nova Foods brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |
| `docs/brand/novafoods-mark-black.svg` | Nova Foods Team | EuLucaDev/craftrules `assets/brand/` | Nova Foods brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |
| `docs/brand/novafoods-mark.png` | Nova Foods Team | EuLucaDev/craftrules `assets/brand/` | Nova Foods brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |
| `docs/brand/novafoods-mark.svg` | Nova Foods Team | EuLucaDev/craftrules `assets/brand/` | Nova Foods brand terms (`docs/brand/LICENSE-brand.txt`) | Trademark, not open source |

## Interface translations

| Asset | Author | Source | Licence | Notes |
|---|---|---|---|---|
| `crates/ui-egui/src/i18n/uk.tsv` | @dmatviichuk | Original Ukrainian translations of Nova CAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `crates/ui-egui/src/i18n/de.tsv` | the Nova CAD contributors | Original German translations of Nova CAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `crates/ui-egui/src/i18n/es.tsv` | @PedroGuille | Original Spanish translations of Nova CAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `crates/ui-egui/src/i18n/pt_br.tsv` | @WladmirJunior | Original Brazilian Portuguese translations of Nova CAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `crates/ui-egui/src/i18n/cs.tsv` | @Losminek (AI-assisted) | Original Czech translations of Nova CAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `crates/ui-egui/src/i18n/ru.tsv` | @pechnikov2026-star | Original Russian translations of Nova CAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `crates/ui-egui/src/i18n/fr.tsv` | @pechnikov2026-star | Original French translations of Nova CAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `crates/ui-egui/src/i18n/zh.tsv` | @mayloglog | Original Simplified Chinese translations of Nova CAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `crates/ui-egui/src/i18n/sv.tsv` | Erik Holmbom (@holmbom) with Claude Code (AI-assisted) | Original Swedish translations of Nova CAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied; terminology and the running app reviewed by a native speaker |
| `crates/ui-egui/src/i18n/it.tsv` | Emanuele Bottelli (AI-assisted) | Original Italian translations of Nova CAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
| `crates/ui-egui/src/i18n/ko.tsv` | @huk082 (AI-assisted) | Original Korean translations of Nova CAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied; no font assets added |
| `crates/ui-egui/src/i18n/ar.tsv` | @hasan-sdaif | Original Arabic translations of Nova CAD English strings | MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`) | No proprietary localization resources copied |
