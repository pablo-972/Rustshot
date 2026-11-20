//! Types shared by the app
//! 
//! The views and the background tasks.

mod filter;
mod message;
mod state;

pub(crate) use filter::Filter;
pub(crate) use message::Message;
pub(crate) use state::{
    DiffResult,
    Shot,
    State,
    Status,
    Step,
    Task,
};
