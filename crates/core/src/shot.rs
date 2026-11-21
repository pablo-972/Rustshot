use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::Result;
use crate::{registry, system};
use crate::types::{
    Snapshot,
    SnapshotData,
    SnapshotMeta,
    SnapshotSummary,
};


/// Captures the system. `report` is called as the capture goes, with the
/// keys and values captured so far, and the keys that could not be opened.
pub fn shot(mut report: impl FnMut(SnapshotSummary)) -> Result<Snapshot> {
    let registry = registry::capture(&mut report);

    Ok(Snapshot {
        meta: SnapshotMeta {
            created_at: unix_now(),
            system: system::system(),
            device: system::device(),
        },
        summary: SnapshotSummary {
            registry_keys: registry.keys.len(),
            registry_values: registry.keys.values().map(|key| key.values.len()).sum(),
            registry_inaccessible: registry.inaccessible.len(),
        },
        data: SnapshotData {
            registry: registry.keys,
            inaccessible: registry.inaccessible,
        },
    })
}


fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
