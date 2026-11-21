# Roadmap

Ideas for future versions. None of them is planned for a date.


## File System Capture

Capture files and folders as a second source, next to the registry: path,
size, times and hash of each file. The diff would show files created, deleted
and modified by the sample.

The core is already organized by source, so it would be a new folder beside
`registry/`. See [Adding a Source](../architecture/README.md#adding-a-source).


## Permission Changes

Record the permissions of each key, to show when a sample changes them. Today
a key that becomes unreadable in the second shot is already flagged as not
compared with `after`, which covers the most suspicious case.


## Reports

Export a diff as an HTML report that can be shared without Rustshot, with the
same filters and colors as the results window.
