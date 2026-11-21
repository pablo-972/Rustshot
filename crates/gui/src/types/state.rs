use super::files::{DiffFile, ShotFile};
use super::status::Status;
use super::task::Task;


// Everything the capture window shows. The widgets keep no state of their
// own, except the text the user types. The results window keeps its filter
// and its rows: nothing else depends on them.
#[derive(Debug, Clone, Default)]
pub(crate) struct State {
    pub first: Option<ShotFile>,
    pub second: Option<ShotFile>,
    pub diff: Option<DiffFile>,
    pub busy: Option<Task>,
    pub status: Status,
}


impl State {
    // State at start, and after Clear.
    pub(crate) fn ready() -> Self {
        State {
            status: Status::info("Ready"),
            ..State::default()
        }
    }
}
