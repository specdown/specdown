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
    /// Signal the process to terminate gracefully (SIGTERM equivalent).
    ///
    /// This is the *first* signal sent during graceful shutdown. The runner
    /// waits a short grace period for the process to exit; if it is still
    /// alive after that, [`kill`](Self::kill) (SIGKILL) is sent as a last
    /// resort.
    ///
    /// The default implementation delegates to [`kill`](Self::kill) for
    /// backwards compatibility with executors that do not distinguish
    /// between graceful and forceful termination.
    fn terminate(&mut self) {
        self.kill();
    }

    /// Forcefully kill the process (SIGKILL equivalent).
    ///
    /// This is the *escalation* signal used when [`terminate`](Self::terminate)
    /// did not stop the process within the grace period.
    fn kill(&mut self);

    /// Check if the process has already exited without blocking.
    ///
    /// Returns `Some(exit_code)` if the process has exited, or `None` if it
    /// is still running.
    fn try_wait(&mut self) -> Option<i32>;

    /// The OS process identifier, for signal delivery.
    ///
    /// Returns `None` on executors that do not expose a PID (e.g. the
    /// container executor tracks a PID inside the container instead). The
    /// shell [`BackgroundHandle`] impl for [`std::process::Child`] returns
    /// `Some(child.id())`.
    ///
    /// Currently only used by the Unix `terminate` implementation to deliver
    /// SIGTERM; on platforms that do not use it the method is still part of
    /// the trait surface for future executors.
    #[allow(dead_code)]
    fn pid(&self) -> Option<i32> {
        None
    }
}

impl BackgroundHandle for std::process::Child {
    fn terminate(&mut self) {
        // Send SIGTERM to the entire process group so that grandchildren
        // (e.g. python3 spawned by `bash -c`) are also terminated gracefully.
        // On Windows there is no SIGTERM equivalent, so fall back to
        // `TerminateProcess`.
        #[cfg(not(windows))]
        {
            if let Some(pid) = self.pid() {
                // SAFETY: `pid` is the child's real OS PID from `Child::id()`.
                // Since we spawned with `process_group(0)`, the child's PID
                // equals its PGID, so `killpg(pid, SIGTERM)` targets the
                // entire process group. `killpg` with a valid PGID and a
                // valid signal is safe.
                unsafe {
                    libc::killpg(pid, libc::SIGTERM);
                }
            }
        }
        #[cfg(windows)]
        {
            // On Windows there is no SIGTERM equivalent. TerminateProcess is
            // force-kill, so the grace-period poll in graceful_stop() is a
            // no-op — the process is already dead after this call. The
            // graceful_stop() function skips the grace-period loop on Windows
            // for this reason.
            let _ = std::process::Child::kill(self);
        }
    }

    fn kill(&mut self) {
        // Kill the entire process group first to reap grandchildren, then
        // kill the direct child as a fallback.
        #[cfg(not(windows))]
        {
            if let Some(pid) = self.pid() {
                // SAFETY: same rationale as `terminate` above.
                unsafe {
                    libc::killpg(pid, libc::SIGKILL);
                }
            }
        }
        let _ = std::process::Child::kill(self);
    }

    fn try_wait(&mut self) -> Option<i32> {
        std::process::Child::try_wait(self)
            .ok()
            .flatten()
            .and_then(|status| status.code())
    }

    fn pid(&self) -> Option<i32> {
        // `Child::id()` returns `u32`; the trait returns `i32`. PIDs are
        // always positive and fit in `i32` on all real platforms.
        #[allow(clippy::cast_possible_wrap)]
        Some(self.id() as i32)
    }
}
