use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};


#[derive(
    Debug, 
    Clone, 
    Default, 
    PartialEq, 
    Eq, 
    Serialize, 
    Deserialize
)]
pub struct RegistryKey {
    pub values: BTreeMap<String, RegistryValue>,
}


#[derive(
    Debug, 
    Clone, 
    PartialEq, 
    Eq, 
    Serialize, 
    Deserialize
)]
#[serde(tag = "type", content = "data")]
pub enum RegistryValue {
    Sz(String),
    ExpandSz(String),
    MultiSz(Vec<String>),
    Dword(u32),
    Qword(u64),
    Binary(#[serde(with = "crate::hex")] Vec<u8>),

    /// Any other type, or data that does not match its type.
    Other {
        kind: u32,
        #[serde(with = "crate::hex")]
        bytes: Vec<u8>,
    },
}