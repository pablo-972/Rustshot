//! Command line interface.
//!
//! Parses the arguments and runs the requested operation: shoot or compare.

mod arguments;
mod commands;

pub use commands::run;