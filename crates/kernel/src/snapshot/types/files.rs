use serde::{Deserialize, Serialize};


#[derive(
    Debug, 
    Clone, 
    PartialEq, 
    Eq, 
    Serialize, 
    Deserialize
)]
pub struct FileEntry {
    pub kind: FileKind,
    /// Last modification time, in seconds since the Unix epoch.
    pub modified: u64,
}


#[derive(
    Debug, 
    Clone, 
    PartialEq, 
    Eq, 
    Serialize, 
    Deserialize
)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum FileKind {
    /// `sha256` is None when it was not computed: disabled for the target,
    /// file too big or not readable.
    File { size: u64, sha256: Option<String> },
    Symlink { target: String },
    Directory,
}