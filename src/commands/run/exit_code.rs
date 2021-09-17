use crate::exit_codes::ExitCode;
use crate::runner::{Error, RunEvent};

pub fn from_events(events: &[RunEvent]) -> ExitCode {
    let mut exit_code = ExitCode::Success;

    for event in events {
        match event {
            RunEvent::SpecFileCompleted { success: false } => {
                if exit_code == ExitCode::Success {
                    exit_code = ExitCode::TestFailed;
                }
            }
            RunEvent::ErrorOccurred(error) => {
                return match error {
                    Error::RunFailed { .. } => ExitCode::TestFailed,
                    _ => ExitCode::ErrorOccurred,
                }
            }
            _ => {}
        }
    }

    exit_code
}
