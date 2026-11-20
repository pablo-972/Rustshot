use fltk::app::Sender;

use super::actions::{
    self,
    failure,
    info,
};
use crate::types::{
    DiffResult,
    Message,
    Shot,
    State,
    Step,
};
use crate::view::{
    Windows, 
    file_name, 
    format_count
};


// State at start, and after Clear.
pub(super) fn ready() -> State {
    State {
        status: info("Ready"),
        ..State::default()
    }
}


pub(super) fn update(
    state: &mut State,
    windows: &mut Windows,
    message: Message,
    sender: Sender<Message>
) {
    match message {
        Message::Browse => actions::browse(&mut windows.capture),
        Message::Shoot(step) => actions::shoot(state, windows, step, sender),
        Message::Compare => actions::compare(state, windows, sender),
        Message::ViewResults => actions::view_results(state, windows),
        Message::OpenShot(step) => actions::open_shot(state, windows, step, sender),
        Message::OpenDiff => actions::open_diff(state, windows, sender),

        Message::Clear => {
            *state = ready();
            windows.results.hide();
        }

        Message::Filter(filter) => windows.results.set_filter(filter),
        Message::Refresh => windows.results.render(),

        Message::ShotDone(step, result) => shot_done(state, step, result, "Saved"),
        Message::ShotOpened(step, result) => shot_done(state, step, result, "Opened"),
        Message::CompareDone(result) => compare_done(state, result),
        Message::DiffOpened(result) => diff_opened(state, windows, result),
    }
}


// A shot taken or opened: `action` says which, for the status bar.
fn shot_done(
    state: &mut State,
    step: Step,
    result: Result<Shot, String>,
    action: &str
) {
    state.busy = None;

    match result {
        Ok(shot) => {
            state.status = info(&format!("{action} {}", file_name(&shot.path)));

            match step {
                Step::First => state.first = Some(shot),
                Step::Second => state.second = Some(shot),
            }
        }
        Err(error) => state.status = failure(&error),
    }
}


fn compare_done(state: &mut State, result: Result<DiffResult, String>) {
    state.busy = None;

    match result {
        Ok(diff) => {
            let total = format_count(diff.diff.summary.total);
            state.status = info(&format!("{total} changes found"));
            state.diff = Some(diff);
        }
        Err(error) => state.status = failure(&error),
    }
}


fn diff_opened(
    state: &mut State,
    windows: &mut Windows,
    result: Result<DiffResult, String>
) {
    state.busy = None;

    match result {
        Ok(diff) => {
            state.status = info(&format!("Opened {}", file_name(&diff.path)));
            windows.results.load(&diff);
            windows.results.show();
            state.diff = Some(diff);
        }
        Err(error) => state.status = failure(&error),
    }
}
