use std::path::PathBuf;

pub use error::Error;
pub use executor::Executor;
pub use runnable_action::to_runnable;
pub use state::State;

use crate::results::ActionResult;

mod error;
mod executor;
mod file;
mod runnable_action;
mod script;
pub mod shell_executor;
mod state;
mod verify;

#[derive(Clone)]
pub enum RunEvent {
    SpecFileStarted(PathBuf),
    TestCompleted(ActionResult),
    SpecFileCompleted { success: bool },
    ErrorOccurred(Error),
}
