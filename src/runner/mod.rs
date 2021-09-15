use std::path::PathBuf;

pub use error::Error;
pub use executor::{Executor, Shell};
pub use runnable_action::to_runnable;

use crate::results::action_result::ActionResult;

mod error;
mod executor;
mod file;
mod runnable_action;
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
