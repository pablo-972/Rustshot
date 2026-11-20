use std::io;
use std::path::PathBuf;

use thiserror::Error;


#[derive(Debug, Error)]
pub enum SnapshotError {
    #[error("invalid registry configuration")]
    Config(#[from] toml::de::Error),

    #[error("cannot read snapshot {}", path.display())]
    Read {path: PathBuf, #[source] source: io::Error},

    #[error("invalid snapshot {}", path.display())]
    Parse {path: PathBuf, #[source] source: serde_json::Error},

    #[error("cannot serialize to JSON")]
    Serialize(#[source] serde_json::Error),

    #[error("cannot write {}", path.display())]
    Write {path: PathBuf, #[source] source: io::Error},
}


pub type Result<T> = std::result::Result<T, SnapshotError>;
