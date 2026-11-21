//! Types of Rustshot.
//!
//! Defines every kind of type used

mod diff;
mod registry;
mod snapshot;


pub use diff::{
    DiffChanges,
    DiffMeta,
    DiffSummary,
    RegistryDiff,
    RegistryDiffSummary,
    SnapshotDiff,
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
