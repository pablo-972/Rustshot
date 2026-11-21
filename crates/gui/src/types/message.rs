use super::files::{DiffFile, ShotFile};
use super::filter::Filter;
use super::step::Step;
use super::theme_mode::ThemeMode;


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

    // Background tasks. Errors come as text: the message must be Clone.
    ShotDone(Step, Result<ShotFile, String>),
    ShotOpened(Step, Result<ShotFile, String>),
    CompareDone(Result<DiffFile, String>),
    DiffOpened(Result<DiffFile, String>),
}
