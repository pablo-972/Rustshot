//! Types of Rustshot.
//!
//! Defines every kind of type used

mod snapshot;
mod registry;
mod files;
mod diff;

pub use snapshot::{
    FORMAT_VERSION,
    Snapshot,
    SnapshotMeta,
    SnapshotData,
    SnapshotSummary,
    System,
};
pub use registry::{RegistryKey, RegistryValue};
pub use files::{FileEntry, FileKind};
pub use diff::{
    SnapshotDiff,
    DiffMeta,
    DiffChanges,
    RegistryDiff, 
    FilesDiff,
    DiffSummary,
    RegistryDiffSummary,
    FilesDiffSummary, 
    ValueEntry,
    FileItem, 
    ValueChange, 
    FileChange, 
};




