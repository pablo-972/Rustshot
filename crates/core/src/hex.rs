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


#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};


    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Bytes(#[serde(with = "crate::hex")] Vec<u8>);

    fn parse(json: &str) -> Result<Bytes, serde_json::Error> {
        serde_json::from_str(json)
    }

    #[test]
    fn bytes_are_lowercase_hex() {
        let json = serde_json::to_string(&Bytes(vec![0x0a, 0xff, 0x00])).unwrap();
        assert_eq!(json, r#""0aff00""#);
    }

    #[test]
    fn hex_goes_back_to_the_same_bytes() {
        assert_eq!(parse(r#""0aff00""#).unwrap(), Bytes(vec![0x0a, 0xff, 0x00]));
        assert_eq!(parse(r#""0AFF""#).unwrap(), Bytes(vec![0x0a, 0xff]));
        assert_eq!(parse(r#""""#).unwrap(), Bytes(vec![]));
    }

    #[test]
    fn invalid_hex_is_an_error_not_a_panic() {
        assert!(parse(r#""abc""#).is_err());
        assert!(parse(r#""zz""#).is_err());
        assert!(parse(r#""éé""#).is_err()); // 4 bytes: even, but not ASCII
    }
}
