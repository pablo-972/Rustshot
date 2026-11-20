use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

use fltk::app::Sender;
use kernel::{Snapshot, SnapshotError};

use crate::types::{DiffResult, Message, Shot, Step};


pub(crate) fn shoot(
    step: Step,
    path: PathBuf,
    sender: Sender<Message>
) {
    thread::spawn(move || {
        let result = take_shot(path).map_err(|e| describe(&e));
        sender.send(Message::ShotDone(step, result));
    });
}


pub(crate) fn compare(
    before: Arc<Snapshot>,
    after: Arc<Snapshot>,
    path: PathBuf,
    sender: Sender<Message>
) {
    thread::spawn(move || {
        let diff = kernel::compare(&before, &after);
        let result = kernel::save(&diff, &path)
            .map(|()| DiffResult { diff: Arc::new(diff), path })
            .map_err(|e| describe(&e));

        sender.send(Message::CompareDone(result));
    });
}


pub(crate) fn open_shot(
    step: Step,
    path: PathBuf,
    sender: Sender<Message>
) {
    thread::spawn(move || {
        let result = kernel::load(&path)
            .map(|snap| Shot { snapshot: Arc::new(snap), path })
            .map_err(|e| describe(&e));

        sender.send(Message::ShotOpened(step, result));
    });
}


pub(crate) fn open_diff(path: PathBuf, sender: Sender<Message>) {
    thread::spawn(move || {
        let result = kernel::load(&path)
            .map(|diff| DiffResult { diff: Arc::new(diff), path })
            .map_err(|e| describe(&e));

        sender.send(Message::DiffOpened(result));
    });
}


fn take_shot(path: PathBuf) -> Result<Shot, SnapshotError> {
    let snap = kernel::shot()?;
    kernel::save(&snap, &path)?;

    Ok(Shot { snapshot: Arc::new(snap), path })
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
