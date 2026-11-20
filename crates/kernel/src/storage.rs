use std::fs;
use std::path::Path;

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{from_slice, to_vec_pretty};

use crate::error::{Result, SnapshotError};


pub fn save<T: Serialize>(value: &T, path: &Path) -> Result<()> {
    // Create the folder of the file if it does not exist.
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|source| SnapshotError::Write {
            path: dir.to_path_buf(),
            source,
        })?;
    }

    let json = to_vec_pretty(value).map_err(SnapshotError::Serialize)?;
    fs::write(path, json).map_err(|source| SnapshotError::Write {
        path: path.to_path_buf(),
        source,
    })
}


pub fn load<T: DeserializeOwned>(path: &Path) -> Result<T> {
    let json = fs::read(path).map_err(|source| SnapshotError::Read {
        path: path.to_path_buf(),
        source,
    })?;

    from_slice(&json).map_err(|source| SnapshotError::Parse {
        path: path.to_path_buf(),
        source,
    })
}