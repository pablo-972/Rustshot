use std::collections::BTreeMap;

use crate::snapshot::error::Result;
use crate::snapshot::types::RegistryKey;


// Roots captured: the whole registry, like Regshot.
// HKCU is not listed: it is the current user's key inside HKU.
// HKCR is not listed: it is a merged view of HKLM\SOFTWARE\Classes and
// HKCU\Software\Classes, both already captured.
#[cfg(windows)]
const ROOTS: [&str; 2] = ["HKLM", "HKU"];


#[cfg(windows)]
pub(crate) fn capture() -> Result<BTreeMap<String, RegistryKey>> {
    let mut keys = BTreeMap::new();
    for root in ROOTS { capture_root(root, &mut keys); }
    
    Ok(keys)
}


#[cfg(windows)]
fn capture_root(root: &str, out: &mut BTreeMap<String, RegistryKey>) {
    use winreg::RegKey;
    use winreg::enums::{
        HKEY_CLASSES_ROOT,
        HKEY_CURRENT_USER,
        HKEY_LOCAL_MACHINE,
        HKEY_USERS,
        KEY_READ,
    };

    let (hive_name, sub) = root.split_once('\\').unwrap_or((root, ""));
    let hive = match hive_name {
        "HKLM" => HKEY_LOCAL_MACHINE,
        "HKCU" => HKEY_CURRENT_USER,
        "HKCR" => HKEY_CLASSES_ROOT,
        "HKU" => HKEY_USERS,
        _ => return,
    };

    // Keys that don't exist on this Windows version are skipped.
    if let Ok(key) = RegKey::predef(hive).open_subkey_with_flags(sub, KEY_READ) {
        walk(&key, root.to_string(), out);
    }
}


#[cfg(windows)]
fn walk(
    key: &winreg::RegKey,
    path: String,
    out: &mut BTreeMap<String, RegistryKey>
) {
    use winreg::enums::KEY_READ;

    let values = key
        .enum_values()
        .flatten()
        .map(|(name, value)| (name, convert(&value)))
        .collect();

    for sub in key.enum_keys().flatten() {
        if let Ok(child) = key.open_subkey_with_flags(&sub, KEY_READ) {
            walk(&child, format!("{path}\\{sub}"), out);
        }
    }

    out.insert(path, RegistryKey { values });
}


#[cfg(windows)]
fn convert(value: &winreg::RegValue) -> crate::snapshot::types::RegistryValue {
    use crate::snapshot::types::RegistryValue;
    use winreg::types::FromRegValue;
    use winreg::enums::{
        REG_BINARY,
        REG_DWORD,
        REG_EXPAND_SZ,
        REG_MULTI_SZ,
        REG_QWORD,
        REG_SZ,
    };

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


// The registry only exists on Windows.
#[cfg(not(windows))]
pub(crate) fn capture() -> Result<BTreeMap<String, RegistryKey>> {
    Ok(BTreeMap::new())
}