//! Windows
//!
//! The two windows: capture and results.

mod capture;
mod file_field;
mod format;
mod icons;
mod results;
mod search_field;
mod widgets;
mod windows;

pub(crate) use capture::CaptureWindow;
pub(crate) use format::{file_name, format_count};
pub(crate) use windows::Windows;
