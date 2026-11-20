//! Entry point.
//!
//! With command line arguments, runs the CLI. Without them, opens the GUI.

fn main() -> anyhow::Result<()> {
    if std::env::args_os().len() > 1 {
        cli::run()?
    }

    Ok(())
}
