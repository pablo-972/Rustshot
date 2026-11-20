//! What to capture
//! 
//! The targets, registries and files, embedded in the binary.

// Capture only runs on Windows, so the targets only exist there.
#[cfg(windows)]
mod files;
#[cfg(windows)]
mod registry;
#[cfg(windows)]
mod types;

#[cfg(windows)]
pub(crate) use files::file_targets;
#[cfg(windows)]
pub(crate) use registry::registry_targets;