# Diff

A diff is the list of changes between two snapshots, with the counts of each
kind of change.


## Shape

```json
{
  "meta": {
    "before": 1763750400,
    "after": 1763750700
  },
  "summary": {
    "registry": {
      "keys_added": 1,
      "keys_deleted": 0,
      "values_added": 1,
      "values_deleted": 0,
      "values_modified": 1,
      "not_compared": 1
    },
    "total": 3
  },
  "changes": {
    "registry": {
      "keys_added": [
        "HKU\\S-1-5-21-1004\\Software\\Updater"
      ],
      "keys_deleted": [],
      "values_added": [
        {
          "key": "HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run",
          "name": "Updater",
          "value": {
            "type": "Sz",
            "data": "C:\\Users\\analyst\\AppData\\Roaming\\upd.exe"
          }
        }
      ],
      "values_deleted": [],
      "values_modified": [
        {
          "key": "HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows Defender",
          "name": "DisableAntiSpyware",
          "before": { "type": "Dword", "data": 0 },
          "after": { "type": "Dword", "data": 1 }
        }
      ],
      "not_compared": [
        {
          "key": "HKLM\\SAM\\SAM",
          "unreadable_in": "both"
        }
      ]
    }
  }
}
```


## meta

| Field | Content |
| --- | --- |
| `before` | Capture time of the first snapshot |
| `after` | Capture time of the second snapshot |

Both are in seconds since the Unix epoch.


## summary

| Field | Content |
| --- | --- |
| `registry` | Count of each kind of change |
| `total` | Every change, without the keys not compared |


## changes

| Field | Content |
| --- | --- |
| `keys_added` | Paths of the keys added |
| `keys_deleted` | Paths of the keys deleted |
| `values_added` | Key, name and value of each value added |
| `values_deleted` | Key, name and value of each value deleted |
| `values_modified` | Key, name, and the value before and after |
| `not_compared` | Keys left out, and in which shot they could not be read |

Values have the same shape as in the [Snapshot](snapshot.md).

The values of an added key also appear in `values_added`, and those of a
deleted key in `values_deleted`, so every value change is in one place.

`unreadable_in` is `before`, `after` or `both`. See
[Comparison](../registry/comparison.md) for what each case means.
