//! Core.
//!
//! Captures the registry, compares snapshots and saves them as JSON.

mod capture;
mod compare;
mod error;
mod hex;
mod storage;
mod types;


pub use capture::shot;
pub use compare::compare;
pub use error::SnapshotError;
pub use storage::{load, save};
pub use types::{
    DeviceInfo,
    DiffChanges,
    DiffMeta,
    DiffSummary,
    RegistryDiff,
    RegistryDiffSummary,
    RegistryKey,
    RegistryValue,
    Snapshot,
    SnapshotData,
    SnapshotDiff,
    SnapshotMeta,
    SnapshotSummary,
    SystemInfo,
    ValueChange,
    ValueEntry,
};
