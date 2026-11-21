use std::path::Path;
use std::time::Instant;

use anyhow::Result;
use clap::Parser;
use rustshot_core::UnreadableIn;

use crate::arguments::{Cli, Command};


pub fn run() -> Result<()> {
    banner();
    execute(Cli::parse())
}


fn execute(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Shot { output } => run_shot(&output),
        Command::Diff { before, after, output } => run_diff(&before, &after, &output),
    }
}

fn run_shot(output: &Path) -> Result<()> {
    let started = Instant::now();

    let snap = rustshot_core::shot(|captured| {
        eprint!(
            "\rCapturing... {} keys, {} values, {} inaccessible, {} s",
            captured.registry_keys,
            captured.registry_values,
            captured.registry_inaccessible,
            started.elapsed().as_secs()
        );
    })?;
    eprintln!();

    rustshot_core::save(&snap, output)?;

    let summary = &snap.summary;

    eprintln!(
        "Snapshot saved to {} ({} registry keys, {} values, {} inaccessible)",
        output.display(),
        summary.registry_keys,
        summary.registry_values,
        summary.registry_inaccessible
    );

    Ok(())
}

fn run_diff(
    before: &Path,
    after: &Path,
    output: &Path
) -> Result<()> {
    let before = rustshot_core::load(before)?;
    let after = rustshot_core::load(after)?;

    let diff = rustshot_core::diff(&before, &after);
    rustshot_core::save(&diff, output)?;

    let summary = &diff.summary.registry;

    // Unreadable only in the 2nd shot is worth a look: the key may have been
    // locked between the two shots.
    let locked = diff.changes.registry.not_compared
        .iter()
        .filter(|entry| entry.unreadable_in == UnreadableIn::After)
        .count();

    let not_compared = if locked > 0 {
        format!("{} ({locked} unreadable only in the 2nd shot)", summary.not_compared)
    } else {
        summary.not_compared.to_string()
    };

    eprintln!("Keys added:      {}", summary.keys_added);
    eprintln!("Keys deleted:    {}", summary.keys_deleted);
    eprintln!("Values added:    {}", summary.values_added);
    eprintln!("Values deleted:  {}", summary.values_deleted);
    eprintln!("Values modified: {}", summary.values_modified);
    eprintln!("Not compared:    {not_compared}");
    eprintln!("Diff saved to    {}", output.display());

    Ok(())
}

// On stderr, like the rest of the messages: only the JSON files are output.
fn banner() {
    eprintln!(
        r#"
         ____            _       _           _
        |  _ \ _   _ ___| |_ ___| |__   ___ | |_
        | |_) | | | / __| __/ __| '_ \ / _ \| __|
        |  _ <| |_| \__ \ |_\__ \ | | | (_) | |_
        |_| \_\\__,_|___/\__|___/_| |_|\___/ \__|
    "#
    );
}
