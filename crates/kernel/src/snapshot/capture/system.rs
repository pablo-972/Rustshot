//! Operating system and device of the machine where the snapshot is taken.

use std::env;

use crate::snapshot::types::{DeviceInfo, SystemInfo};


// Registry key where Windows keeps the details of the installed version.
// It exists from Windows 7 on.
#[cfg(windows)]
const CURRENT_VERSION: &str = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion";


#[cfg(windows)]
pub(super) fn system() -> SystemInfo {
    let key = VersionKey::open();

    SystemInfo {
        // On Windows 11 this still says "Windows 10": the build (22000 or
        // higher) tells them apart.
        name: key.text("ProductName"),
        version: version(&key),
        release: key.text("DisplayVersion").or_else(|| key.text("ReleaseId")),
        build: build(&key),
        service_pack: key.text("CSDVersion"),
        edition: key.text("EditionID"),
        architecture: architecture(),
    }
}


// Capture only runs on Windows: elsewhere only the basics are filled in.
#[cfg(not(windows))]
pub(super) fn system() -> SystemInfo {
    SystemInfo {
        name: Some(env::consts::OS.to_string()),
        architecture: Some(env::consts::ARCH.to_string()),
        ..SystemInfo::default()
    }
}


pub(super) fn device() -> DeviceInfo {
    DeviceInfo {
        hostname: variable("COMPUTERNAME").or_else(|| variable("HOSTNAME")),
        user: variable("USERNAME").or_else(|| variable("USER")),
        domain: variable("USERDOMAIN"),
    }
}


// The CurrentVersion key. If it cannot be opened, every value reads as None.
#[cfg(windows)]
struct VersionKey(Option<winreg::RegKey>);


#[cfg(windows)]
impl VersionKey {
    fn open() -> Self {
        use winreg::RegKey;
        use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};

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


// Kernel version, for example "6.1". Windows 10 and later keep it in two
// numbers and leave CurrentVersion at "6.3" for compatibility.
#[cfg(windows)]
fn version(key: &VersionKey) -> Option<String> {
    let major = key.number("CurrentMajorVersionNumber");
    let minor = key.number("CurrentMinorVersionNumber");

    match (major, minor) {
        (Some(major), Some(minor)) => Some(format!("{major}.{minor}")),
        _ => key.text("CurrentVersion"),
    }
}


// Build and revision, for example "7601.24544". UBR is the revision: the
// number of the last cumulative update.
#[cfg(windows)]
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
#[cfg(windows)]
fn architecture() -> Option<String> {
    variable("PROCESSOR_ARCHITEW6432").or_else(|| variable("PROCESSOR_ARCHITECTURE"))
}


// An environment variable, or None if it is missing or empty.
fn variable(name: &str) -> Option<String> {
    env::var(name).ok().filter(|value| !value.is_empty())
}
