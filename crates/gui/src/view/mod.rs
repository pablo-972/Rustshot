//! Windows
//!
//! The two windows: capture and results.

mod capture;
mod format;
mod results;
mod widgets;
mod windows;

pub(crate) use capture::CaptureWindow;
pub(crate) use format::{file_name, format_count};
pub(crate) use windows::Windows;
