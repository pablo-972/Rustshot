# Building

Rustshot is built on Linux and cross-compiled to Windows with MinGW. The
result is a static `.exe` with no extra DLLs.

Rustshot only runs on Windows, so every build targets Windows, also the ones
for checking the code.


## Requirements

| Tool | Used for |
| --- | --- |
| Rust stable, with the `x86_64-pc-windows-gnu` target | Daily builds, checks and tests |
| Rust nightly, with the `rust-src` component | Windows 7 builds |
| MinGW-w64, 64 and 32 bit | C and C++ compilers, linker and `windres` |
| CMake | Building FLTK, used by the graphical interface |
| Wine | Running the tests on Linux |
| ImageMagick | Only to regenerate the icon |

On Debian or Ubuntu:

```bash
sudo apt install mingw-w64 cmake wine imagemagick
```

```bash
rustup target add x86_64-pc-windows-gnu
```

```bash
rustup toolchain install nightly --component rust-src
```


## Builds

| Build | Windows | Toolchain |
| --- | --- | --- |
| `x86_64-win7-windows-gnu` | 64-bit Windows 7 to 11 | Nightly |
| `i686-win7-windows-gnu` | 32-bit Windows 7 to 11 | Nightly |
| `x86_64-pc-windows-gnu` | 64-bit Windows 8 to 11 | Stable |

The Windows 7 builds are the ones released. Rust has no ready-made standard
library for them, so it is built from source with `-Zbuild-std`.

**64-bit**

```bash
cargo +nightly build -Zbuild-std --target x86_64-win7-windows-gnu --release
```

**32-bit**

```bash
cargo +nightly build -Zbuild-std --target i686-win7-windows-gnu --release
```

The `.exe` is written to:

```text
target/<target>/release/Rustshot.exe
```


## Daily Work

The default target is `x86_64-pc-windows-gnu`, set in `.cargo/config.toml`, so
the usual commands need no options:

```bash
cargo build
```

```bash
cargo clippy --workspace --all-targets
```

rust-analyzer uses the same target.


## Tests

```bash
cargo test --workspace
```

The tests are compiled for Windows and run with Wine, set as the runner in
`.cargo/config.toml`. They do not touch the real registry or open windows, so
they take milliseconds.

The first run builds FLTK for the default target and takes a few minutes.


## Configuration

`.cargo/config.toml` holds every build setting:

| Setting | Purpose |
| --- | --- |
| `[build] target` | Windows as the default target |
| `runner = "wine"` | Runs the tests and `cargo run` with Wine |
| `crt-static`, `-static` | Puts the C runtime and MinGW libraries inside the `.exe` |
| `--large-address-aware` | Lets the 32-bit `.exe` use 4 GB on 64-bit Windows |
| `CC_*`, `CXX_*`, `AR_*` | MinGW compilers for the Windows 7 targets, which `cc` does not know |

The release profile, in the root `Cargo.toml`, uses link-time optimization and
strips the symbols, for a smaller and faster `.exe`.


## Icon and Version Details

`build.rs` puts the icon and the version details inside the `.exe`, with
`embed-resource` and `windres`:

```text
crates/gui/icons/
    rustshot.svg        Source of the icon
    rustshot.ico        Icon of the .exe, in several sizes
    rustshot.rc         Icon and version details
```

The version comes from `Cargo.toml`: `build.rs` writes it to a header that
`rustshot.rc` includes. Explorer shows it in **Properties → Details**.

To regenerate the `.ico` after changing the `.svg`:

```bash
cd crates/gui/icons
for size in 16 24 32 48 64 256; do
    convert -background none -density 384 rustshot.svg -resize ${size}x${size} /tmp/rustshot-$size.png
done
convert /tmp/rustshot-{16,24,32,48,64,256}.png rustshot.ico
```


## Releasing

1. Set the new version in `[workspace.package]` of the root `Cargo.toml`.
2. Build both Windows 7 targets.
3. Rename the 32-bit `.exe` to `Rustshot32.exe`.
4. Write the SHA-256 of both files to `SHA256SUMS.txt`.
5. Publish both files and the checksums in a GitHub release.

```bash
sha256sum Rustshot.exe Rustshot32.exe > SHA256SUMS.txt
```
