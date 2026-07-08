use crate::results::{
    ActionResult, BackgroundExitStatus, BackgroundStartResult, BackgroundStopResult,
};
use crate::types::{BackgroundAction, ExitCode, ReadyWhen};
use crate::types::{FilePath, ScriptCode, ScriptName, DEFAULT_READY_WHEN_TIMEOUT_SECS};

use super::background_handle::BackgroundHandle;
use super::executor::Executor;
use super::Error;

use std::path::Path;
use std::time::{Duration, Instant};

/// How long to wait between readiness polls.
const READY_POLL_INTERVAL: Duration = Duration::from_millis(100);

/// Grace period after SIGTERM before escalating to SIGKILL, in milliseconds.
const GRACEFUL_SHUTDOWN_GRACE_MS: u64 = 5_000;

pub struct BackgroundProcess {
    pub handle: Box<dyn BackgroundHandle>,
    pub script_name: Option<ScriptName>,
}

pub fn start(
    action: &BackgroundAction,
    executor: &dyn Executor,
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
) -> Result<(), Error> {
    let name = script_name_name(script_name);
    let deadline = Instant::now() + Duration::from_secs(u64::from(timeout_secs));

    loop {
        // If the process has already exited, readiness can never be reached.
        if let Some(exit_code) = handle.try_wait() {
            handle.kill();
            return Err(Error::BackgroundExitedBeforeReady {
                script_name: name,
                exit_code,
            });
        }

        if condition_is_met(condition, executor) {
            return Ok(());
        }

        if Instant::now() >= deadline {
            handle.kill();
            let _ = handle.wait();
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
fn condition_is_met(condition: &ReadyWhen, executor: &dyn Executor) -> bool {
    match condition {
        ReadyWhen::FileExists(FilePath(path)) => Path::new(path).exists(),
        ReadyWhen::PortOpen(port) => {
            let addr_str = format!("127.0.0.1:{port}");
            let addr = addr_str.parse().unwrap_or_else(|_| {
                "127.0.0.1:0"
                    .parse::<std::net::SocketAddr>()
                    .expect("fallback socket addr is valid")
            });
            std::net::TcpStream::connect_timeout(&addr, Duration::from_millis(500)).is_ok()
        }
        ReadyWhen::CheckExitZero(script_code) => match executor.execute(script_code) {
            Ok(output) => output.exit_code == Some(0),
            // A command that fails to execute is not "ready yet" — keep
            // polling. (e.g. curl not yet installed, or server not up.)
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
        // The process exited before we could kill it.
        ActionResult::BackgroundStop(BackgroundStopResult {
            script_name: bg.script_name,
            exit_status: BackgroundExitStatus::Exited(ExitCode(exit_code)),
        })
    } else {
        // The process is still running. Send SIGTERM first (graceful), wait
        // a short grace period, then escalate to SIGKILL if it is still
        // alive.
        graceful_stop(&mut *bg.handle);
        let _ = bg.handle.wait();
        ActionResult::BackgroundStop(BackgroundStopResult {
            script_name: bg.script_name,
            exit_status: BackgroundExitStatus::Killed,
        })
    }
}

/// Send SIGTERM, poll for exit up to the grace period, then SIGKILL.
///
/// This implements the "TERM → wait → KILL" escalation so that background
/// processes get a chance to shut down gracefully (flush buffers, close
/// sockets) before being force-killed.
fn graceful_stop(handle: &mut dyn BackgroundHandle) {
    handle.terminate();

    let grace = Duration::from_millis(GRACEFUL_SHUTDOWN_GRACE_MS);
    let deadline = Instant::now() + grace;
    while Instant::now() < deadline {
        if handle.try_wait().is_some() {
            // Process exited gracefully after SIGTERM.
            return;
        }
        std::thread::sleep(Duration::from_millis(50));
    }

    // Still alive after the grace period — force kill.
    handle.kill();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::executor::Output;
    use crate::types::ScriptName;
    use std::sync::{Arc, Mutex};

    /// A mock background handle whose exit behaviour is scriptable.
    ///
    /// - `try_wait` returns `None` until `exit_after` calls have been made,
    ///   then returns `Some(exit_code)`.
    /// - `kill`/`terminate`/`wait` are recorded.
    #[derive(Debug)]
    struct MockHandle {
        exit_code: i32,
        try_wait_calls: Arc<Mutex<usize>>,
        killed: Arc<Mutex<bool>>,
        terminated: Arc<Mutex<bool>>,
        waited: Arc<Mutex<bool>>,
    }

    impl MockHandle {
        fn running(exit_code: i32) -> Self {
            Self {
                exit_code,
                try_wait_calls: Arc::new(Mutex::new(0)),
                killed: Arc::new(Mutex::new(false)),
                terminated: Arc::new(Mutex::new(false)),
                waited: Arc::new(Mutex::new(false)),
            }
        }
    }

    impl BackgroundHandle for MockHandle {
        fn terminate(&mut self) {
            *self.terminated.lock().expect("mtx") = true;
        }
        fn kill(&mut self) {
            *self.killed.lock().expect("mtx") = true;
        }
        fn wait(&mut self) -> Option<i32> {
            *self.waited.lock().expect("mtx") = true;
            Some(self.exit_code)
        }
        fn try_wait(&mut self) -> Option<i32> {
            let mut calls = self.try_wait_calls.lock().expect("mtx");
            *calls += 1;
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
        assert!(condition_is_met(&cond, &ReadyExecutor));
    }

    #[test]
    fn condition_is_met_file_missing_when_path_absent() {
        let cond = ReadyWhen::FileExists(FilePath(
            "/this/path/should/not/exist/specdown-test".to_string(),
        ));
        assert!(!condition_is_met(&cond, &ReadyExecutor));
    }

    #[test]
    fn condition_is_met_check_exit_zero_when_executor_exits_zero() {
        let cond = ReadyWhen::CheckExitZero(ScriptCode("true".to_string()));
        assert!(condition_is_met(&cond, &ReadyExecutor));
    }

    #[test]
    fn condition_is_met_check_exit_zero_false_when_executor_exits_nonzero() {
        let cond = ReadyWhen::CheckExitZero(ScriptCode("false".to_string()));
        assert!(!condition_is_met(&cond, &NeverReadyExecutor));
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
        let mut handle = MockHandle::running(0);
        let cond = ReadyWhen::CheckExitZero(ScriptCode("true".to_string()));
        let name = ScriptName("srv".to_string());
        // condition_is_met returns true immediately
        let result = wait_for_ready(&mut handle, &cond, 1, Some(&name), &ReadyExecutor);
        assert!(result.is_ok());
    }

    #[test]
    fn wait_for_ready_returns_timeout_when_never_ready() {
        let mut handle = MockHandle::running(0);
        let cond = ReadyWhen::CheckExitZero(ScriptCode("false".to_string()));
        let name = ScriptName("srv".to_string());
        // timeout_secs=1 -> should time out quickly
        let result = wait_for_ready(&mut handle, &cond, 1, Some(&name), &NeverReadyExecutor);
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
            other => panic!("expected ReadyWhenTimeout, got {:?}", other),
        }
    }

    #[test]
    fn graceful_stop_sends_terminate_then_kill_if_still_alive() {
        let mut handle = MockHandle::running(0);
        graceful_stop(&mut handle);
        assert!(*handle.terminated.lock().expect("mtx"));
        assert!(*handle.killed.lock().expect("mtx"));
    }

    #[test]
    fn stop_reports_killed_when_process_was_running() {
        let handle: Box<dyn BackgroundHandle> = Box::new(MockHandle::running(0));
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
            other => panic!("expected Killed, got {:?}", other),
        }
    }
}
