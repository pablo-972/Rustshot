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
}


/// Operating system where the snapshot was taken.
/// Fields are None when Windows does not provide them.
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
    /// For example "Windows 7 Professional".
    pub name: Option<String>,
    /// Kernel version, for example "6.1" or "10.0".
    pub version: Option<String>,
    /// Feature update, for example "22H2". Windows 10 and later only.
    pub release: Option<String>,
    /// Build and revision, for example "7601.24544".
    pub build: Option<String>,
    /// For example "Service Pack 1". Up to Windows 8.1 only.
    pub service_pack: Option<String>,
    /// For example "Professional".
    pub edition: Option<String>,
    /// Architecture of the system, for example "AMD64" or "x86".
    pub architecture: Option<String>,
}


/// Machine and user that took the snapshot.
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
    /// Domain of the user, or the machine name if it is a local account.
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
}