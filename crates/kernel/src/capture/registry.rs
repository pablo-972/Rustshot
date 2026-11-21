use std::collections::BTreeMap;

use winreg::enums::{
    HKEY_LOCAL_MACHINE,
    HKEY_USERS,
    KEY_READ,
    REG_BINARY,
    REG_DWORD,
    REG_EXPAND_SZ,
    REG_MULTI_SZ,
    REG_QWORD,
    REG_SZ,
};
use winreg::types::FromRegValue;
use winreg::{HKEY, RegKey, RegValue};

use super::progress::Progress;
use crate::error::Result;
use crate::types::{
    RegistryKey,
    RegistryValue,
    SnapshotSummary
};


// Roots captured: the whole registry.
// HKCU is not listed: it is the current user's key inside HKU.
// HKCR is not listed: it is a merged view of HKLM\SOFTWARE\Classes and
// HKCU\Software\Classes, both already captured.
const ROOTS: [(&str, HKEY); 2] = [
    ("HKLM", HKEY_LOCAL_MACHINE),
    ("HKU", HKEY_USERS),
];


pub(crate) fn capture(
    report: &mut dyn FnMut(SnapshotSummary)
) -> Result<BTreeMap<String, RegistryKey>> {
    let mut keys = BTreeMap::new();
    let mut progress = Progress::new(report);

    for (name, hive) in ROOTS {
        walk(&RegKey::
            predef(hive), 
            name.to_string(), 
            &mut keys, 
            &mut progress
        );
    }

    progress.finish(keys.len());
    
    Ok(keys)
}


fn walk(
    key: &RegKey,
    path: String,
    out: &mut BTreeMap<String, RegistryKey>,
    progress: &mut Progress
) {
    let values = key
        .enum_values()
        .flatten()
        .map(|(name, value)| (name, convert(&value)))
        .collect();

    for sub in key.enum_keys().flatten() {
        if let Ok(child) = key.open_subkey_with_flags(&sub, KEY_READ) {
            walk(&child, format!("{path}\\{sub}"), out, progress);
        }
    }

    let captured = RegistryKey { values };
    let count = captured.values.len();

    out.insert(path, captured);
    progress.add(out.len(), count);
}

fn convert(value: &RegValue) -> RegistryValue {
    let typed = match value.vtype {
        REG_SZ => String::from_reg_value(value).ok().map(RegistryValue::Sz),
        REG_EXPAND_SZ => String::from_reg_value(value).ok().map(RegistryValue::ExpandSz),
        REG_MULTI_SZ => Vec::<String>::from_reg_value(value).ok().map(RegistryValue::MultiSz),
        REG_DWORD => u32::from_reg_value(value).ok().map(RegistryValue::Dword),
        REG_QWORD => u64::from_reg_value(value).ok().map(RegistryValue::Qword),
        REG_BINARY => Some(RegistryValue::Binary(value.bytes.to_vec())),
        _ => None,
    };

    typed.unwrap_or_else(|| RegistryValue::Other {
        kind: value.vtype.clone() as u32,
        bytes: value.bytes.to_vec(),
    })
}
