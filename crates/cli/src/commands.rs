use std::path::Path;

use anyhow::Result;
use clap::Parser;

use crate::arguments::Cli;
use crate::arguments::Command;


pub fn run() -> Result<()> {
    banner();
    execute(Cli::parse())
}


fn execute(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Shot {output} => run_shot(&output),
        Command::Compare {before, after, output} => {
            run_compare(&before, &after, &output)
        } 
    }
}

fn run_shot(output: &Path) -> Result<()> {
    let snap = kernel::shot()?;
    kernel::save(&snap, output)?;
    
    let summary = &snap.summary;

    eprintln!(
        "Snapshot saved to {} ({} registry keys, {} values)",
        output.display(),
        summary.registry_keys,
        summary.registry_values
    );

    Ok(())
}

fn run_compare(
    before: &Path,
    after: &Path,
    output: &Path
) -> Result<()> {
    let before = kernel::load(before)?;
    let after = kernel::load(after)?;

    let diff = kernel::compare(&before, &after);
    kernel::save(&diff, output)?;

    eprintln!("Keys added:      {}", diff.summary.registry.keys_added);
    eprintln!("Keys deleted:    {}", diff.summary.registry.keys_deleted);
    eprintln!("Values added:    {}", diff.summary.registry.values_added);
    eprintln!("Values deleted:  {}", diff.summary.registry.values_deleted);
    eprintln!("Values modified: {}", diff.summary.registry.values_modified);
    eprintln!("Diff saved to    {}", output.display());

    Ok(())
}

fn banner() {
    println!(
        r#"
         ____            _       _           _
        |  _ \ _   _ ___| |_ ___| |__   ___ | |_
        | |_) | | | / __| __/ __| '_ \ / _ \| __|
        |  _ <| |_| \__ \ |_\__ \ | | | (_) | |_
        |_| \_\\__,_|___/\__|___/_| |_|\___/ \__|
    "#
    );
}