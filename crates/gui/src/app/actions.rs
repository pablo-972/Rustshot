use std::path::{Path, PathBuf};

use fltk::app::Sender;
use fltk::dialog::{NativeFileChooser, NativeFileChooserType};

use crate::tasks;
use crate::types::{
    Message,
    State,
    Status,
    Step,
    Task,
};
use crate::view::{CaptureWindow, Windows, file_name};


pub(super) fn browse(capture: &mut CaptureWindow) {
    let chosen = choose(NativeFileChooserType::BrowseDir, &capture.output());

    if let Some(dir) = chosen {
        capture.set_output(&dir.to_string_lossy());
    }
}


pub(super) fn shoot(
    state: &mut State,
    windows: &mut Windows,
    step: Step,
    sender: Sender<Message>
) {
    let capture = &windows.capture;
    let name = capture.shot_name(step);

    if let Err(error) = check(capture, &name) {
        state.status = failure(error);
        return;
    }

    // A new shot makes the later steps outdated.
    if step == Step::First {
        state.second = None;
    }
    state.diff = None;
    windows.results.hide();

    state.busy = Some(Task::Shoot(step));
    state.status = info(match step {
        Step::First => "Taking 1st shot…",
        Step::Second => "Taking 2nd shot…",
    });

    tasks::shoot(step, output_path(capture, &name), sender);
}


pub(super) fn compare(
    state: &mut State,
    windows: &mut Windows,
    sender: Sender<Message>
) {
    let (Some(first), Some(second)) = (&state.first, &state.second) else {
        return;
    };

    let capture = &windows.capture;
    let name = capture.diff_name();

    if let Err(error) = check(capture, &name) {
        state.status = failure(error);
        return;
    }

    windows.results.hide();
    let before = first.snapshot.clone();
    let after = second.snapshot.clone();

    state.busy = Some(Task::Compare);
    state.status = info("Comparing…");

    tasks::compare(before, after, output_path(capture, &name), sender);
}


pub(super) fn view_results(state: &State, windows: &mut Windows) {
    if let Some(diff) = &state.diff {
        windows.results.load(diff);
        windows.results.show();
    }
}


// Uses a snapshot saved before instead of taking it again.
pub(super) fn open_shot(
    state: &mut State,
    windows: &mut Windows,
    step: Step,
    sender: Sender<Message>
) {
    let Some(path) = choose(NativeFileChooserType::BrowseFile, &windows.capture.output()) else {
        return;
    };

    windows.capture.set_shot_name(step, &file_name(&path));

    // Like a new shot, it makes the later steps outdated.
    if step == Step::First {
        state.second = None;
    }
    state.diff = None;
    windows.results.hide();

    state.busy = Some(Task::OpenShot(step));
    state.status = info("Opening…");

    tasks::open_shot(step, path, sender);
}


pub(super) fn open_diff(
    state: &mut State,
    windows: &mut Windows,
    sender: Sender<Message>
) {
    let Some(path) = choose(NativeFileChooserType::BrowseFile, &windows.capture.output()) else {
        return;
    };

    windows.capture.set_diff_name(&file_name(&path));

    state.busy = Some(Task::OpenDiff);
    state.status = info("Opening…");

    tasks::open_diff(path, sender);
}


pub(super) fn info(text: &str) -> Status {
    Status {
        text: text.to_string(),
        error: false,
    }
}


pub(super) fn failure(text: &str) -> Status {
    Status {
        text: text.to_string(),
        error: true,
    }
}


// Checks the output folder and the file name of a step before using them.
fn check(capture: &CaptureWindow, name: &str) -> Result<(), &'static str> {
    if capture.output().is_empty() {
        return Err("Choose an output folder");
    }

    if name.is_empty() {
        return Err("Enter a file name");
    }

    // A repeated name would overwrite the file of another step.
    let repeated = capture.names().iter().filter(|n| *n == name).count() > 1;
    if repeated {
        return Err("Use a different file name for each step");
    }

    Ok(())
}


fn output_path(capture: &CaptureWindow, name: &str) -> PathBuf {
    Path::new(&capture.output()).join(name)
}


// System dialog to pick a folder or a file. None if it is cancelled.
fn choose(kind: NativeFileChooserType, start: &str) -> Option<PathBuf> {
    let mut chooser = NativeFileChooser::new(kind);

    if kind == NativeFileChooserType::BrowseFile {
        chooser.set_filter("JSON\t*.json");
    }

    let start = Path::new(start);
    if start.is_dir() {
        chooser.set_directory(&start).ok();
    }

    chooser.show();

    let chosen = chooser.filename();
    (!chosen.as_os_str().is_empty()).then_some(chosen)
}
