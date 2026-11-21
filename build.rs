use std::env;
use std::fs;
use std::path::PathBuf;


// Puts the icon and the version details inside the .exe. The version is the
// one of Cargo.toml, written to a header the .rc file includes, so it is
// written in one place only.
fn main() {
    println!("cargo:rerun-if-changed=crates/gui/icons/rustshot.ico");
    println!("cargo:rerun-if-changed=crates/gui/icons/rustshot.rc");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let version = |part| env::var(part).unwrap();

    let header = format!(
        "#define VERSION_MAJOR {}\n#define VERSION_MINOR {}\n#define VERSION_PATCH {}\n\
         #define VERSION \"{}\"\n",
        version("CARGO_PKG_VERSION_MAJOR"),
        version("CARGO_PKG_VERSION_MINOR"),
        version("CARGO_PKG_VERSION_PATCH"),
        version("CARGO_PKG_VERSION")
    );
    fs::write(out.join("version.h"), header).unwrap();

    let includes = embed_resource::ParamsIncludeDirs([&out]);
    embed_resource::compile("crates/gui/icons/rustshot.rc", includes)
        .manifest_optional()
        .unwrap();
}
