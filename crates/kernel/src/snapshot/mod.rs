//! Snapshot module 
//! 
//! Captures, comparison and storage.

mod capture;
mod compare;
mod error;
mod storage;
mod types;

pub use capture::shot;
pub use compare::compare;
pub use error::SnapshotError;
pub use storage::{load, save};
pub use types::{
    DiffChanges,
    DiffMeta,
    DiffSummary,
    FileChange,
    FileEntry,
    FileItem,
    FileKind,
    FilesDiff,
    FilesDiffSummary,
    FORMAT_VERSION,
    RegistryDiff,
    RegistryDiffSummary,
    RegistryKey,
    RegistryValue,
    Snapshot,
    SnapshotData,
    SnapshotDiff,
    SnapshotMeta,
    SnapshotSummary,
    System,
    ValueChange,
    ValueEntry,
};