use std::io;
use std::path::PathBuf;


// Errors of reading and writing the JSON files, snapshots and diffs alike.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cannot read {}", path.display())]
    Read {path: PathBuf, #[source] source: io::Error},

    #[error("invalid JSON in {}", path.display())]
    Parse {path: PathBuf, #[source] source: serde_json::Error},

    #[error("cannot serialize to JSON")]
    Serialize(#[source] serde_json::Error),

    #[error("cannot write {}", path.display())]
    Write {path: PathBuf, #[source] source: io::Error},
}


pub type Result<T> = std::result::Result<T, Error>;
