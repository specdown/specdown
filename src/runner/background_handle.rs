//! Trait abstracting a background process that can be killed and waited on.
//!
//! This allows the [`Executor`](super::executor::Executor) trait's `spawn`
//! method to return a type-erased handle rather than being hard-wired to
//! `std::process::Child`, enabling non-process-based executors (such as the
//! container executor) to support background scripts.

use std::fmt::Debug;

/// A handle to a background process that can be stopped and waited on.
///
/// Implementations include:
/// - [`ShellBackgroundHandle`](crate::runner::shell_executor::ShellBackgroundHandle)
///   wrapping `std::process::Child`
/// - [`ContainerBackgroundHandle`](crate::runner::container_executor::ContainerBackgroundHandle)
///   wrapping a Docker container managed via the socket API
///   (only available with the `container` feature)
pub trait BackgroundHandle: Debug + Send {
    /// Signal the process to terminate (SIGTERM equivalent).
    fn kill(&mut self);

    /// Wait for the process to exit and return its exit code.
    ///
    /// Returns `None` if the process was killed by a signal and no exit code
    /// is available.
    fn wait(&mut self) -> Option<i32>;

    /// Check if the process has already exited without blocking.
    ///
    /// Returns `Some(exit_code)` if the process has exited, or `None` if it
    /// is still running.
    fn try_wait(&mut self) -> Option<i32>;
}

impl BackgroundHandle for std::process::Child {
    fn kill(&mut self) {
        let _ = std::process::Child::kill(self);
    }

    fn wait(&mut self) -> Option<i32> {
        std::process::Child::wait(self)
            .ok()
            .and_then(|status| status.code())
    }

    fn try_wait(&mut self) -> Option<i32> {
        std::process::Child::try_wait(self)
            .ok()
            .and_then(|status| status?.code())
    }
}
