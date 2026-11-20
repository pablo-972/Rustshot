//! Entry point.
//!
//! With a double click, opens the GUI. From a terminal, runs the CLI:
//! `rustshot gui` opens the GUI from there.

use std::env;


fn main() -> anyhow::Result<()> {
    let mut args = env::args_os().skip(1);

    match args.next() {
        Some(arg) if arg == "gui" => gui(),
        Some(_) => cli::run(),
        None => no_arguments(),
    }
}


// Double click: the GUI. From a terminal: the help of the CLI.
#[cfg(windows)]
fn no_arguments() -> anyhow::Result<()> {
    if gui::launched_from_terminal() { cli::run() } else { gui::run() }
}


#[cfg(windows)]
fn gui() -> anyhow::Result<()> {
    gui::run()
}


// The GUI only exists on Windows: elsewhere there is only the CLI.
#[cfg(not(windows))]
fn no_arguments() -> anyhow::Result<()> {
    cli::run()
}


#[cfg(not(windows))]
fn gui() -> anyhow::Result<()> {
    anyhow::bail!("the graphical interface only exists on Windows")
}
