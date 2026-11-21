//! Types shared by the app
//!
//! The views and the background tasks.

mod message;
mod state;


pub(crate) use message::{Filter, Message};
pub(crate) use state::{
    DiffFile,
    Progress,
    ShotFile,
    State,
    Status,
    Step,
    Task,
    ThemeMode,
};
