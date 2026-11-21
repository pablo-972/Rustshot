use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use fltk::app::Sender;
use rustshot_core::Snapshot;

use crate::types::{
    DiffFile,
    Message,
    ShotFile,
    Step
};


pub(super) fn shoot(
    step: Step,
    path: PathBuf,
    sender: Sender<Message>
) {
    let done = Arc::new(AtomicBool::new(false));
    tick(done.clone(), sender);

    thread::spawn(move || {
        let result = take_shot(path, sender).map_err(|e| describe(&e));
        done.store(true, Ordering::Relaxed);
        sender.send(Message::ShotDone(step, result));
    });
}

pub(super) fn compare(
    before: Arc<Snapshot>,
    after: Arc<Snapshot>,
    path: PathBuf,
    sender: Sender<Message>
) {
    thread::spawn(move || {
        let diff = rustshot_core::diff(&before, &after);
        let result = rustshot_core::save(&diff, &path)
            .map(|()| DiffFile { diff: Arc::new(diff), path })
            .map_err(|e| describe(&e));

        sender.send(Message::CompareDone(result));
    });
}

pub(super) fn open_shot(
    step: Step,
    path: PathBuf,
    sender: Sender<Message>
) {
    thread::spawn(move || {
        let result = rustshot_core::load(&path)
            .map(|snap| ShotFile { snapshot: Arc::new(snap), path })
            .map_err(|e| describe(&e));

        sender.send(Message::ShotOpened(step, result));
    });
}

pub(super) fn open_diff(path: PathBuf, sender: Sender<Message>) {
    thread::spawn(move || {
        let result = rustshot_core::load(&path)
            .map(|diff| DiffFile { diff: Arc::new(diff), path })
            .map_err(|e| describe(&e));

        sender.send(Message::DiffOpened(result));
    });
}


fn take_shot(path: PathBuf, sender: Sender<Message>) -> Result<ShotFile, rustshot_core::Error> {
    let snap = rustshot_core::shot(|captured| sender.send(Message::ShotProgress(captured)))?;
    rustshot_core::save(&snap, &path)?;

    Ok(ShotFile { snapshot: Arc::new(snap), path })
}

// Sends a Tick every second until `done`: the capture only reports every
// thousand keys, and saving the file reports nothing.
fn tick(done: Arc<AtomicBool>, sender: Sender<Message>) {
    thread::spawn(move || {
        while !done.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_secs(1));
            sender.send(Message::Tick);
        }
    });
}

// Error message with its causes, e.g. "cannot write C:\x: Access is denied".
fn describe(error: &dyn Error) -> String {
    let mut text = error.to_string();
    let mut source = error.source();

    while let Some(cause) = source {
        text.push_str(": ");
        text.push_str(&cause.to_string());
        source = cause.source();
    }

    text
}
