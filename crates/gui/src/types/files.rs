use std::path::PathBuf;
use std::sync::Arc;

use kernel::{Snapshot, SnapshotDiff};


// A snapshot, taken or opened, and the file it is saved in.
#[derive(Debug, Clone)]
pub(crate) struct ShotFile {
    pub snapshot: Arc<Snapshot>,
    pub path: PathBuf,
}


// A diff, from a comparison or opened, and the file it is saved in.
#[derive(Debug, Clone)]
pub(crate) struct DiffFile {
    pub diff: Arc<SnapshotDiff>,
    pub path: PathBuf,
}
