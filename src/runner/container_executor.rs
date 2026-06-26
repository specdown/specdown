//! Container executor that runs spec scripts inside a Docker container
//! using the Docker Engine API via the `bollard` crate.
//!
//! This executor connects directly to the Docker socket
//! (`/var/run/docker.sock` on Linux) and manages containers through the
//! API — it does NOT shell out to `docker run`.

use std::collections::HashMap;
use std::path::PathBuf;

use bollard::container::{
    AttachContainerOptions, AttachContainerResults, Config, CreateContainerOptions, LogOutput,
    RemoveContainerOptions,
};
use bollard::Docker;

use crate::types::ScriptCode;

use super::background_handle::BackgroundHandle;
use super::executor::Output;
use super::{Error, Executor};
use futures_util::StreamExt;
use std::convert::TryFrom;

/// An executor that runs scripts inside a Docker container, communicating
/// with the Docker daemon over its socket API.
///
/// Created with [`ContainerExecutor::new`], which establishes a connection
/// to the Docker daemon.
#[derive(Debug)]
pub struct ContainerExecutor {
    image: String,
    shell_command: String,
    env: HashMap<String, String>,
    unset_env: Vec<String>,
    paths: Vec<PathBuf>,
    runtime: tokio::runtime::Runtime,
    docker: Docker,
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
            runtime,
            docker,
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

    /// Build the container command from the shell command string.
    ///
    /// Splits the shell command (e.g. `"bash -c"`) into entrypoint + cmd
    /// parts, then appends the script code as the final argument.
    fn container_command(&self, code: &str) -> (Vec<String>, Vec<String>) {
        let words = shell_words::split(&self.shell_command).unwrap_or_default();
        if words.is_empty() {
            return (
                vec!["bash".to_string()],
                vec!["-c".to_string(), code.to_string()],
            );
        }

        let (entrypoint, cmd) = words.split_at(1);
        let mut cmd = cmd.to_vec();
        cmd.push(code.to_string());

        (entrypoint.to_vec(), cmd)
    }

    /// Build a `Config` for the Docker container.
    fn container_config(&self, code: &str) -> Config<String> {
        let (entrypoint, cmd) = self.container_command(code);
        let env = self.container_env();

        Config {
            image: Some(self.image.clone()),
            entrypoint: Some(entrypoint),
            cmd: Some(cmd),
            env: Some(env),
            attach_stdout: Some(true),
            attach_stderr: Some(true),
            open_stdin: Some(false),
            stdin_once: Some(false),
            tty: Some(false),
            ..Default::default()
        }
    }
}

impl Executor for ContainerExecutor {
    fn execute(&self, script: &ScriptCode) -> Result<Output, Error> {
        let ScriptCode(code_string) = script;

        let config = self.container_config(code_string);

        let docker = self.docker.clone();

        self.runtime.block_on(async move {
            // Create the container
            let create_result = docker
                .create_container(None::<CreateContainerOptions<String>>, config)
                .await
                .map_err(|err| Error::SpawnFailed {
                    message: format!("Failed to create container: {err}"),
                })?;

            let container_id = create_result.id;

            // Attach to the container to capture stdout/stderr
            let attach_options = Some(AttachContainerOptions::<String> {
                stdin: Some(false),
                stdout: Some(true),
                stderr: Some(true),
                stream: Some(true),
                logs: Some(false),
                detach_keys: None,
            });

            let AttachContainerResults { output, .. } = docker
                .attach_container(&container_id, attach_options)
                .await
                .map_err(|err| Error::SpawnFailed {
                    message: format!("Failed to attach to container: {err}"),
                })?;

            // Start the container
            docker
                .start_container(
                    &container_id,
                    None::<bollard::container::StartContainerOptions<String>>,
                )
                .await
                .map_err(|err| Error::SpawnFailed {
                    message: format!("Failed to start container: {err}"),
                })?;

            // Collect stdout and stderr from the attach stream
            let mut stdout = String::new();
            let mut stderr = String::new();

            let mut output = output;
            while let Some(msg) = output.next().await {
                match msg {
                    Ok(LogOutput::StdOut { message }) => {
                        stdout.push_str(&String::from_utf8_lossy(&message));
                    }
                    Ok(LogOutput::StdErr { message }) => {
                        stderr.push_str(&String::from_utf8_lossy(&message));
                    }
                    Ok(_) => {}
                    Err(err) => {
                        stderr.push_str("Error reading container output: ");
                        stderr.push_str(&err.to_string());
                        stderr.push('\n');
                    }
                }
            }

            // Wait for the container to exit and get the exit code
            let wait_stream = docker.wait_container(
                &container_id,
                None::<bollard::container::WaitContainerOptions<String>>,
            );
            let exit_code = {
                futures_util::pin_mut!(wait_stream);
                let mut code = None;
                while let Some(msg) = wait_stream.next().await {
                    if let Ok(response) = msg {
                        code = Some(i32::try_from(response.status_code).unwrap_or(0));
                        break;
                    }
                }
                code
            };

            // Remove the container (force, in case it's still running)
            let _ = docker
                .remove_container(
                    &container_id,
                    Some(RemoveContainerOptions {
                        force: true,
                        ..Default::default()
                    }),
                )
                .await;

            Ok(Output {
                stdout,
                stderr,
                exit_code,
            })
        })
    }

    fn spawn(&self, script: &ScriptCode) -> Result<Box<dyn BackgroundHandle>, Error> {
        let ScriptCode(code_string) = script;

        let config = self.container_config(code_string);

        let docker = self.docker.clone();

        self.runtime.block_on(async move {
            // Create the container
            let create_result = docker
                .create_container(None::<CreateContainerOptions<String>>, config)
                .await
                .map_err(|err| Error::SpawnFailed {
                    message: format!("Failed to create container: {err}"),
                })?;

            let container_id = create_result.id;

            // Start the container
            docker
                .start_container(
                    &container_id,
                    None::<bollard::container::StartContainerOptions<String>>,
                )
                .await
                .map_err(|err| Error::SpawnFailed {
                    message: format!("Failed to start container: {err}"),
                })?;

            Ok(Box::new(ContainerBackgroundHandle {
                container_id,
                docker,
                runtime: tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .map_err(|err| Error::SpawnFailed {
                        message: format!("Failed to create runtime for background handle: {err}"),
                    })?,
            }) as Box<dyn BackgroundHandle>)
        })
    }
}

/// A handle to a background Docker container that can be killed and waited on.
#[derive(Debug)]
pub struct ContainerBackgroundHandle {
    container_id: String,
    docker: Docker,
    runtime: tokio::runtime::Runtime,
}

impl BackgroundHandle for ContainerBackgroundHandle {
    fn kill(&mut self) {
        let container_id = self.container_id.clone();
        let docker = self.docker.clone();
        let _ = self.runtime.block_on(async move {
            docker
                .kill_container(
                    &container_id,
                    None::<bollard::container::KillContainerOptions<String>>,
                )
                .await
        });
    }

    fn wait(&mut self) -> Option<i32> {
        let container_id = self.container_id.clone();
        let docker = self.docker.clone();
        self.runtime.block_on(async move {
            use futures_util::StreamExt;
            let mut stream = docker.wait_container(
                &container_id,
                None::<bollard::container::WaitContainerOptions<String>>,
            );
            stream
                .next()
                .await
                .and_then(std::result::Result::ok)
                .map(|r| i32::try_from(r.status_code).unwrap_or(0))
        })
        // Container may have already been removed; return None
    }

    fn try_wait(&mut self) -> Option<i32> {
        let container_id = self.container_id.clone();
        let docker = self.docker.clone();
        self.runtime.block_on(async move {
            // Inspect the container to check if it has exited
            match docker.inspect_container(&container_id, None).await {
                Ok(info) => {
                    let state = info.state?;
                    match state.status {
                        Some(bollard::models::ContainerStateStatusEnum::EXITED) => {
                            state.exit_code.map(|c| i32::try_from(c).unwrap_or(0))
                        }
                        _ => None,
                    }
                }
                Err(_) => None,
            }
        })
    }
}

impl Drop for ContainerBackgroundHandle {
    fn drop(&mut self) {
        let container_id = self.container_id.clone();
        let docker = self.docker.clone();
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
        let executor = ContainerExecutor::new::<PathBuf>("bash:5", "bash -c", &[], &[], &[])
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
        let executor = ContainerExecutor::new::<PathBuf>("bash:5", "bash -c", &[], &[], &[])
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
        let executor = ContainerExecutor::new::<PathBuf>("bash:5", "bash -c", &[], &[], &[])
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
        )
        .expect("executor to be created");

        let output = executor
            .execute(&ScriptCode("echo $MESSAGE".to_string()))
            .expect("execution to succeed");

        assert_eq!(output.stdout, "hello\n");
    }

    #[test]
    #[ignore = "requires Docker daemon"]
    fn container_executor_spawns_and_stops_background_container() {
        if !docker_available() {
            return;
        }
        let executor = ContainerExecutor::new::<PathBuf>("bash:5", "bash -c", &[], &[], &[])
            .expect("executor to be created");

        let mut handle = executor
            .spawn(&ScriptCode("sleep 60".to_string()))
            .expect("spawn to succeed");

        handle.kill();
        let _ = handle.wait();
    }
}
