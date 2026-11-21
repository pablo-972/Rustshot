//! The Windows registry: how it is captured, and how two captures of it are
//! compared.

mod capture;
mod compare;
mod progress;


pub(crate) use capture::capture;
pub(crate) use compare::compare;
