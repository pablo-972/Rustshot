use serde::{Deserialize, Serialize};

use super::registry::RegistryValue;
use super::files::FileEntry;


#[derive(
    Debug, 
    Clone, 
    Default, 
    PartialEq, 
    Eq, 
    Serialize, 
    Deserialize
)]
pub struct SnapshotDiff {
    pub meta: DiffMeta,
    pub summary: DiffSummary,
    pub changes: DiffChanges,
}


#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize
)]
pub struct DiffMeta {
    pub format: u32,
    /// Capture time of the first snapshot.
    pub before: u64,
    /// Capture time of the second snapshot.
    pub after: u64,
}


#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Serialize,
    Deserialize
)]
pub struct DiffChanges {
    pub registry: RegistryDiff,
    pub files: FilesDiff,
}


#[derive(
    Debug, 
    Clone, 
    Default, 
    PartialEq, 
    Eq, 
    Serialize, 
    Deserialize
)]
pub struct DiffSummary {
    pub registry: RegistryDiffSummary,
    pub files: FilesDiffSummary,
    pub total: usize,
}


#[derive(
    Debug, 
    Clone, 
    Default, 
    PartialEq, 
    Eq, 
    Serialize, 
    Deserialize
)]
pub struct RegistryDiffSummary {
    pub keys_added: usize,
    pub keys_deleted: usize,
    pub values_added: usize,
    pub values_deleted: usize,
    pub values_modified: usize,
}


#[derive(
    Debug, 
    Clone, 
    Default, 
    PartialEq, 
    Eq, 
    Serialize, 
    Deserialize
)]
pub struct FilesDiffSummary {
    pub added: usize,
    pub deleted: usize,
    pub modified: usize,
}


#[derive(
    Debug, 
    Clone, 
    Default, 
    PartialEq, 
    Eq, 
    Serialize, 
    Deserialize
)]
pub struct RegistryDiff {
    pub keys_added: Vec<String>,
    pub keys_deleted: Vec<String>,
    pub values_added: Vec<ValueEntry>,
    pub values_deleted: Vec<ValueEntry>,
    pub values_modified: Vec<ValueChange>,
}


#[derive(
    Debug, 
    Clone, 
    Default, 
    PartialEq, 
    Eq, 
    Serialize, 
    Deserialize
)]
pub struct FilesDiff {
    pub added: Vec<FileItem>,
    pub deleted: Vec<FileItem>,
    pub modified: Vec<FileChange>,
}


#[derive(
    Debug, 
    Clone, 
    PartialEq, 
    Eq, 
    Serialize, 
    Deserialize
)]
pub struct ValueEntry {
    pub key: String,
    pub name: String,
    pub value: RegistryValue,
}


#[derive(
    Debug, 
    Clone, 
    PartialEq, 
    Eq, 
    Serialize, 
    Deserialize
)]
pub struct ValueChange {
    pub key: String,
    pub name: String,
    pub before: RegistryValue,
    pub after: RegistryValue,
}


#[derive(
    Debug, 
    Clone, 
    PartialEq, 
    Eq, 
    Serialize, 
    Deserialize
)]
pub struct FileItem {
    pub path: String,
    pub entry: FileEntry,
}


#[derive(
    Debug, 
    Clone, 
    PartialEq, 
    Eq, 
    Serialize, 
    Deserialize
)]
pub struct FileChange {
    pub path: String,
    pub before: FileEntry,
    pub after: FileEntry,
}