//! The light or dark theme chosen in the Windows settings.
//!
//! Windows 7 and 8 have no dark theme: there the interface is always light.

use std::ffi::c_void;
use std::sync::OnceLock;

use winreg::RegKey;
use winreg::enums::HKEY_CURRENT_USER;


const PERSONALIZE: &str = r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize";
const DARK_MODE: u32 = 20; // Since Windows 10 20H1
const DARK_MODE_OLD: u32 = 19; // On older builds of Windows 10


#[link(name = "dwmapi")]
unsafe extern "system" {
    fn DwmSetWindowAttribute(
        window: *mut c_void,
        attribute: u32,
        value: *const c_void,
        size: u32
    ) -> i32;
}


pub(super) fn dark() -> bool {
    static DARK: OnceLock<bool> = OnceLock::new();
    *DARK.get_or_init(read)
}


// The value is 0 for dark and 1 for light, and it does not exist before
// Windows 10.
fn read() -> bool {
    RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey(PERSONALIZE)
        .and_then(|key| key.get_value::<u32, _>("AppsUseLightTheme"))
        .is_ok_and(|light| light == 0)
}


/// FLTK only draws inside the window: the title bar is drawn by Windows, and
/// it stays light unless it is asked for a dark one.
pub(crate) fn dark_title_bar(window: *mut c_void) {
    if !dark() {
        return;
    }

    let enabled = 1i32;
    let value = &enabled as *const i32 as *const c_void;
    let size = size_of::<i32>() as u32;

    // SAFETY: the window is shown, so its handle is valid, and the value
    // points to an i32 of the size passed.
    unsafe {
        if DwmSetWindowAttribute(window, DARK_MODE, value, size) != 0 {
            DwmSetWindowAttribute(window, DARK_MODE_OLD, value, size);
        }
    }
}
