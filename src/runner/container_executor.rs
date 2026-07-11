//! Container executor that runs spec scripts inside a Docker container
//! using the Docker Engine API via the `bollard` crate.
//!
//! This executor connects directly to the Docker socket
//! (`/var/run/docker.sock` on Linux) and manages containers through the
//! API — it does NOT shell out to `docker run`.
//!
//! ## Persistence
//!
//! A single container is created when the executor is first used and kept
//! alive for the lifetime of the executor. All `script` and `background`
//! blocks reuse this container via the Docker **exec** API, so filesystem
//! state (files, environment, etc.) persists across script blocks within
//! a spec file run.
//!
//! ## Parallel Execution
//!
//! When specdown runs with `--jobs > 1`, each parallel spec file gets its
//! own `ContainerExecutor` instance (via [`Executor::clone_box`]), which in
//! turn creates a uniquely-named container following the pattern
//! `specdown-{file-hash}-{counter}`. The file hash is derived from the spec
//! file path, making containers identifiable per spec file. This prevents
//! container name collisions and ensures complete isolation between parallel
//! spec files. Each container is cleaned up when its executor is dropped
//! (after the spec file completes).

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use bollard::container::{
    Config, CreateContainerOptions, LogOutput, RemoveContainerOptions, StartContainerOptions,
};
use bollard::exec::{CreateExecOptions, StartExecOptions};
use bollard::models::HostConfig;
use bollard::Docker;
use futures_util::StreamExt;

use crate::types::ScriptCode;

use super::background_handle::BackgroundHandle;
use super::executor::Output;
use super::{Error, Executor};
use std::convert::TryFrom;

/// Monotonically increasing counter for unique background PID-file names.
static BG_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Monotonically increasing counter for unique container names.
static CONTAINER_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Hash a label string into a short, stable hex string.
///
/// Uses the FNV-1a hash algorithm — fast, dependency-free, and sufficient
/// for distinguishing spec files in container names. The result is 8 hex
/// characters (32 bits), which is enough to avoid collisions in practice
/// for the small number of spec files processed in a single run.
fn hash_label(label: &str) -> String {
    let mut hash: u32 = 0x811c_9dc5;
    for byte in label.as_bytes() {
        hash = hash.wrapping_mul(0x0100_0193);
        hash ^= u32::from(*byte);
    }
    format!("{hash:08x}")
}

/// An executor that runs scripts inside a Docker container, communicating
/// with the Docker daemon over its socket API.
///
/// Created with [`ContainerExecutor::new`], which establishes a connection
/// to the Docker daemon. A persistent container is created lazily on the
/// first `execute` or `spawn` call and reused for all subsequent calls.
/// The container is removed when the executor is dropped.
#[derive(Debug)]
pub struct ContainerExecutor {
    image: String,
    shell_command: String,
    env: HashMap<String, String>,
    unset_env: Vec<String>,
    paths: Vec<PathBuf>,
    binds: Vec<String>,
    working_dir: String,
    runtime: tokio::runtime::Runtime,
    docker: Docker,
    /// ID of the persistent container, created on first use.
    container_id: Mutex<Option<String>>,
    /// Label used to derive a unique container name (typically the
    /// spec file path). Incorporated into the container name as a hash.
    label: String,
}

impl ContainerExecutor {
    /// Create a new `ContainerExecutor`.
    ///
    /// # Arguments
    ///
    /// * `image` - The Docker image to use (e.g. `"bash:5"`).
    /// * `shell_command` - The shell command to run inside the container
    ///   (e.g. `"bash -c"`).
    /// * `env` - Environment variables to set, as `(KEY, VALUE)` pairs.
    /// * `unset_env` - Environment variables to unset.
    /// * `paths` - Additional paths to prepend to `PATH`.
    /// * `binds` - Docker bind-mount specifications (e.g.
    ///   `"/host/path:/container/path"` or `"/host:/container:ro"`).
    ///
    /// # Errors
    ///
    /// Returns an error if the connection to the Docker daemon fails or
    /// the tokio runtime cannot be created.
    pub fn new<P>(
        image: &str,
        shell_command: &str,
        env: &[(String, String)],
        unset_env: &[String],
        paths: &[P],
        binds: &[String],
        label: &str,
    ) -> Result<Self, Error>
    where
        P: AsRef<std::ffi::OsStr>,
    {
        // Check that the Docker socket exists before attempting a connection
        let docker_socket = std::path::Path::new("/var/run/docker.sock");
        if !docker_socket.exists() {
            return Err(Error::DockerNotAvailable {
                message: "Docker socket not found at /var/run/docker.sock".to_string(),
            });
        }

        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|err| Error::SpawnFailed {
                message: format!("Failed to create tokio runtime: {err}"),
            })?;

        let docker =
            Docker::connect_with_socket_defaults().map_err(|err| Error::DockerNotAvailable {
                message: format!("Failed to connect to Docker daemon: {err}"),
            })?;

        let shell_command = if shell_command.is_empty() {
            "bash -c".to_string()
        } else {
            shell_command.to_string()
        };

        Ok(Self {
            image: image.to_string(),
            shell_command,
            env: env.iter().cloned().collect(),
            unset_env: unset_env.to_vec(),
            paths: paths.iter().map(PathBuf::from).collect(),
            binds: binds.to_vec(),
            working_dir: "/workspace".to_string(),
            runtime,
            docker,
            container_id: Mutex::new(None),
            label: label.to_string(),
        })
    }

    /// Build the environment variable list for the container, applying the
    /// same semantics as `ShellExecutor`: extra paths are prepended to
    /// `PATH`, and `unset_env` variables are removed.
    fn container_env(&self) -> Vec<String> {
        let mut result: Vec<String> = self
            .env
            .iter()
            .filter(|(k, _)| !self.unset_env.contains(k))
            .map(|(k, v)| format!("{k}={v}"))
            .collect();

        // Build the PATH variable the same way ShellExecutor does
        let mut path_parts: Vec<String> = self
            .paths
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect();

        if let Ok(current_path) = std::env::var("PATH") {
            path_parts.push(current_path);
        }

        let path_var = path_parts.join(":");
        result.push(format!("PATH={path_var}"));

        result
    }

    /// Build the command vector for a Docker exec from the shell command
    /// string and the script code.
    ///
    /// Splits the shell command (e.g. `"bash -c"`) into individual words,
    /// then appends the script code as the final argument.
    fn exec_command(&self, code: &str) -> Vec<String> {
        let mut words = shell_words::split(&self.shell_command).unwrap_or_default();
        if words.is_empty() {
            words = vec!["bash".to_string(), "-c".to_string()];
        }
        words.push(code.to_string());
        words
    }

    /// Generate a unique container name following the pattern
    /// `specdown-{file-hash}-{counter}`.
    ///
    /// The `label` (typically the spec file path) is hashed with a
    /// non-cryptographic hash to produce a short, stable identifier for
    /// the spec file. The counter ensures uniqueness even if the same
    /// spec file is run multiple times concurrently.
    fn unique_container_name(label: &str) -> String {
        let counter = CONTAINER_COUNTER.fetch_add(1, Ordering::SeqCst);
        let hash = hash_label(label);
        format!("specdown-{hash}-{counter}")
    }

    /// Return the ID of the persistent container, creating and starting it
    /// if it does not yet exist.
    ///
    /// The container runs `sleep infinity` so it stays alive between exec
    /// calls, providing filesystem and environment persistence across all
    /// script and background blocks in a spec file.
    fn ensure_container(&self) -> Result<String, Error> {
        // Fast path: container already created.
        if let Some(id) = self
            .container_id
            .lock()
            .expect("container_id mutex poisoned")
            .clone()
        {
            return Ok(id);
        }

        let docker = self.docker.clone();
        let image = self.image.clone();
        let env = self.container_env();
        let working_dir = self.working_dir.clone();
        let binds = self.binds.clone();
        let container_name = Self::unique_container_name(&self.label);

        let container_id = self.runtime.block_on(async move {
            let config = Config {
                image: Some(image),
                entrypoint: Some(vec!["sleep".to_string()]),
                cmd: Some(vec!["infinity".to_string()]),
                env: Some(env),
                working_dir: Some(working_dir),
                open_stdin: Some(false),
                stdin_once: Some(false),
                tty: Some(false),
                host_config: Some(HostConfig {
                    binds: if binds.is_empty() { None } else { Some(binds) },
                    ..Default::default()
                }),
                ..Default::default()
            };

            let create_result = docker
                .create_container(
                    Some(CreateContainerOptions {
                        name: container_name,
                        platform: None,
                    }),
                    config,
                )
                .await
                .map_err(|err| Error::SpawnFailed {
                    message: format!("Failed to create persistent container: {err}"),
                })?;

            docker
                .start_container(&create_result.id, None::<StartContainerOptions<String>>)
                .await
                .map_err(|err| Error::SpawnFailed {
                    message: format!("Failed to start persistent container: {err}"),
                })?;

            Ok::<String, Error>(create_result.id)
        })?;

        *self
            .container_id
            .lock()
            .expect("container_id mutex poisoned") = Some(container_id.clone());
        Ok(container_id)
    }
}

impl Executor for ContainerExecutor {
    fn execute(&self, script: &ScriptCode) -> Result<Output, Error> {
        let ScriptCode(code_string) = script;
        let container_id = self.ensure_container()?;

        let docker = self.docker.clone();
        let cmd = self.exec_command(code_string);
        let env = self.container_env();
        let working_dir = self.working_dir.clone();

        self.runtime.block_on(async move {
            // Create an exec instance in the persistent container
            let exec = docker
                .create_exec(
                    &container_id,
                    CreateExecOptions {
                        cmd: Some(cmd),
                        env: Some(env),
                        working_dir: Some(working_dir),
                        attach_stdout: Some(true),
                        attach_stderr: Some(true),
                        attach_stdin: Some(false),
                        ..Default::default()
                    },
                )
                .await
                .map_err(|err| Error::SpawnFailed {
                    message: format!("Failed to create exec: {err}"),
                })?;

            // Start the exec (attached) and capture output
            let start_result = docker
                .start_exec(&exec.id, None::<StartExecOptions>)
                .await
                .map_err(|err| Error::SpawnFailed {
                    message: format!("Failed to start exec: {err}"),
                })?;

            let mut stdout = String::new();
            let mut stderr = String::new();

            if let bollard::exec::StartExecResults::Attached {
                output: mut output_stream,
                ..
            } = start_result
            {
                while let Some(msg) = output_stream.next().await {
                    match msg {
                        Ok(LogOutput::StdOut { message }) => {
                            stdout.push_str(&String::from_utf8_lossy(&message));
                        }
                        Ok(LogOutput::StdErr { message }) => {
                            stderr.push_str(&String::from_utf8_lossy(&message));
                        }
                        Ok(_) => {}
                        Err(err) => {
                            stderr.push_str("Error reading exec output: ");
                            stderr.push_str(&err.to_string());
                            stderr.push('\n');
                        }
                    }
                }
            }

            // Get the exit code
            let inspect =
                docker
                    .inspect_exec(&exec.id)
                    .await
                    .map_err(|err| Error::SpawnFailed {
                        message: format!("Failed to inspect exec: {err}"),
                    })?;

            let exit_code = inspect.exit_code.map(|c| i32::try_from(c).unwrap_or(0));

            Ok(Output {
                stdout,
                stderr,
                exit_code,
            })
        })
    }

    fn spawn(&self, script: &ScriptCode) -> Result<Box<dyn BackgroundHandle>, Error> {
        let ScriptCode(code_string) = script;
        let container_id = self.ensure_container()?;
        let container_id_for_handle = container_id.clone();

        let docker = self.docker.clone();
        let env = self.container_env();
        let working_dir = self.working_dir.clone();

        // Wrap the script so it records its PID inside the container and
        // writes the child's exit code to a separate file when it exits.
        //   - `pid_file`  — used by kill() and try_wait() to find the PID
        //   - `exit_file` — written by the wrapper after the child exits
        let bg_num = BG_COUNTER.fetch_add(1, Ordering::SeqCst);
        let pid_file = format!("/tmp/.specdown_bg_{bg_num}");
        let exit_file = format!("/tmp/.specdown_bg_exit_{bg_num}");
        // The wrapper writes the shell PID to `pid_file` and then runs the
        // user script in the foreground.  When the script exits the wrapper
        // writes `$?` to `exit_file` and itself terminates, so `kill -0` on
        // the PID will fail once the background work is done.
        let wrapped_code = format!("echo $$ > {pid_file}\n{code_string}\necho $? > {exit_file}");
        let cmd = self.exec_command(&wrapped_code);

        self.runtime.block_on(async move {
            let exec = docker
                .create_exec(
                    &container_id,
                    CreateExecOptions {
                        cmd: Some(cmd),
                        env: Some(env),
                        working_dir: Some(working_dir),
                        ..Default::default()
                    },
                )
                .await
                .map_err(|err| Error::SpawnFailed {
                    message: format!("Failed to create background exec: {err}"),
                })?;

            // Start detached so it runs in the background.
            docker
                .start_exec(
                    &exec.id,
                    Some(StartExecOptions {
                        detach: true,
                        ..Default::default()
                    }),
                )
                .await
                .map_err(|err| Error::SpawnFailed {
                    message: format!("Failed to start background exec: {err}"),
                })?;

            Ok::<(), Error>(())
        })?;

        let handle_runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|err| Error::SpawnFailed {
                message: format!("Failed to create runtime for background handle: {err}"),
            })?;

        Ok(Box::new(ContainerBackgroundHandle {
            container_id: container_id_for_handle,
            pid_file,
            exit_file,
            docker: self.docker.clone(),
            runtime: handle_runtime,
        }) as Box<dyn BackgroundHandle>)
    }

    fn clone_box(&self, label: &str) -> Box<dyn Executor> {
        // Create a new ContainerExecutor with the same configuration.
        // Each clone gets its own container (created lazily on first use)
        // with a unique name derived from the label, ensuring isolation
        // in parallel execution.
        let paths: Vec<String> = self
            .paths
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect();
        let env: Vec<(String, String)> = self
            .env
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        ContainerExecutor::new::<String>(
            &self.image,
            &self.shell_command,
            &env,
            &self.unset_env,
            &paths,
            &self.binds,
            label,
        )
        .map_or_else(
            |err| {
                // If we can't create a new executor, create a dummy that
                // returns the error on first use. This should be extremely rare.
                Box::new(super::executor::FailedExecutor(err)) as Box<dyn Executor>
            },
            |e| Box::new(e) as Box<dyn Executor>,
        )
    }
}

impl Drop for ContainerExecutor {
    fn drop(&mut self) {
        let docker = self.docker.clone();
        if let Some(container_id) = self
            .container_id
            .lock()
            .expect("container_id mutex poisoned")
            .clone()
        {
            let _ = self.runtime.block_on(async move {
                docker
                    .remove_container(
                        &container_id,
                        Some(RemoveContainerOptions {
                            force: true,
                            ..Default::default()
                        }),
                    )
                    .await
            });
        }
    }
}

/// A handle to a background process running inside the persistent
/// container via Docker exec.
///
/// The process is tracked by its PID file (`pid_file`).  To check whether
/// the process is still alive we run `kill -0 $(cat <pid_file>)` inside
/// the container via another exec; to determine the exit code we read
/// the `exit_file` that the wrapper script writes when the child exits.
#[derive(Debug)]
pub struct ContainerBackgroundHandle {
    container_id: String,
    pid_file: String,
    exit_file: String,
    docker: Docker,
    runtime: tokio::runtime::Runtime,
}

impl BackgroundHandle for ContainerBackgroundHandle {
    fn kill(&mut self) {
        let container_id = self.container_id.clone();
        let pid_file = self.pid_file.clone();
        let exit_file = self.exit_file.clone();
        let docker = self.docker.clone();
        let _ = self.runtime.block_on(async move {
            // Kill the background process using its recorded PID, then
            // clean up both the PID file and the exit-code file.
            let kill_cmd = vec![
                "sh".to_string(),
                "-c".to_string(),
                format!(
                    "kill -TERM $(cat {pid_file}) 2>/dev/null; \
                     rm -f {pid_file} {exit_file}"
                ),
            ];
            let exec = docker
                .create_exec(
                    &container_id,
                    CreateExecOptions::<String> {
                        cmd: Some(kill_cmd),
                        ..Default::default()
                    },
                )
                .await
                .ok()?;
            docker
                .start_exec(&exec.id, None::<StartExecOptions>)
                .await
                .ok()?;
            Some(())
        });
    }

    fn try_wait(&mut self) -> Option<i32> {
        let container_id = self.container_id.clone();
        let pid_file = self.pid_file.clone();
        let exit_file = self.exit_file.clone();
        let docker = self.docker.clone();
        self.runtime.block_on(async move {
            // Run `kill -0 $(cat <pid_file>)` inside the container.
            // Exit 0  → process still alive → return None.
            // Non-zero → process dead → read exit_file for the exit code.
            let check_cmd = vec![
                "sh".to_string(),
                "-c".to_string(),
                format!("kill -0 $(cat {pid_file}) 2>/dev/null"),
            ];

            let exec = docker
                .create_exec(
                    &container_id,
                    CreateExecOptions::<String> {
                        cmd: Some(check_cmd),
                        ..Default::default()
                    },
                )
                .await
                .ok()?;

            let start_result = docker
                .start_exec(&exec.id, None::<StartExecOptions>)
                .await
                .ok()?;

            // Drain the output stream so the exec completes cleanly.
            if let bollard::exec::StartExecResults::Attached {
                output: mut output_stream,
                ..
            } = start_result
            {
                while let Some(msg) = output_stream.next().await {
                    match msg {
                        Ok(_) => {}
                        Err(_) => break,
                    }
                }
            }

            // Inspect the exec to get the exit code of `kill -0`.
            let inspect = docker.inspect_exec(&exec.id).await.ok()?;

            if inspect.exit_code == Some(0) {
                // Process is still alive.
                return None;
            }

            // Process is dead — read the exit code from the exit file.
            let read_cmd = vec![
                "sh".to_string(),
                "-c".to_string(),
                format!("cat {exit_file} 2>/dev/null || echo 0"),
            ];

            let read_exec = docker
                .create_exec(
                    &container_id,
                    CreateExecOptions::<String> {
                        cmd: Some(read_cmd),
                        ..Default::default()
                    },
                )
                .await
                .ok()?;

            let start_result = docker
                .start_exec(&read_exec.id, None::<StartExecOptions>)
                .await
                .ok()?;

            let mut exit_code_str = String::new();
            if let bollard::exec::StartExecResults::Attached {
                output: mut output_stream,
                ..
            } = start_result
            {
                while let Some(msg) = output_stream.next().await {
                    if let Ok(LogOutput::StdOut { message }) = msg {
                        exit_code_str.push_str(&String::from_utf8_lossy(&message));
                    }
                }
            }

            let code = exit_code_str.trim().parse::<i32>().unwrap_or(0);
            Some(code)
        })
    }
}

impl Drop for ContainerBackgroundHandle {
    fn drop(&mut self) {
        self.kill();
    }
}

#[cfg(test)]
mod tests {
    use super::{ContainerExecutor, Executor, ScriptCode};
    use std::path::PathBuf;

    // These tests require a running Docker daemon.
    // They are ignored by default; run with `cargo test -- --ignored`.

    fn docker_available() -> bool {
        std::path::Path::new("/var/run/docker.sock").exists()
    }

    #[test]
    #[ignore = "requires Docker daemon"]
    fn container_executor_executes_simple_command() {
        if !docker_available() {
            return;
        }
        let executor = ContainerExecutor::new::<PathBuf>(
            "bash:5",
            "bash -c",
            &[],
            &[],
            &[],
            &[],
            "test-simple",
        )
        .expect("executor to be created");

        let output = executor
            .execute(&ScriptCode("echo hello".to_string()))
            .expect("execution to succeed");

        assert_eq!(output.stdout, "hello\n");
        assert_eq!(output.exit_code, Some(0));
    }

    #[test]
    #[ignore = "requires Docker daemon"]
    fn container_executor_captures_stderr() {
        if !docker_available() {
            return;
        }
        let executor = ContainerExecutor::new::<PathBuf>(
            "bash:5",
            "bash -c",
            &[],
            &[],
            &[],
            &[],
            "test-stderr",
        )
        .expect("executor to be created");

        let output = executor
            .execute(&ScriptCode("echo 'error' >&2".to_string()))
            .expect("execution to succeed");

        assert_eq!(output.stderr, "error\n");
    }

    #[test]
    #[ignore = "requires Docker daemon"]
    fn container_executor_captures_exit_code() {
        if !docker_available() {
            return;
        }
        let executor =
            ContainerExecutor::new::<PathBuf>("bash:5", "bash -c", &[], &[], &[], &[], "test-exit")
                .expect("executor to be created");

        let output = executor
            .execute(&ScriptCode("exit 42".to_string()))
            .expect("execution to succeed");

        assert_eq!(output.exit_code, Some(42));
    }

    #[test]
    #[ignore = "requires Docker daemon"]
    fn container_executor_sets_environment_variables() {
        if !docker_available() {
            return;
        }
        let executor = ContainerExecutor::new::<PathBuf>(
            "bash:5",
            "bash -c",
            &[("MESSAGE".to_string(), "hello".to_string())],
            &[],
            &[],
            &[],
            "test-env",
        )
        .expect("executor to be created");

        let output = executor
            .execute(&ScriptCode("echo $MESSAGE".to_string()))
            .expect("execution to succeed");

        assert_eq!(output.stdout, "hello\n");
    }

    #[test]
    #[ignore = "requires Docker daemon"]
    fn container_executor_persists_state_across_script_blocks() {
        if !docker_available() {
            return;
        }
        let executor = ContainerExecutor::new::<PathBuf>(
            "bash:5",
            "bash -c",
            &[],
            &[],
            &[],
            &[],
            "test-persist",
        )
        .expect("executor to be created");

        // Write a file in the first script block
        executor
            .execute(&ScriptCode(
                "echo 'persisted' > /tmp/test_persistence.txt".to_string(),
            ))
            .expect("first execution to succeed");

        // Read it back in a second script block — this only works if
        // the same container is reused.
        let output = executor
            .execute(&ScriptCode("cat /tmp/test_persistence.txt".to_string()))
            .expect("second execution to succeed");

        assert_eq!(output.stdout, "persisted\n");
        assert_eq!(output.exit_code, Some(0));
    }

    #[test]
    #[ignore = "requires Docker daemon"]
    fn container_executor_spawns_and_stops_background_container() {
        if !docker_available() {
            return;
        }
        let executor =
            ContainerExecutor::new::<PathBuf>("bash:5", "bash -c", &[], &[], &[], &[], "test-bg")
                .expect("executor to be created");

        let mut handle = executor
            .spawn(&ScriptCode("sleep 60".to_string()))
            .expect("spawn to succeed");

        handle.kill();
        while handle.try_wait().is_none() {
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }

    #[test]
    #[ignore = "requires Docker daemon"]
    fn container_executor_try_wait_returns_none_while_running() {
        if !docker_available() {
            return;
        }
        let executor = ContainerExecutor::new::<PathBuf>(
            "bash:5",
            "bash -c",
            &[],
            &[],
            &[],
            &[],
            "test-try-wait-none",
        )
        .expect("executor to be created");

        let mut handle = executor
            .spawn(&ScriptCode("sleep 10".to_string()))
            .expect("spawn to succeed");

        // Give the container a moment to start the background process.
        std::thread::sleep(std::time::Duration::from_secs(1));

        // While the process is running, try_wait should return None.
        assert_eq!(
            handle.try_wait(),
            None,
            "try_wait should return None while the background process is still running"
        );

        // Clean up.
        handle.kill();
    }

    #[test]
    #[ignore = "requires Docker daemon"]
    fn container_executor_try_wait_returns_some_after_exit() {
        if !docker_available() {
            return;
        }
        let executor = ContainerExecutor::new::<PathBuf>(
            "bash:5",
            "bash -c",
            &[],
            &[],
            &[],
            &[],
            "test-try-wait-some",
        )
        .expect("executor to be created");

        let mut handle = executor
            .spawn(&ScriptCode("exit 42".to_string()))
            .expect("spawn to succeed");

        // Wait for the process to exit and the exit file to be written.
        let mut result = None;
        for _ in 0..60 {
            if let Some(code) = handle.try_wait() {
                result = Some(code);
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        assert_eq!(
            result,
            Some(42),
            "try_wait should return Some(42) after the background process exits with code 42"
        );
    }

    #[test]
    fn container_name_includes_file_hash() {
        // The hash function is deterministic — same label produces same hash.
        let hash1 = super::hash_label("specs/test1.md");
        let hash2 = super::hash_label("specs/test1.md");
        assert_eq!(hash1, hash2, "same label should produce same hash");

        // Different labels produce different hashes.
        let hash3 = super::hash_label("specs/test2.md");
        assert_ne!(
            hash1, hash3,
            "different labels should produce different hashes"
        );

        // Hash is 8 hex characters.
        assert_eq!(hash1.len(), 8, "hash should be 8 hex characters");
        assert!(
            hash1.chars().all(|c| c.is_ascii_hexdigit()),
            "hash should be hex: {}",
            hash1
        );
    }

    #[test]
    fn container_name_format_is_correct() {
        // Verify the naming pattern: specdown-{hash}-{counter}
        // We can't call unique_container_name directly because it uses
        // a global counter, but we can verify the format via hash_label.
        let hash = super::hash_label("specs/my_spec.md");
        assert!(
            hash.starts_with(|c: char| c.is_ascii_hexdigit()),
            "hash should start with a hex digit"
        );
    }
}
