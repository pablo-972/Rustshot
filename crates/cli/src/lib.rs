//! Command line interface.
//!
//! Parses the arguments and runs the requested operation: shot or diff.

mod arguments;
mod commands;


pub use commands::run;
