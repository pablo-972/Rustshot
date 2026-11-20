//! Graphical interface.
//!
//! Only on Windows: on other systems this crate is empty.

#![cfg(windows)]

mod app;
mod console;
mod tasks;
mod theme;
mod types;
mod view;

pub use app::run;
pub use console::launched_from_terminal;
