use std::path::PathBuf;

use clap::{Parser, Subcommand};


/// Captures the Windows registry and compares two captures.
#[derive(Debug, Parser)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Take a snapshot of the system state
    Shot {
        #[arg(
            short,
            long,
            value_name = "FILE",
            default_value = "snapshot.json"
        )]
        output: PathBuf,
    },

    /// Compare two snapshot files
    Compare {
        before: PathBuf,
        after: PathBuf,

        #[arg(
            short,
            long,
            value_name = "FILE",
            default_value = "diff.json"
        )]
        output: PathBuf
    }
}
