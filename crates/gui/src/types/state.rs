use super::files::{DiffFile, ShotFile};
use super::status::Status;
use super::task::Task;
use super::theme_mode::ThemeMode;


// Everything the capture window shows. 
#[derive(Debug, Clone, Default)]
pub(crate) struct State {
    pub first: Option<ShotFile>,
    pub second: Option<ShotFile>,
    pub diff: Option<DiffFile>,
    pub busy: Option<Task>,
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
