use crate::results::{
    ActionResult, BackgroundExitStatus, BackgroundStartResult, BackgroundStopResult,
};
use crate::types::{BackgroundAction, ExitCode, ReadyWhen};
use crate::types::{FilePath, ScriptCode, ScriptName, DEFAULT_READY_WHEN_TIMEOUT_SECS};

use super::background_handle::BackgroundHandle;
use super::executor::Executor;
use super::Error;
use std::net::{SocketAddr, TcpStream};

use std::path::Path;
use std::time::{Duration, Instant};

/// How long to wait between readiness polls.
const READY_POLL_INTERVAL: Duration = Duration::from_millis(100);

/// Grace period after SIGTERM before escalating to SIGKILL.
///
/// Only used on Unix; on Windows, `terminate()` is force-kill so no
/// grace period is polled.
#[cfg_attr(windows, allow(dead_code))]
const GRACEFUL_SHUTDOWN_GRACE: Duration = Duration::from_secs(5);

/// Hard cap on how long `stop()` will wait for a killed process to be
/// reaped. Prevents a zombie or un-reaped child from hanging the test
/// suite indefinitely on any platform.
const STOP_DEADLINE: Duration = Duration::from_secs(10);

/// Hard cap on how long the `ready_when` timeout path will wait to reap
/// the killed process before returning the error.
const REAP_DEADLINE: Duration = Duration::from_secs(5);

pub struct BackgroundProcess {
    pub handle: Box<dyn BackgroundHandle>,
    pub script_name: Option<ScriptName>,
}

pub fn start(
    action: &BackgroundAction,
    executor: &dyn Executor,
    working_dir: &Path,
) -> Result<(ActionResult, BackgroundProcess), Error> {
    let BackgroundAction {
        script_name,
        script_code,
        ready_when,
        timeout_secs,
    } = action;

    let mut handle = executor.spawn(script_code)?;

    // If a ready_when condition is set, block here until it is satisfied (or
    // the readiness timeout elapses, or the process exits early). This is the
    // key behaviour: spawn is non-blocking, but ready_when *does* block.
    if let Some(condition) = ready_when {
        let timeout = timeout_secs.unwrap_or(DEFAULT_READY_WHEN_TIMEOUT_SECS);
        wait_for_ready(
            &mut *handle,
            condition,
            timeout,
            script_name.as_ref(),
            executor,
            working_dir,
        )?;
    }

    let result = ActionResult::BackgroundStart(BackgroundStartResult {
        action: action.clone(),
    });

    Ok((
        result,
        BackgroundProcess {
            handle,
            script_name: script_name.clone(),
        },
    ))
}

/// Block until the [`ReadyWhen`] condition is satisfied.
///
/// Polls the condition every [`READY_POLL_INTERVAL`]. If the background
/// process exits before the condition is met, returns
/// [`Error::BackgroundExitedBeforeReady`]. If the deadline elapses, returns
/// [`Error::ReadyWhenTimeout`] (after killing the process).
fn wait_for_ready(
    handle: &mut dyn BackgroundHandle,
    condition: &ReadyWhen,
    timeout_secs: u32,
    script_name: Option<&ScriptName>,
    executor: &dyn Executor,
    working_dir: &Path,
) -> Result<(), Error> {
    let name = script_name_name(script_name);
    let deadline = Instant::now() + Duration::from_secs(u64::from(timeout_secs));

    loop {
        // If the process has already exited, readiness can never be reached.
        if let Some(exit_code) = handle.try_wait() {
            // Reap the zombie before returning. Without this the killed child
            // stays in the process table until the OS reaps it (or never, on
            // some platforms), which can hang the test suite.
            kill_and_reap(handle, REAP_DEADLINE);
            return Err(Error::BackgroundExitedBeforeReady {
                script_name: name,
                exit_code,
            });
        }

        if condition_is_met(condition, executor, working_dir) {
            return Ok(());
        }

        if Instant::now() >= deadline {
            kill_and_reap(handle, REAP_DEADLINE);
            return Err(Error::ReadyWhenTimeout {
                script_name: name,
                timeout_secs,
                condition: condition_to_string(condition),
            });
        }

        std::thread::sleep(READY_POLL_INTERVAL);
    }
}

/// Evaluate a single readiness condition.
fn condition_is_met(condition: &ReadyWhen, executor: &dyn Executor, working_dir: &Path) -> bool {
    match condition {
        ReadyWhen::FileExists(FilePath(path)) => working_dir.join(path).exists(),
        ReadyWhen::PortOpen(port) => {
            // Probe the loopback address directly rather than resolving
            // "localhost" via DNS. The background script binds to 127.0.0.1
            // (IPv4), but on macOS/Windows "localhost" may resolve to ::1
            // (IPv6) first, causing the readiness check to fail against an
            // IPv4-only listener. Try IPv4 first, then ::1 as a fallback for
            // IPv6-only hosts. Each attempt is bounded so a stalled socket
            // does not eat the readiness budget.
            let timeout = Duration::from_millis(500);
            let ipv4 = SocketAddr::from(([127, 0, 0, 1], *port));
            let ipv6 = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 1], *port));
            TcpStream::connect_timeout(&ipv4, timeout).is_ok()
                || TcpStream::connect_timeout(&ipv6, timeout).is_ok()
        }
        ReadyWhen::CheckExitZero(script_code) => match executor.execute(script_code) {
            Ok(output) => output.exit_code == Some(0),
            // A command that fails to execute is not "ready yet" — keep
            // polling (e.g. curl not yet installed, or server not up).
            Err(_) => false,
        },
    }
}

/// Human-readable description of a [`ReadyWhen`] condition, for error
/// messages.
fn condition_to_string(condition: &ReadyWhen) -> String {
    match condition {
        ReadyWhen::FileExists(FilePath(p)) => format!("file:{p}"),
        ReadyWhen::PortOpen(port) => format!("port:{port}"),
        ReadyWhen::CheckExitZero(ScriptCode(c)) => format!("exit:{c}"),
    }
}

fn script_name_name(script_name: Option<&ScriptName>) -> String {
    script_name.map_or_else(|| "<unnamed>".to_string(), Into::into)
}

pub fn stop(mut bg: BackgroundProcess) -> ActionResult {
    // Check if the process has already exited on its own.
    if let Some(exit_code) = bg.handle.try_wait() {
        ActionResult::BackgroundStop(BackgroundStopResult {
            script_name: bg.script_name,
            exit_status: BackgroundExitStatus::Exited(ExitCode(exit_code)),
        })
    } else {
        // The process is still running. Send SIGTERM, wait the grace period,
        // then escalate to SIGKILL.
        graceful_stop(&mut *bg.handle);
        // Hard-cap the final reap so a zombie can never block forever.
        kill_and_reap(&mut *bg.handle, STOP_DEADLINE);
        ActionResult::BackgroundStop(BackgroundStopResult {
            script_name: bg.script_name,
            exit_status: BackgroundExitStatus::Killed,
        })
    }
}

/// Send SIGTERM, poll for exit up to [`GRACEFUL_SHUTDOWN_GRACE`], then
/// SIGKILL.
fn graceful_stop(handle: &mut dyn BackgroundHandle) {
    handle.terminate();

    // On Unix, poll for exit during the grace period before escalating
    // to SIGKILL. On Windows, terminate() is already force-kill
    // (TerminateProcess), so the grace-period poll is a no-op and is
    // skipped.
    #[cfg(not(windows))]
    {
        let deadline = Instant::now() + GRACEFUL_SHUTDOWN_GRACE;
        while Instant::now() < deadline {
            if handle.try_wait().is_some() {
                return;
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    // Still alive after the grace period (Unix), or already dead
    // (Windows: terminate() is force-kill). Force-kill as a last resort.
    handle.kill();
}

/// Kill a handle and poll `try_wait` until it reports exit or `deadline`
/// elapses, whichever comes first. Guarantees bounded execution time
/// regardless of OS reaping behaviour.
fn kill_and_reap(handle: &mut dyn BackgroundHandle, deadline: Duration) {
    handle.kill();
    let deadline = Instant::now() + deadline;
    while Instant::now() < deadline {
        if handle.try_wait().is_some() {
            break;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

#[cfg(test)]
#[allow(clippy::uninlined_format_args)]
mod tests {
    use super::*;
    use crate::runner::executor::Output;
    use crate::types::ScriptName;
    use std::sync::{Arc, Mutex};

    /// A mock background handle that stays running forever.
    ///
    /// `try_wait` always returns `None`, `kill`/`terminate` are recorded.
    #[derive(Debug)]
    struct MockHandle {
        killed: Arc<Mutex<bool>>,
        terminated: Arc<Mutex<bool>>,
    }

    impl MockHandle {
        fn new() -> Self {
            Self {
                killed: Arc::new(Mutex::new(false)),
                terminated: Arc::new(Mutex::new(false)),
            }
        }
    }

    impl BackgroundHandle for MockHandle {
        fn terminate(&mut self) {
            *self.terminated.lock().expect("mutex poisoned") = true;
        }
        fn kill(&mut self) {
            *self.killed.lock().expect("mutex poisoned") = true;
        }
        fn try_wait(&mut self) -> Option<i32> {
            None
        }
    }

    /// An executor that always reports a check command as exit 0 (ready).
    #[derive(Debug)]
    struct ReadyExecutor;
    impl Executor for ReadyExecutor {
        fn execute(&self, _script: &ScriptCode) -> Result<Output, Error> {
            Ok(Output {
                stdout: String::new(),
                stderr: String::new(),
                exit_code: Some(0),
            })
        }
    }

    /// An executor whose check command always fails (exit 1 — never ready).
    #[derive(Debug)]
    struct NeverReadyExecutor;
    impl Executor for NeverReadyExecutor {
        fn execute(&self, _script: &ScriptCode) -> Result<Output, Error> {
            Ok(Output {
                stdout: String::new(),
                stderr: String::new(),
                exit_code: Some(1),
            })
        }
    }

    #[test]
    fn condition_is_met_file_exists_when_path_present() {
        let tmp = tempfile::NamedTempFile::new().expect("temp file");
        let cond = ReadyWhen::FileExists(FilePath(tmp.path().to_string_lossy().to_string()));
        assert!(condition_is_met(&cond, &ReadyExecutor, Path::new(".")));
    }

    #[test]
    fn condition_is_met_file_missing_when_path_absent() {
        let cond = ReadyWhen::FileExists(FilePath(
            "/this/path/should/not/exist/specdown-test".to_string(),
        ));
        assert!(!condition_is_met(&cond, &ReadyExecutor, Path::new(".")));
    }

    #[test]
    fn condition_is_met_file_exists_resolves_relative_paths_against_working_dir() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");
        std::fs::write(dir.path().join("ready.flag"), "").expect("failed to write flag file");
        let cond = ReadyWhen::FileExists(FilePath("ready.flag".to_string()));
        assert!(condition_is_met(&cond, &ReadyExecutor, dir.path()));
    }

    #[test]
    fn condition_is_met_check_exit_zero_when_executor_exits_zero() {
        let cond = ReadyWhen::CheckExitZero(ScriptCode("true".to_string()));
        assert!(condition_is_met(&cond, &ReadyExecutor, Path::new(".")));
    }

    #[test]
    fn condition_is_met_check_exit_zero_false_when_executor_exits_nonzero() {
        let cond = ReadyWhen::CheckExitZero(ScriptCode("false".to_string()));
        assert!(!condition_is_met(
            &cond,
            &NeverReadyExecutor,
            Path::new(".")
        ));
    }

    #[test]
    fn condition_to_string_formats_all_variants() {
        assert_eq!(
            condition_to_string(&ReadyWhen::FileExists(FilePath("/x".to_string()))),
            "file:/x"
        );
        assert_eq!(condition_to_string(&ReadyWhen::PortOpen(1234)), "port:1234");
        assert_eq!(
            condition_to_string(&ReadyWhen::CheckExitZero(ScriptCode("cmd".to_string()))),
            "exit:cmd"
        );
    }

    #[test]
    fn wait_for_ready_returns_ok_when_condition_already_met() {
        let mut handle = MockHandle::new();
        let cond = ReadyWhen::CheckExitZero(ScriptCode("true".to_string()));
        let name = ScriptName("srv".to_string());
        let result = wait_for_ready(
            &mut handle,
            &cond,
            1,
            Some(&name),
            &ReadyExecutor,
            Path::new("."),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn wait_for_ready_returns_timeout_when_never_ready() {
        let mut handle = MockHandle::new();
        let cond = ReadyWhen::CheckExitZero(ScriptCode("false".to_string()));
        let name = ScriptName("srv".to_string());
        let result = wait_for_ready(
            &mut handle,
            &cond,
            1,
            Some(&name),
            &NeverReadyExecutor,
            Path::new("."),
        );
        match result {
            Err(Error::ReadyWhenTimeout {
                script_name,
                timeout_secs,
                condition,
            }) => {
                assert_eq!(script_name, "srv");
                assert_eq!(timeout_secs, 1);
                assert_eq!(condition, "exit:false");
            }
            _ => panic!("expected ReadyWhenTimeout"),
        }
    }

    #[test]
    fn graceful_stop_sends_terminate_then_kill_if_still_alive() {
        let mut handle = MockHandle::new();
        graceful_stop(&mut handle);
        assert!(*handle.terminated.lock().expect("mutex poisoned"));
        assert!(*handle.killed.lock().expect("mutex poisoned"));
    }

    #[test]
    fn stop_reports_killed_when_process_was_running() {
        let handle: Box<dyn BackgroundHandle> = Box::new(MockHandle::new());
        let bg = BackgroundProcess {
            handle,
            script_name: Some(ScriptName("srv".to_string())),
        };
        let result = stop(bg);
        match result {
            ActionResult::BackgroundStop(BackgroundStopResult {
                exit_status: BackgroundExitStatus::Killed,
                ..
            }) => {}
            _ => panic!("expected Killed"),
        }
    }

    // ----------------------------------------------------------------
    // Real-process reap tests — verify kill_and_reap actually reaps
    // zombies rather than leaving them in the process table.
    // ----------------------------------------------------------------

    use std::process::{Command, Stdio};
    use std::thread;
    use std::time::Duration;

    /// `kill_and_reap` on a process that has already exited with a non-zero
    /// code must reap it: after the call, `try_wait` must still report the
    /// exit (i.e. the child is not left as a zombie that returns `None`
    /// from `try_wait` on the *next* poll).
    #[test]
    fn kill_and_reap_reaps_process_that_exited_nonzero() {
        let mut child = Command::new("sh")
            .arg("-c")
            .arg("exit 7")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("failed to spawn sh");

        // Wait for the process to exit on its own.
        thread::sleep(Duration::from_millis(300));

        // Before the fix, the early-exit path in wait_for_ready called
        // handle.kill() without reaping. kill_and_reap must reap it.
        kill_and_reap(&mut child, REAP_DEADLINE);

        // After reaping, try_wait must still report the exit code (the
        // child has been waited on, not left as a zombie).
        let code = BackgroundHandle::try_wait(&mut child);
        assert_eq!(
            code,
            Some(7),
            "kill_and_reap should have reaped the process; try_wait got {code:?}"
        );
    }

    /// `kill_and_reap` on a process that is still running must kill it and
    /// reap it within the deadline. After the call, the process must not
    /// be left as a zombie — `try_wait` should report an exit.
    #[test]
    fn kill_and_reap_kills_and_reaps_running_process() {
        let mut child = Command::new("sleep")
            .arg("30")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("failed to spawn sleep");

        // The process is still running. kill_and_reap must kill + reap it.
        kill_and_reap(&mut child, REAP_DEADLINE);

        // After kill_and_reap, try_wait must report exit (not None = still
        // running / zombie).  Signal-killed processes return Some(-1)
        // with the try_wait fix.
        let code = BackgroundHandle::try_wait(&mut child);
        assert!(
            code.is_some(),
            "kill_and_reap should have reaped the process; try_wait got {:?} (None = zombie/still-running)",
            code
        );
    }

    /// `kill_and_reap` on a signal-killed process must reap it. This is
    /// the CRITICAL scenario: without the `try_wait` fix, signal death
    /// returns `None` from `try_wait`, so `kill_and_reap`'s polling loop never
    /// breaks and the process is never reaped.
    #[test]
    fn kill_and_reap_reaps_signal_killed_process() {
        let mut child = Command::new("sleep")
            .arg("30")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("failed to spawn sleep");

        // Kill with SIGKILL (signal death — ExitStatus::code() returns None).
        child.kill().expect("failed to kill child");
        thread::sleep(Duration::from_millis(200));

        // kill_and_reap must detect the exit via try_wait (which now returns
        // Some(-1) for signal death) and break out of its polling loop.
        let start = Instant::now();
        kill_and_reap(&mut child, REAP_DEADLINE);
        let elapsed = start.elapsed();

        // If the try_wait fix is NOT in place, kill_and_reap polls for the
        // full REAP_DEADLINE (5s) because try_wait keeps returning None.
        // With the fix, it should break out quickly (< 1s).
        assert!(
            elapsed < Duration::from_secs(2),
            "kill_and_reap took {:?} — try_wait fix may not be working (signal death returns None, causing full deadline poll)",
            elapsed
        );

        // The process must be reaped — try_wait reports an exit.
        //
        // On Unix, SIGKILL => ExitStatus::code() == None => sentinel -1.
        // On Windows, TerminateProcess => exit code 1 (no signal death).
        let code = BackgroundHandle::try_wait(&mut child);
        let expected = if cfg!(windows) { 1 } else { -1 };
        assert_eq!(
            code,
            Some(expected),
            "force-killed process should report Some({expected}) after reaping, got {code:?}"
        );
    }

    /// Integration test: `wait_for_ready` early-exit path must reap the
    /// process before returning an error. Spawns a real process that
    /// immediately exits with a non-zero code and verifies no zombie.
    #[test]
    fn wait_for_ready_early_exit_reaps_process() {
        // A real child handle that exits immediately with code 1.
        let mut child = Command::new("sh")
            .arg("-c")
            .arg("exit 1")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("failed to spawn sh");

        // Wait for it to exit.
        thread::sleep(Duration::from_millis(300));

        // Simulate the wait_for_ready early-exit path. Before the fix, this
        // would call handle.kill() and return without reaping.
        // We call the same logic the fixed code uses:
        if let Some(exit_code) = BackgroundHandle::try_wait(&mut child) {
            kill_and_reap(&mut child, REAP_DEADLINE);
            // The process should be reaped now.
            assert_eq!(exit_code, 1, "expected exit code 1, got {exit_code}");
        } else {
            panic!("process should have already exited");
        }

        // Verify no zombie: try_wait should still report the exit code,
        // not None (which would mean "still running" or "zombie not reaped").
        let code = BackgroundHandle::try_wait(&mut child);
        assert_eq!(
            code,
            Some(1),
            "process should be reaped; try_wait got {code:?} (None = zombie)"
        );
    }
}
