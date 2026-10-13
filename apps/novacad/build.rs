//! Windows only: embed the app icon and version info into `novacad.exe`. Elsewhere a no-op.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/novacad.ico");
    println!("cargo:rerun-if-env-changed=NOVACAD_REQUIRE_WINRES");
    println!("cargo:rerun-if-env-changed=NOVACAD_BUILD_SHA");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    if std::path::Path::new("../../assets/app-icon/novacad.ico").exists() {
        res.set_icon("../../assets/app-icon/novacad.ico");
    }
    res.set("ProductName", "Nova CAD")
        .set("FileDescription", "Nova CAD computer-aided design")
        .set("CompanyName", "Learning Machines LLC")
        .set("LegalCopyright", "Copyright (c) 2026 Nova Foods Team and the Nova CAD contributors. MIT OR Apache-2.0.")
        .set("OriginalFilename", "novacad.exe")
        .set("InternalName", "novacad");
    if let Err(e) = res.compile() {
        if std::env::var_os("NOVACAD_REQUIRE_WINRES").is_some() {
            eprintln!("embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=novacad.exe built without icon/version resources: {e}");
    }
}
