//! Binary registry values as hex text in the JSON: [0x0a, 0xff] -> "0aff".
//!
//! Used with #[serde(with = "crate::hex")].

use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serializer};


pub fn serialize<S: Serializer>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&encode(bytes))
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
    let hex = String::deserialize(deserializer)?;

    // Only ASCII: slicing two bytes of a multi-byte character would panic.
    if !hex.is_ascii() || !hex.len().is_multiple_of(2) {
        return Err(D::Error::custom("invalid hex string"));
    }

    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).map_err(D::Error::custom))
        .collect()
}


fn encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
