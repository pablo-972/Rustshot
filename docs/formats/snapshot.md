# Snapshot

A snapshot is the whole registry at one moment, with information about the
system it was taken on.


## Shape

```json
{
  "meta": {
    "created_at": 1763750400,
    "system": {
      "name": "Windows 10 Pro",
      "version": "10.0",
      "release": "22H2",
      "build": "19045.3803",
      "service_pack": null,
      "edition": "Professional",
      "architecture": "AMD64"
    },
    "device": {
      "hostname": "LAB-WIN10",
      "user": "analyst",
      "domain": "LAB-WIN10"
    }
  },
  "summary": {
    "registry_keys": 312456,
    "registry_values": 612345,
    "registry_inaccessible": 14
  },
  "data": {
    "registry": {
      "HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run": {
        "values": {
          "SecurityHealth": {
            "type": "ExpandSz",
            "data": "%windir%\\system32\\SecurityHealthSystray.exe"
          }
        }
      }
    },
    "inaccessible": [
      "HKLM\\SAM\\SAM",
      "HKLM\\SECURITY"
    ]
  }
}
```


## meta

| Field | Content |
| --- | --- |
| `created_at` | Capture time, in seconds since the Unix epoch |
| `system` | Windows version details, `null` when a value is missing |
| `device` | Host name, user and domain |

`version` is the kernel version, such as `6.1` for Windows 7 and `10.0` for
Windows 10 and 11. `build` includes the revision of the last cumulative
update.


## summary

| Field | Content |
| --- | --- |
| `registry_keys` | Keys captured |
| `registry_values` | Values captured, in all keys |
| `registry_inaccessible` | Keys that exist but could not be opened |


## data

| Field | Content |
| --- | --- |
| `registry` | Every key, by its full path, with its values by name |
| `inaccessible` | Paths of the keys that could not be opened, sorted |

The default value of a key has an empty name, `""`.


## Values

Each value has a `type` and its `data`:

| Type | Data | Example |
| --- | --- | --- |
| `Sz` | Text | `{ "type": "Sz", "data": "C:\\tool.exe" }` |
| `ExpandSz` | Text, variables not expanded | `{ "type": "ExpandSz", "data": "%TEMP%\\x" }` |
| `MultiSz` | List of texts | `{ "type": "MultiSz", "data": ["a", "b"] }` |
| `Dword` | Number | `{ "type": "Dword", "data": 1 }` |
| `Qword` | Number | `{ "type": "Qword", "data": 1700000000 }` |
| `Binary` | Bytes as lowercase hexadecimal | `{ "type": "Binary", "data": "0aff00" }` |
| `Other` | Type number and bytes | `{ "type": "Other", "data": { "kind": 11, "bytes": "0100" } }` |

`Other` holds any other registry type, and also values whose data does not
match their type, so no value is lost.
