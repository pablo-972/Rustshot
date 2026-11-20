//! How Rustshot was opened: with a double click or from a terminal.
//!
//! Rustshot.exe is a console program, because it is also the CLI. When it is
//! opened with a double click, Windows creates a console just for it.

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetConsoleProcessList(list: *mut u32, count: u32) -> u32;
    fn FreeConsole() -> i32;
}


pub fn is_gui() -> bool {
    let mut processes = [0u32; 2];
    let attached = unsafe { GetConsoleProcessList(processes.as_mut_ptr(), 2) };
    
    attached == 1
}


/// Closes the console Windows created for the double click and opens the GUI.
pub fn free_console() {
    unsafe { FreeConsole(); }
}

