# Troubleshooting

Known issues and their fixes, grouped by where they happen.


## Categories

- [Capture](#capture)
- [Graphical interface](#graphical-interface)
- [Building](#building)


## Capture

### Many inaccessible keys

Common symptoms:

- The count of inaccessible keys is in the hundreds or more.
- Whole branches of `HKLM` are missing from the snapshot.

Rustshot was not run as administrator. Right click the `.exe` and choose
**Run as administrator**, or open the terminal as administrator before using
the command line.

A few inaccessible keys, such as `HKLM\SAM\SAM` and `HKLM\SECURITY`, are
normal even as administrator: only `SYSTEM` can read them.


### Changes that are not real

Common symptoms:

- Hundreds of keys added or deleted under one branch.
- The branch exists in both shots on the system.

Make sure both shots were taken with the same rights. A branch readable in one
shot and not in the other is listed as not compared, but if the shots come
from different users, `HKU` holds different users and their keys differ.


### Windows says the app is not recognized

SmartScreen may warn about an unknown `.exe` downloaded from the internet.
Check its SHA-256 against `SHA256SUMS.txt` of the release, then choose
**More info → Run anyway**.

Inside an isolated analysis lab, copying the `.exe` from a shared folder
instead of downloading it avoids the warning.


## Graphical Interface

### The title bar stays light in dark mode

The dark title bar needs Windows 10 version 1809 or later. On older versions,
only the inside of the windows is dark.


### The window shows Working… for a long time

A full capture of a large registry can take a minute or more, and saving the
file can take several seconds more. The time below the field keeps counting
while it works.

If the time stops counting for more than a minute, the capture is stuck. Close
Rustshot and report the issue with the Windows version.


## Building

### FLTK is built with the Linux compiler

Common symptoms:

- Linker errors about ELF objects when building for a Windows 7 target.
- `cmake` reports a Linux C compiler for FLTK.

The `cc` crate does not know the Windows 7 targets. Check that the `CC_*`,
`CXX_*` and `AR_*` variables of `.cargo/config.toml` are there.

If the build failed once with the wrong compiler, CMake keeps it in its cache.
Remove the FLTK build folder of that target and build again:

```bash
rm -rf target/x86_64-win7-windows-gnu/release/build/fltk-sys-*
```


### Tests fail to start

Common symptoms:

- `cargo test` reports that the runner `wine` was not found.
- Wine asks for missing DLLs.

Install Wine, and check that the default target in `.cargo/config.toml` keeps
its static flags, so the test `.exe` does not need MinGW DLLs.
