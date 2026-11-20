use std::time::{SystemTime, UNIX_EPOCH};

use super::{files, registry};
use crate::snapshot::error::Result;
use crate::snapshot::types::{
    FORMAT_VERSION,
    Snapshot,
    SnapshotData,
    SnapshotMeta,
    SnapshotSummary,
    System,
};


pub fn shot() -> Result<Snapshot> {
    let registry = registry::capture()?;
    let files = files::capture()?;

    let summary = SnapshotSummary {
            registry_keys: registry.len(),
            registry_values: registry.values()
                .map(|key| key.values
                .len())
                .sum(),
            files: files.len(),
        };

    Ok(Snapshot {
        meta: SnapshotMeta {
            format: FORMAT_VERSION,
            created_at: unix_now(),
            system: System::Windows,
        },
        summary,
        data: SnapshotData { registry, files },
    })
}


fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}