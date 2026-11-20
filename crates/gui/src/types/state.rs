use std::path::PathBuf;
use std::sync::Arc;

use kernel::snapshot::{Snapshot, SnapshotDiff};


#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq
)]
pub(crate) enum Step {
    First,
    Second,
}


// Work running in a background thread.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq
)]
pub(crate) enum Task {
    Shoot(Step),
    Compare,
    OpenDiff,
}


#[derive(Debug, Clone)]
pub(crate) struct Shot {
    pub snapshot: Arc<Snapshot>,
    pub path: PathBuf,
}


// A diff from a comparison, or opened from a file.
#[derive(Debug, Clone)]
pub(crate) struct DiffResult {
    pub diff: Arc<SnapshotDiff>,
    pub path: PathBuf,
}


// Text of the bottom bar.
#[derive(Debug, Clone, Default)]
pub(crate) struct Status {
    pub text: String,
    pub error: bool,
}


// Everything the windows show. The widgets never keep state of their own,
// except the text the user types.
#[derive(Debug, Clone, Default)]
pub(crate) struct State {
    pub first: Option<Shot>,
    pub second: Option<Shot>,
    pub diff: Option<DiffResult>,
    pub busy: Option<Task>,
    pub status: Status,
}
