use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use kernel::{Snapshot, SnapshotDiff, SnapshotSummary};


// Everything the capture window shows.
#[derive(Debug, Clone, Default)]
pub(crate) struct State {
    pub first: Option<ShotFile>,
    pub second: Option<ShotFile>,
    pub diff: Option<DiffFile>,
    pub busy: Option<Task>,
    pub progress: Option<Progress>,
    pub status: Status,
    pub theme: ThemeMode,
}

impl State {
    // State at start. Clear keeps the theme.
    pub(crate) fn ready() -> Self {
        State {
            status: Status::info("Ready"),
            ..State::default()
        }
    }
}

// How a shot that is being taken goes: when it started, and the keys and
// values captured so far.
#[derive(Debug, Clone)]
pub(crate) struct Progress {
    pub started: Instant,
    pub captured: SnapshotSummary,
}

impl Progress {
    pub(crate) fn start() -> Self {
        Progress {
            started: Instant::now(),
            captured: SnapshotSummary::default(),
        }
    }
}

// Text of the bottom bar.
#[derive(Debug, Clone, Default)]
pub(crate) struct Status {
    pub text: String,
    pub error: bool,
}

impl Status {
    pub(crate) fn info(text: &str) -> Self {
        Status { text: text.to_string(), error: false }
    }

    pub(crate) fn error(text: &str) -> Self {
        Status { text: text.to_string(), error: true }
    }
}

// The two shots of the workflow.
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
    OpenShot(Step),
    Compare,
    OpenDiff,
}

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

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq
)]
pub(crate) enum ThemeMode {
    #[default]
    System,
    Light,
    Dark,
}
