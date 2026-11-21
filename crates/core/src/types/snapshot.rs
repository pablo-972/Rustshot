use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::registry::RegistryKey;


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
    /// Capture time, in seconds since the Unix epoch.
    pub created_at: u64,
    pub system: SystemInfo,
    pub device: DeviceInfo,
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
    /// Keys that exist but could not be opened, such as HKLM\SAM\SAM. Their
    /// subkeys are not captured either.
    #[serde(default)]
    pub inaccessible: Vec<String>,
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
pub struct SystemInfo {
    pub name: Option<String>,
    pub version: Option<String>,
    pub release: Option<String>,
    pub build: Option<String>,
    pub service_pack: Option<String>,
    pub edition: Option<String>,
    pub architecture: Option<String>,
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
pub struct DeviceInfo {
    pub hostname: Option<String>,
    pub user: Option<String>,
    pub domain: Option<String>,
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
    #[serde(default)]
    pub registry_inaccessible: usize,
}
