use super::filter::Filter;
use super::state::{DiffResult, Shot, Step};


// Everything that can happen: a button pressed, or a background task done.
#[derive(Debug, Clone)]
pub(crate) enum Message {
    // Capture window
    Browse,
    Shoot(Step),
    Compare,
    ViewResults,
    OpenDiff,
    Clear,

    // Results window
    Filter(Filter),
    // Search text changed or window resized: draw the list again.
    Refresh,

    // Background tasks. Errors come as text: the message must be Clone.
    ShotDone(Step, Result<Shot, String>),
    CompareDone(Result<DiffResult, String>),
    DiffOpened(Result<DiffResult, String>),
}
