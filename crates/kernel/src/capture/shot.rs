use std::time::{SystemTime, UNIX_EPOCH};

use super::{registry, system};
use crate::error::Result;
use crate::types::{
    Snapshot,
    SnapshotData,
    SnapshotMeta,
    SnapshotSummary,
};


/// Captures the system. `report` is called as the capture goes, with the
/// keys and values captured so far.
pub fn shot(mut report: impl FnMut(SnapshotSummary)) -> Result<Snapshot> {
    let registry = registry::capture(&mut report)?;

    Ok(Snapshot {
        meta: SnapshotMeta {
            created_at: unix_now(),
            system: system::system(),
            device: system::device(),
        },
        summary: SnapshotSummary {
            registry_keys: registry.len(),
            registry_values: registry.values().map(|key| key.values.len()).sum(),
        },
        data: SnapshotData { registry },
    })
}


fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
