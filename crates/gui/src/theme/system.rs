//! The theme chosen in the Windows settings, and the title bars.
//!
//! Windows 7 and 8 have no dark theme: there System is always light.

use std::ffi::c_void;
use std::ptr;

use winreg::RegKey;
use winreg::enums::HKEY_CURRENT_USER;


const PERSONALIZE: &str = r"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize";
const DARK_MODE: u32 = 20; // Since Windows 10 20H1
const DARK_MODE_OLD: u32 = 19; // On older builds of Windows 10


// Flags of SetWindowPos: keep the place, size and order of the window, do not
// activate it, and draw its frame again.
const FRAME_CHANGED: u32 = 0x0001 | 0x0002 | 0x0004 | 0x0010 | 0x0020;


#[link(name = "dwmapi")]
unsafe extern "system" {
    fn DwmSetWindowAttribute(
        window: *mut c_void,
        attribute: u32,
        value: *const c_void,
        size: u32
    ) -> i32;
}


#[link(name = "user32")]
unsafe extern "system" {
    fn SetWindowPos(
        window: *mut c_void,
        after: *mut c_void,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        flags: u32
    ) -> i32;
}


// Whether the apps are dark in the Windows settings. The value is 0 for dark
// and 1 for light, and it does not exist before Windows 10.
pub(super) fn dark() -> bool {
    RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey(PERSONALIZE)
        .and_then(|key| key.get_value::<u32, _>("AppsUseLightTheme"))
        .is_ok_and(|light| light == 0)
}


/// FLTK only draws inside the window: the title bar is drawn by Windows, and
/// it stays light unless it is asked for a dark one.
pub(super) fn title_bar(window: *mut c_void, dark: bool) {
    let enabled = i32::from(dark);
    let value = &enabled as *const i32 as *const c_void;
    let size = size_of::<i32>() as u32;

    // SAFETY: the window is shown, so its handle is valid, and the value
    // points to an i32 of the size passed. Both calls fail without harm on
    // Windows versions that have no dark title bar.
    unsafe {
        if DwmSetWindowAttribute(window, DARK_MODE, value, size) != 0 {
            DwmSetWindowAttribute(window, DARK_MODE_OLD, value, size);
        }

        // Without this, a window already shown keeps the old title bar until
        // it loses the focus.
        SetWindowPos(window, ptr::null_mut(), 0, 0, 0, 0, FRAME_CHANGED);
    }
}
