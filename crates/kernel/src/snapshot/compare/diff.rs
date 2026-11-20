use std::collections::{BTreeMap, BTreeSet};

use super::{files, registry};
use crate::snapshot::types::{
    FORMAT_VERSION,
    Snapshot,
    SnapshotDiff,
    DiffChanges,
    DiffMeta,
    DiffSummary,
    RegistryDiff,
    FilesDiff,
    RegistryDiffSummary,
    FilesDiffSummary
};


pub fn compare(before: &Snapshot, after: &Snapshot) -> SnapshotDiff {
    let registry = registry::compare(&before.data.registry, &after.data.registry);
    let files = files::compare(&before.data.files, &after.data.files);
    let summary = summarize(&registry, &files);

    SnapshotDiff {
        meta: DiffMeta {
            format: FORMAT_VERSION,
            before: before.meta.created_at,
            after: after.meta.created_at,
        },
        summary,
        changes: DiffChanges { registry, files },
    }
}


fn summarize(registry: &RegistryDiff, files: &FilesDiff) -> DiffSummary {
    let registry = RegistryDiffSummary {
        keys_added: registry.keys_added.len(),
        keys_deleted: registry.keys_deleted.len(),
        values_added: registry.values_added.len(),
        values_deleted: registry.values_deleted.len(),
        values_modified: registry.values_modified.len(),
    };

    let files = FilesDiffSummary {
        added: files.added.len(),
        deleted: files.deleted.len(),
        modified: files.modified.len(),
    };
    
    let total = registry.keys_added
        + registry.keys_deleted
        + registry.values_added
        + registry.values_deleted
        + registry.values_modified
        + files.added
        + files.deleted
        + files.modified;

    DiffSummary { registry, files, total }
}


/// Keys of both maps, sorted and without duplicates.
pub(super) fn union<'a, V>(
    a: &'a BTreeMap<String, V>,
    b: &'a BTreeMap<String, V>,
) -> BTreeSet<&'a String> {
    a.keys().chain(b.keys()).collect()
}