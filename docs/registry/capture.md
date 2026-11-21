# Capture

The capture walks the registry from its roots and stores every key with all
its values. The result is a snapshot, saved as JSON.


## Roots

| Root | Captured | Reason |
| --- | --- | --- |
| `HKLM` | Yes | The machine: software, services, drivers, system settings |
| `HKU` | Yes | Every user loaded in memory |
| `HKCU` | No | It is the current user's key inside `HKU`, already captured |
| `HKCR` | No | It is a merged view of `HKLM\SOFTWARE\Classes` and `HKCU\Software\Classes` |
| `HKCC` | No | It is a link to a key inside `HKLM` |

The roots are listed in `ROOTS`, in:

```text
crates/core/src/registry/capture.rs
```


## The Walk

Each key is read in the same order:

1. Its values are read and converted.
2. Its subkeys are listed and opened one by one.
3. Each subkey is walked the same way.
4. The key is stored with its values.

Keys are stored by their full path, such as
`HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\Run`.


## 64-bit View

On 64-bit Windows, a 32-bit program that opens `HKLM\SOFTWARE` is redirected to
`HKLM\SOFTWARE\Wow6432Node` without notice.

Rustshot opens every key with `KEY_WOW64_64KEY`, so both binaries read the real
64-bit view. `Wow6432Node` is still captured as a normal subkey, so the keys of
32-bit programs are not lost either.

On 32-bit Windows the flag has no effect.


## Values

Every value keeps its type:

| Registry type | Stored as |
| --- | --- |
| `REG_SZ` | `Sz`, text |
| `REG_EXPAND_SZ` | `ExpandSz`, text without expanding variables |
| `REG_MULTI_SZ` | `MultiSz`, list of texts |
| `REG_DWORD` | `Dword`, number |
| `REG_QWORD` | `Qword`, number |
| `REG_BINARY` | `Binary`, bytes as hexadecimal text |
| Any other | `Other`, type number and bytes |

A value whose data does not match its type is stored as `Other`, with its raw
bytes, so it is never lost.

The JSON shape of each type is in [Snapshot](../formats/snapshot.md).


## Inaccessible Keys

A subkey that is listed but cannot be opened is recorded as inaccessible. It
exists, but its access is usually denied.

Its path is saved in the snapshot, and its subkeys are not captured, because
they cannot be read.

Common examples:

| Key | Why |
| --- | --- |
| `HKLM\SAM\SAM` | Only readable by `SYSTEM` |
| `HKLM\SECURITY` | Only readable by `SYSTEM` |
| Many keys under `HKLM` | Rustshot was not run as administrator |

See [Comparison](comparison.md) for how they are handled in the diff.


## System Information

Each snapshot also records where it was taken:

| Field | Source |
| --- | --- |
| Windows name, version, release, build, service pack, edition | `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion` |
| Architecture | `PROCESSOR_ARCHITEW6432` or `PROCESSOR_ARCHITECTURE` |
| Host name, user, domain | `COMPUTERNAME`, `USERNAME` and `USERDOMAIN` |
| Capture time | System clock, in seconds since the Unix epoch |

On Windows 11, the name still says Windows 10: the build, 22000 or higher,
tells them apart.


## Progress

While the walk runs, Rustshot reports how many keys, values and inaccessible
keys it has found so far, every 1,000 keys and once more at the end.

Both interfaces show it: the graphical interface below the field of the shot,
and the command line on a line written again over itself.
