//! Compare module.
//! 
//! Comparison of two snapshots, one file per source.

mod diff;
mod files;
mod registry;

pub use diff::compare;