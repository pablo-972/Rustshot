use std::collections::{BTreeMap, BTreeSet};

use super::registry;
use crate::types::{
    DiffChanges,
    DiffMeta,
    DiffSummary,
    RegistryDiff,
    RegistryDiffSummary,
    Snapshot,
    SnapshotDiff,
};


pub fn compare(before: &Snapshot, after: &Snapshot) -> SnapshotDiff {
    let registry = registry::compare(&before.data.registry, &after.data.registry);
    let summary = summarize(&registry);

    SnapshotDiff {
        meta: DiffMeta {
            before: before.meta.created_at,
            after: after.meta.created_at,
        },
        summary,
        changes: DiffChanges { registry },
    }
}


fn summarize(registry: &RegistryDiff) -> DiffSummary {
    let registry = RegistryDiffSummary {
        keys_added: registry.keys_added.len(),
        keys_deleted: registry.keys_deleted.len(),
        values_added: registry.values_added.len(),
        values_deleted: registry.values_deleted.len(),
        values_modified: registry.values_modified.len(),
    };

    let total = registry.keys_added
        + registry.keys_deleted
        + registry.values_added
        + registry.values_deleted
        + registry.values_modified;

    DiffSummary { registry, total }
}


/// Keys of both maps, sorted and without duplicates.
pub(super) fn union<'a, V>(
    a: &'a BTreeMap<String, V>,
    b: &'a BTreeMap<String, V>,
) -> BTreeSet<&'a String> {
    a.keys().chain(b.keys()).collect()
}
