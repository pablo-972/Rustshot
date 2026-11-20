//! Types of Rustshot.
//!
//! Defines every kind of type used

mod snapshot;
mod registry;
mod diff;

pub use snapshot::{
    Snapshot,
    SnapshotMeta,
    SnapshotData,
    SnapshotSummary,
    SystemInfo,
    DeviceInfo,
};
pub use registry::{RegistryKey, RegistryValue};
pub use diff::{
    SnapshotDiff,
    DiffMeta,
    DiffChanges,
    RegistryDiff, 
    DiffSummary,
    RegistryDiffSummary,
    ValueEntry,
    ValueChange, 
};




