//! Serde helpers that store bytes as a lowercase hex string.
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};


/// Bytes as a lowercase hex string: [0x0a, 0xff] -> "0aff".
pub fn encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn serialize<S: Serializer>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&encode(bytes))
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
    let hex = String::deserialize(deserializer)?;

    if !hex.is_ascii() || !hex.len().is_multiple_of(2) {
        return Err(D::Error::custom("invalid hex string"));
    }

    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16)
        .map_err(D::Error::custom))
        .collect()
}