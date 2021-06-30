use crate::results::action_result::ActionResult;
use crate::runner::error::Error;
use std::path::PathBuf;

pub mod error;
pub mod executor;
mod file;
pub mod runnable_action;
mod script;
pub mod state;
mod verify;

#[derive(Clone)]
pub enum RunEvent {
    SpecFileStarted(PathBuf),
    TestCompleted(ActionResult),
    SpecFileCompleted { success: bool },
    ErrorOccurred(Error),
}
