use std::error::Error;
use std::panic::{self, AssertUnwindSafe};
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
    Step,
};


pub(super) fn shoot(
    step: Step,
    path: PathBuf,
    sender: Sender<Message>
) {
    let done = Arc::new(AtomicBool::new(false));
    tick(done.clone(), sender);

    thread::spawn(move || {
        let result = guarded(|| take_shot(path, sender).map_err(|e| describe(&e)));
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
        let result = guarded(|| {
            let diff = rustshot_core::diff(&before, &after);

            rustshot_core::save(&diff, &path)
                .map(|()| DiffFile { diff: Arc::new(diff), path })
                .map_err(|e| describe(&e))
        });

        sender.send(Message::CompareDone(result));
    });
}

pub(super) fn open_shot(
    step: Step,
    path: PathBuf,
    sender: Sender<Message>
) {
    thread::spawn(move || {
        let result = guarded(|| {
            rustshot_core::load(&path)
                .map(|snap| ShotFile { snapshot: Arc::new(snap), path })
                .map_err(|e| describe(&e))
        });

        sender.send(Message::ShotOpened(step, result));
    });
}

pub(super) fn open_diff(path: PathBuf, sender: Sender<Message>) {
    thread::spawn(move || {
        let result = guarded(|| {
            rustshot_core::load(&path)
                .map(|diff| DiffFile { diff: Arc::new(diff), path })
                .map_err(|e| describe(&e))
        });

        sender.send(Message::DiffOpened(result));
    });
}


fn take_shot(path: PathBuf, sender: Sender<Message>) -> Result<ShotFile, rustshot_core::Error> {
    let snap = rustshot_core::shot(|captured| sender.send(Message::ShotProgress(captured)))?;
    rustshot_core::save(&snap, &path)?;

    Ok(ShotFile { snapshot: Arc::new(snap), path })
}

// Runs a task and turns a panic into an error. Otherwise its message would
// never arrive: the window would stay busy and the tick would never stop.
fn guarded<T>(task: impl FnOnce() -> Result<T, String>) -> Result<T, String> {
    panic::catch_unwind(AssertUnwindSafe(task))
        .unwrap_or_else(|_| Err("Unexpected error: the task stopped".to_string()))
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


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn a_result_passes_through() {
        assert_eq!(guarded(|| Ok::<_, String>(5)), Ok(5));
        assert_eq!(guarded(|| Err::<(), _>("cannot read".to_string())), Err("cannot read".into()));
    }

    #[test]
    fn a_panic_becomes_an_error() {
        let result: Result<(), String> = guarded(|| panic!("boom"));
        assert!(result.is_err());
    }
}
