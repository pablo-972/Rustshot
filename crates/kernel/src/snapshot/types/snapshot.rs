use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::registry::RegistryKey;
use super::files::FileEntry;


/// Version of the JSON format of snapshots and diffs.
pub const FORMAT_VERSION: u32 = 1;


#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize
)]
pub struct Snapshot {
    pub meta: SnapshotMeta,
    pub summary: SnapshotSummary,
    pub data: SnapshotData,
}


#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Serialize,
    Deserialize
)]
pub struct SnapshotMeta {
    pub format: u32,
    /// Capture time, in seconds since the Unix epoch.
    pub created_at: u64,
    pub system: System,
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
pub struct SnapshotData {
    /// Registry: key path -> (key -> (value name -> value))
    #[serde(default)]
    pub registry: BTreeMap<String, RegistryKey>,

    /// Files by full path.
    #[serde(default)]
    pub files: BTreeMap<String, FileEntry>,
}


#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize
)]
#[serde(rename_all = "lowercase")]
pub enum System {
    Windows,
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
pub struct SnapshotSummary {
    pub registry_keys: usize,
    pub registry_values: usize,
    pub files: usize,
}