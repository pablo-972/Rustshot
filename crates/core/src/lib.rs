//! Core.
//!
//! Captures the registry, compares snapshots and saves them as JSON.

mod diff;
mod error;
mod hex;
mod registry;
mod shot;
mod storage;
mod system;
mod types;


pub use diff::diff;
pub use error::Error;
pub use shot::shot;
pub use storage::{load, save};
pub use types::{
    DeviceInfo,
    DiffChanges,
    DiffMeta,
    DiffSummary,
    NotCompared,
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
    UnreadableIn,
    ValueChange,
    ValueEntry,
};
