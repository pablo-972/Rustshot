use std::collections::BTreeMap;

use super::diff::union;
use crate::snapshot::types::{
    FileChange,
    FileEntry,
    FileItem,
    FilesDiff,
};


pub(super) fn compare(
    before: &BTreeMap<String, FileEntry>,
    after: &BTreeMap<String, FileEntry>,
) -> FilesDiff {
    let mut diff = FilesDiff::default();

    for path in union(before, after) {
        match (before.get(path), after.get(path)) {

            (Some(entry), None) => diff.deleted.push(FileItem {
                path: path.clone(),
                entry: entry.clone(),
            }),

            (None, Some(entry)) => diff.added.push(FileItem {
                path: path.clone(),
                entry: entry.clone(),
            }),

            (Some(old), Some(new)) if changed(old, new) => diff.modified.push(FileChange {
                path: path.clone(),
                before: old.clone(),
                after: new.clone(),
            }),

            _ => {}
        }
    }

    diff
}


// The modification time alone is not a change: files can be touched
// without changing their content.
fn changed(old: &FileEntry, new: &FileEntry) -> bool {
    old.kind != new.kind
}