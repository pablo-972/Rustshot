// Puts the icon inside the .exe when building for Windows.
fn main() {
    println!("cargo:rerun-if-changed=crates/gui/icons/rustshot.ico");

    embed_resource::compile("crates/gui/icons/rustshot.rc", embed_resource::NONE)
        .manifest_optional()
        .unwrap();
}
