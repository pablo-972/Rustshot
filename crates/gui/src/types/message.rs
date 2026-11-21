use kernel::SnapshotSummary;

use super::state::{DiffFile, ShotFile, Step, ThemeMode};


// Everything that can happen: a button pressed, or a background task done.
#[derive(Debug, Clone)]
pub(crate) enum Message {
    // Capture window
    Browse,
    Shoot(Step),
    Compare,
    ViewResults,
    OpenShot(Step),
    OpenDiff,
    Clear,

    // Menu of the capture window
    Theme(ThemeMode),
    About,

    // Results window
    Filter(Filter),

    // Search text changed or window resized: draw the list again.
    Refresh,

    // While a shot is taken: what is captured so far, and every second, to
    // keep the time up to date.
    ShotProgress(SnapshotSummary),
    Tick,

    // Background tasks done. Errors come as text: the message must be Clone.
    ShotDone(Step, Result<ShotFile, String>),
    ShotOpened(Step, Result<ShotFile, String>),
    CompareDone(Result<DiffFile, String>),
    DiffOpened(Result<DiffFile, String>),
}

// Which changes the results window shows.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq
)]
pub(crate) enum Filter {
    #[default]
    All,
    KeysAdded,
    KeysDeleted,
    ValuesAdded,
    ValuesDeleted,
    Modified,
}
