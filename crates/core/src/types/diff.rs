use serde::{Deserialize, Serialize};

use super::registry::RegistryValue;


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
    #[serde(default)]
    pub not_compared: usize,
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
    /// Keys that could not be read in one of the snapshots, so they and their
    /// subkeys were not compared.
    #[serde(default)]
    pub not_compared: Vec<String>,
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
