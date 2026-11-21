use std::path::{Path, PathBuf};

use fltk::app::Sender;
use fltk::dialog::{NativeFileChooser, NativeFileChooserType};

use super::tasks;
use crate::format::file_name;
use crate::types::{
    Message,
    Progress,
    State,
    Status,
    Step,
    Task,
};
use crate::view::{CaptureWindow, Windows};


pub(super) fn browse(capture: &mut CaptureWindow) {
    let chosen = choose_folder(&capture.output());

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
        state.status = Status::error(error);
        return;
    }

    let path = output_path(capture, &name);
    discard_after(state, windows, step);

    state.busy = Some(Task::Shoot(step));
    state.progress = Some(Progress::start());
    state.status = Status::info("Working…");

    tasks::shoot(step, path, sender);
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
        state.status = Status::error(error);
        return;
    }

    windows.results.hide();
    let before = first.snapshot.clone();
    let after = second.snapshot.clone();

    state.busy = Some(Task::Compare);
    state.status = Status::info("Comparing…");

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
    let Some(path) = choose_file(&windows.capture.output()) else {
        return;
    };

    windows.capture.set_shot_name(step, &file_name(&path));

    discard_after(state, windows, step);

    state.busy = Some(Task::OpenShot(step));
    state.status = Status::info("Opening…");

    tasks::open_shot(step, path, sender);
}

pub(super) fn open_diff(
    state: &mut State,
    windows: &mut Windows,
    sender: Sender<Message>
) {
    let Some(path) = choose_file(&windows.capture.output()) else {
        return;
    };

    windows.capture.set_diff_name(&file_name(&path));

    state.busy = Some(Task::OpenDiff);
    state.status = Status::info("Opening…");

    tasks::open_diff(path, sender);
}


// A new shot, taken or opened, makes the later steps outdated.
fn discard_after(state: &mut State, windows: &mut Windows, step: Step) {
    if step == Step::First {
        state.second = None;
    }

    state.diff = None;
    windows.results.hide();
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

// System dialogs to pick a folder or a JSON file. None if they are cancelled.
fn choose_folder(start: &str) -> Option<PathBuf> {
    choose(NativeFileChooserType::BrowseDir, start)
}

fn choose_file(start: &str) -> Option<PathBuf> {
    choose(NativeFileChooserType::BrowseFile, start)
}

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
