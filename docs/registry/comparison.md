# Comparison

The comparison takes two snapshots, before and after running a sample, and
finds every key and value that changed. The result is a diff, saved as JSON.


## Changes

Every key path of both snapshots is checked once:

| Key in 1st shot | Key in 2nd shot | Result |
| --- | --- | --- |
| Yes | No | Key deleted, and its values deleted |
| No | Yes | Key added, and its values added |
| Yes | Yes | Its values are compared |

Values are compared by name inside each key:

| Value in 1st shot | Value in 2nd shot | Result |
| --- | --- | --- |
| Yes | No | Value deleted |
| No | Yes | Value added |
| Yes | Yes, different | Value modified, with both values |
| Yes | Yes, same | Nothing |

A value is different when its type or its data changes.


## Keys Not Compared

A key that could not be read in one of the shots cannot be compared: what is
below it is unknown in that shot.

Without care, its whole subtree would show up as added or deleted. For
example, a key unreadable in the first shot and readable in the second would
look like hundreds of new keys.

To avoid it, Rustshot leaves out every key that was inaccessible in either
shot, together with everything below it, and lists them as not compared.


### In Which Shot

Each key not compared says in which shot it could not be read:

| Case | Meaning |
| --- | --- |
| `before` | Unreadable in the 1st shot only |
| `after` | Unreadable in the 2nd shot only |
| `both` | Unreadable in both shots |

`both` is the usual case, such as `HKLM\SAM\SAM`, which is never readable.

`after` is the one to look at: the key could be read before the sample ran and
not after. The sample may have changed its permissions to protect itself. The
graphical interface marks it with an amber `!`, and the command line counts it
apart.


## Summary

Every diff carries a summary with the number of each kind of change:

| Field | Counts |
| --- | --- |
| `keys_added` | Keys added |
| `keys_deleted` | Keys deleted |
| `values_added` | Values added |
| `values_deleted` | Values deleted |
| `values_modified` | Values modified |
| `not_compared` | Keys not compared |
| `total` | Every change, without the keys not compared |

The JSON shape of the diff is in [Diff](../formats/diff.md).
