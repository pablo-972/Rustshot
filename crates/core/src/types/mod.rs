//! Types of Rustshot.
//!
//! The snapshot, the diff and the registry values, as saved in the JSON.

mod diff;
mod registry;
mod snapshot;


pub use diff::{
    DiffChanges,
    DiffMeta,
    DiffSummary,
    NotCompared,
    RegistryDiff,
    RegistryDiffSummary,
    SnapshotDiff,
    UnreadableIn,
    ValueChange,
    ValueEntry,
};
pub use registry::{RegistryKey, RegistryValue};
pub use snapshot::{
    DeviceInfo,
    Snapshot,
    SnapshotData,
    SnapshotMeta,
    SnapshotSummary,
    SystemInfo,
};
