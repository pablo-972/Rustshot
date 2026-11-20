use std::path::Path;
use anyhow::Result;

use clap::Parser;

use kernel::snapshot;

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
    let snap = snapshot::shot()?;
    snapshot::save(&snap, output)?;
    
    let summary = &snap.summary;

    eprintln!(
        "Snapshot saved to {} ({} registry keys, {} values, {} files)",
        output.display(),
        summary.registry_keys,
        summary.registry_values,
        summary.files
    );

    Ok(())
}


fn run_compare(
    before: &Path,
    after: &Path,
    output: &Path
) -> Result<()> {
    let before = snapshot::load(before)?;
    let after = snapshot::load(after)?;

    let diff = snapshot::compare(&before, &after);
    snapshot::save(&diff, output)?;

    eprintln!("Keys added:      {}", diff.summary.registry.keys_added);
    eprintln!("Keys deleted:    {}", diff.summary.registry.keys_deleted);
    eprintln!("Values added:    {}", diff.summary.registry.values_added);
    eprintln!("Values deleted:  {}", diff.summary.registry.values_deleted);
    eprintln!("Values modified: {}", diff.summary.registry.values_modified);
    eprintln!("Files added:     {}", diff.summary.files.added);
    eprintln!("Files deleted:   {}", diff.summary.files.deleted);
    eprintln!("Files modified:  {}", diff.summary.files.modified);
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