//! Windows
//!
//! The two windows, capture and results, and the widgets they share.

mod about;
mod capture;
mod results;
mod widgets;
mod windows;

pub(crate) use about::about;
pub(crate) use capture::CaptureWindow;
pub(crate) use windows::Windows;
