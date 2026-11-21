use std::collections::BTreeMap;

use winreg::enums::{
    HKEY_LOCAL_MACHINE,
    HKEY_USERS,
    KEY_READ,
    KEY_WOW64_64KEY,
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
use crate::types::{
    RegistryKey,
    RegistryValue,
    SnapshotSummary,
};


// Roots captured: the whole registry.
// HKCU is not listed: it is the current user's key inside HKU.
// HKCR is not listed: it is a merged view of HKLM\SOFTWARE\Classes and
// HKCU\Software\Classes, both already captured.
const ROOTS: [(&str, HKEY); 2] = [
    ("HKLM", HKEY_LOCAL_MACHINE),
    ("HKU", HKEY_USERS),
];


// What the capture of the registry gathers.
#[derive(Default)]
pub(crate) struct Registry {
    pub keys: BTreeMap<String, RegistryKey>,
    pub inaccessible: Vec<String>,
}


pub(crate) fn capture(report: &mut dyn FnMut(SnapshotSummary)) -> Registry {
    let mut registry = Registry::default();
    let mut progress = Progress::new(report);

    for (name, hive) in ROOTS {
        let root = RegKey::predef(hive);
        walk(&root, name.to_string(), &mut registry, &mut progress);
    }

    registry.inaccessible.sort();
    progress.finish();

    registry
}


fn walk(
    key: &RegKey,
    path: String,
    registry: &mut Registry,
    progress: &mut Progress
) {
    let values = key
        .enum_values()
        .flatten()
        .map(|(name, value)| (name, convert(&value)))
        .collect();

    for sub in key.enum_keys().flatten() {
        let child_path = format!("{path}\\{sub}");

        // It is listed, so it exists: if it cannot be opened, access is
        // usually denied.
        match key.open_subkey_with_flags(&sub, KEY_READ | KEY_WOW64_64KEY) {
            Ok(child) => walk(&child, child_path, registry, progress),
            Err(_) => {
                registry.inaccessible.push(child_path);
                progress.inaccessible();
            }
        }
    }

    let captured = RegistryKey { values };
    progress.key(captured.values.len());
    registry.keys.insert(path, captured);
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