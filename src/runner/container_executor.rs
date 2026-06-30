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

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

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
    container_id: std::cell::RefCell<Option<String>>,
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
            container_id: std::cell::RefCell::new(None),
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

    /// Return the ID of the persistent container, creating and starting it
    /// if it does not yet exist.
    ///
    /// The container runs `sleep infinity` so it stays alive between exec
    /// calls, providing filesystem and environment persistence across all
    /// script and background blocks in a spec file.
    fn ensure_container(&self) -> Result<String, Error> {
        // Fast path: container already created.
        if let Some(id) = self.container_id.borrow().clone() {
            return Ok(id);
        }

        let docker = self.docker.clone();
        let image = self.image.clone();
        let env = self.container_env();
        let working_dir = self.working_dir.clone();
        let binds = self.binds.clone();

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
                .create_container(None::<CreateContainerOptions<String>>, config)
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

        *self.container_id.borrow_mut() = Some(container_id.clone());
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

        // Wrap the script so it records its PID inside the container.
        // This lets us kill the background process later via another exec.
        let bg_num = BG_COUNTER.fetch_add(1, Ordering::SeqCst);
        let pid_file = format!("/tmp/.specdown_bg_{bg_num}");
        let wrapped_code = format!("echo $$ > {pid_file}; exec {code_string}");
        let cmd = self.exec_command(&wrapped_code);

        let exec_id = self.runtime.block_on(async move {
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

            // Start detached so it runs in the background
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

            Ok::<String, Error>(exec.id)
        })?;

        let handle_runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|err| Error::SpawnFailed {
                message: format!("Failed to create runtime for background handle: {err}"),
            })?;

        Ok(Box::new(ContainerBackgroundHandle {
            container_id: container_id_for_handle,
            exec_id,
            pid_file,
            docker: self.docker.clone(),
            runtime: handle_runtime,
        }) as Box<dyn BackgroundHandle>)
    }
}

impl Drop for ContainerExecutor {
    fn drop(&mut self) {
        let docker = self.docker.clone();
        if let Some(container_id) = self.container_id.borrow().clone() {
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
/// The process is tracked by its exec ID. To kill it, we read the PID
/// (recorded at spawn time) and send `SIGTERM` from another exec.
#[derive(Debug)]
pub struct ContainerBackgroundHandle {
    container_id: String,
    exec_id: String,
    pid_file: String,
    docker: Docker,
    runtime: tokio::runtime::Runtime,
}

impl BackgroundHandle for ContainerBackgroundHandle {
    fn kill(&mut self) {
        let container_id = self.container_id.clone();
        let pid_file = self.pid_file.clone();
        let docker = self.docker.clone();
        let _ = self.runtime.block_on(async move {
            // Kill the background process using its recorded PID, then
            // clean up the PID file.
            let kill_cmd = vec![
                "sh".to_string(),
                "-c".to_string(),
                format!("kill -TERM $(cat {pid_file}) 2>/dev/null; rm -f {pid_file}"),
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

    fn wait(&mut self) -> Option<i32> {
        let exec_id = self.exec_id.clone();
        let docker = self.docker.clone();
        self.runtime.block_on(async move {
            loop {
                match docker.inspect_exec(&exec_id).await {
                    Ok(info) => {
                        if info.running != Some(true) {
                            return info.exit_code.map(|c| i32::try_from(c).unwrap_or(0));
                        }
                    }
                    Err(_) => return None,
                }
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        })
    }

    fn try_wait(&mut self) -> Option<i32> {
        let exec_id = self.exec_id.clone();
        let docker = self.docker.clone();
        self.runtime.block_on(async move {
            match docker.inspect_exec(&exec_id).await {
                Ok(info) => {
                    if info.running == Some(true) {
                        None
                    } else {
                        info.exit_code.map(|c| i32::try_from(c).unwrap_or(0))
                    }
                }
                Err(_) => None,
            }
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
        let executor = ContainerExecutor::new::<PathBuf>("bash:5", "bash -c", &[], &[], &[], &[])
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
        let executor = ContainerExecutor::new::<PathBuf>("bash:5", "bash -c", &[], &[], &[], &[])
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
        let executor = ContainerExecutor::new::<PathBuf>("bash:5", "bash -c", &[], &[], &[], &[])
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
        let executor = ContainerExecutor::new::<PathBuf>("bash:5", "bash -c", &[], &[], &[], &[])
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
        let executor = ContainerExecutor::new::<PathBuf>("bash:5", "bash -c", &[], &[], &[], &[])
            .expect("executor to be created");

        let mut handle = executor
            .spawn(&ScriptCode("sleep 60".to_string()))
            .expect("spawn to succeed");

        handle.kill();
        let _ = handle.wait();
    }
}
