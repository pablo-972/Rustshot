use crate::registry;
use crate::types::{
    DiffChanges,
    DiffMeta,
    DiffSummary,
    RegistryDiff,
    RegistryDiffSummary,
    Snapshot,
    SnapshotDiff,
};


pub fn diff(before: &Snapshot, after: &Snapshot) -> SnapshotDiff {
    let registry = registry::compare(&before.data, &after.data);
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
        not_compared: registry.not_compared.len(),
    };

    let total = registry.keys_added
        + registry.keys_deleted
        + registry.values_added
        + registry.values_deleted
        + registry.values_modified;

    DiffSummary { registry, total }
}
