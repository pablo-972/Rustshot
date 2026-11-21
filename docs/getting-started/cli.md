# Command Line

The command line runs the same capture and comparison as the graphical
interface. It is useful for scripts, remote sessions and automated labs.

Run it from `cmd.exe` or PowerShell, as administrator.


## Commands

| Command | Purpose |
| --- | --- |
| `shot` | Takes a snapshot of the registry and saves it |
| `diff` | Compares two snapshots and saves the differences |

Every command accepts `--help`. The version is shown with `--version`.


## shot

```text
Rustshot.exe shot [-o FILE]
```

| Option | Default | Purpose |
| --- | --- | --- |
| `-o`, `--output` | `snapshot.json` | File where the snapshot is saved |

While the capture runs, one line is written again over itself:

```text
Capturing... 123000 keys, 456789 values, 14 inaccessible, 12 s
```

When it finishes:

```text
Snapshot saved to before.json (312456 registry keys, 612345 values, 14 inaccessible)
```


## diff

```text
Rustshot.exe diff BEFORE AFTER [-o FILE]
```

| Argument | Default | Purpose |
| --- | --- | --- |
| `BEFORE` | | Snapshot taken before running the sample |
| `AFTER` | | Snapshot taken after running the sample |
| `-o`, `--output` | `diff.json` | File where the diff is saved |

The summary of the changes is written when it finishes:

```text
Keys added:      12
Keys deleted:    0
Values added:    35
Values deleted:  2
Values modified: 118
Not compared:    14 (2 unreadable only in the 2nd shot)
Diff saved to    diff.json
```

The part in brackets only appears when some key became unreadable in the
second shot. See [Registry](../registry/comparison.md) for what that means.


## Example

```powershell
.\Rustshot.exe shot -o before.json
```

Run the sample, then:

```powershell
.\Rustshot.exe shot -o after.json
```

```powershell
.\Rustshot.exe diff before.json after.json -o diff.json
```

The diff can then be opened in the graphical interface with
**File → Open diff…**, to review it with filters and search.


## Output

Messages and progress are written to stderr. The only output of Rustshot is
the JSON files, described in [File Formats](../formats/README.md).
