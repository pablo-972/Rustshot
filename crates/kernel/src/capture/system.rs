use std::env;

use winreg::RegKey;
use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};

use crate::types::{DeviceInfo, SystemInfo};


// Registry key where Windows keeps the details of the installed version.
const CURRENT_VERSION: &str = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion";


pub(super) fn system() -> SystemInfo {
    let key = VersionKey::open();

    SystemInfo {
        name: key.text("ProductName"),
        version: version(&key),
        release: key.text("DisplayVersion").or_else(|| key.text("ReleaseId")),
        build: build(&key),
        service_pack: key.text("CSDVersion"),
        edition: key.text("EditionID"),
        architecture: architecture(),
    }
}


pub(super) fn device() -> DeviceInfo {
    DeviceInfo {
        hostname: variable("COMPUTERNAME"),
        user: variable("USERNAME"),
        domain: variable("USERDOMAIN"),
    }
}


// The CurrentVersion key. If it cannot be opened, every value reads as None.
struct VersionKey(Option<RegKey>);


impl VersionKey {
    fn open() -> Self {
        let key = RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey_with_flags(CURRENT_VERSION, KEY_READ)
            .ok();

        Self(key)
    }

    fn text(&self, name: &str) -> Option<String> {
        self.0.as_ref()?.get_value(name).ok()
    }

    fn number(&self, name: &str) -> Option<u32> {
        self.0.as_ref()?.get_value(name).ok()
    }
}


fn version(key: &VersionKey) -> Option<String> {
    let major = key.number("CurrentMajorVersionNumber");
    let minor = key.number("CurrentMinorVersionNumber");

    match (major, minor) {
        (Some(major), Some(minor)) => Some(format!("{major}.{minor}")),
        _ => key.text("CurrentVersion"),
    }
}


fn build(key: &VersionKey) -> Option<String> {
    let build = key.text("CurrentBuildNumber")?;

    match key.number("UBR") {
        Some(revision) => Some(format!("{build}.{revision}")),
        None => Some(build),
    }
}


// Architecture of the system, not of this program: a 32-bit program on
// 64-bit Windows sees "x86" in PROCESSOR_ARCHITECTURE and the real one in
// PROCESSOR_ARCHITEW6432.
fn architecture() -> Option<String> {
    variable("PROCESSOR_ARCHITEW6432").or_else(|| variable("PROCESSOR_ARCHITECTURE"))
}


// An environment variable, or None if it is missing or empty.
fn variable(name: &str) -> Option<String> {
    env::var(name).ok().filter(|value| !value.is_empty())
}
