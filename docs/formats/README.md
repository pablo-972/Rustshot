# File Formats

Rustshot saves everything as indented JSON, so the files can be read by hand,
kept as evidence or processed by other tools.

| File | Default name | Produced by |
| --- | --- | --- |
| Snapshot | `snapshot.json`, or `before.json` and `after.json` in the GUI | `shot`, **Take shot** |
| Diff | `diff.json` | `diff`, **Compare** |

Both files have the same three parts:

| Part | Content |
| --- | --- |
| `meta` | When and where the data comes from |
| `summary` | Counts, to know the size of the file without reading it all |
| `data` or `changes` | The data itself |


## Documents

- [Snapshot](snapshot.md)
- [Diff](diff.md)


## Compatibility

Fields added after the first files were written have default values, so older
snapshots can still be opened and compared. A missing list is read as empty,
and a missing count as zero.
