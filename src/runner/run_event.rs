use std::path::PathBuf;

use crate::results::ActionResult;
use crate::runner::Error;

#[derive(Clone)]
pub enum RunEvent {
    SpecFileStarted(PathBuf),
    TestCompleted(ActionResult),
    SpecFileCompleted { success: bool },
    ErrorOccurred(Error),
}
