use std::env;

use crate::snapshot::types::{DeviceInfo, SystemInfo};


// Operating system details, read from the registry key where Windows keeps
// them. It exists from Windows 7 on.
#[cfg(windows)]
pub(super) fn system() -> SystemInfo {
    use winreg::RegKey;
    use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_READ};

    let key = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey_with_flags(r"SOFTWARE\Microsoft\Windows NT\CurrentVersion", KEY_READ)
        .ok();

    let text = |name: &str| key.as_ref()?.get_value::<String, _>(name).ok();
    let number = |name: &str| key.as_ref()?.get_value::<u32, _>(name).ok();

    // Windows 10 and later keep the real version in two numbers, and
    // CurrentVersion stays at "6.3" for compatibility.
    let version = match (number("CurrentMajorVersionNumber"), number("CurrentMinorVersionNumber")) {
        (Some(major), Some(minor)) => Some(format!("{major}.{minor}")),
        _ => text("CurrentVersion"),
    };

    // UBR is the revision: the number of the last cumulative update.
    let build = match (text("CurrentBuildNumber"), number("UBR")) {
        (Some(build), Some(revision)) => Some(format!("{build}.{revision}")),
        (build, _) => build,
    };

    SystemInfo {
        // On Windows 11 this still says "Windows 10": the build (22000 or
        // higher) tells them apart.
        name: text("ProductName"),
        version,
        release: text("DisplayVersion").or_else(|| text("ReleaseId")),
        build,
        service_pack: text("CSDVersion"),
        edition: text("EditionID"),
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


// Architecture of the system, not of this program: a 32-bit program on
// 64-bit Windows sees "x86" in PROCESSOR_ARCHITECTURE and the real one in
// PROCESSOR_ARCHITEW6432.
#[cfg(windows)]
fn architecture() -> Option<String> {
    variable("PROCESSOR_ARCHITEW6432").or_else(|| variable("PROCESSOR_ARCHITECTURE"))
}


fn variable(name: &str) -> Option<String> {
    env::var(name).ok().filter(|value| !value.is_empty())
}
