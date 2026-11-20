//! The console window of Rustshot.
//!
//! Rustshot.exe is a console program, because it is also the CLI. When it is
//! opened with a double click, Windows creates a console just for it.

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetConsoleProcessList(list: *mut u32, count: u32) -> u32;
    fn FreeConsole() -> i32;
}


/// Whether Rustshot was started from a terminal instead of a double click.
pub fn launched_from_terminal() -> bool {
    let mut processes = [0u32; 2];

    // SAFETY: the buffer has room for the 2 entries passed as its size.
    let attached = unsafe { GetConsoleProcessList(processes.as_mut_ptr(), 2) };

    // Only Rustshot uses the console: Windows created it for a double click.
    attached > 1
}


// Closes the console Windows created for a double click.
pub(crate) fn detach() {
    if !launched_from_terminal() {
        // SAFETY: FreeConsole has no preconditions.
        unsafe { FreeConsole(); }
    }
}
