use std::env;

use fltk::app::{self, App, Scheme};

use super::update::update;
use crate::theme;
use crate::types::State;
use crate::view::Windows;


pub fn run() -> anyhow::Result<()> {
    let app = App::default().with_scheme(Scheme::Base);
    theme::apply();

    let (sender, receiver) = app::channel();
    let mut windows = Windows::new(sender, &default_output());
    let mut state = State::ready();

    windows.capture.render(&state);

    // Every button and every background task sends a message. Each one
    // changes the state, and the capture window is drawn again from it.
    while app.wait() {
        if let Some(message) = receiver.recv() {
            update(&mut state, &mut windows, message, sender);
            windows.capture.render(&state);
        }
    }

    Ok(())
}


// The folder Rustshot was opened from: the folder of the .exe when it is
// opened with a double click.
fn default_output() -> String {
    env::current_dir()
        .map(|dir| dir.to_string_lossy().into_owned())
        .unwrap_or_default()
}
