use std::path::Path;

use crate::runner::shell_executor::ShellExecutor;
use crate::runner::{Error, Executor};

/// Builds a fresh, fully-configured `Executor` instance.
///
/// `label` uniquely identifies the spec file (or `"main"` for the
/// shared-workspace path) and is used by the container executor to derive a
/// unique container name. `extra_env` is appended on top of the factory's
/// base environment — this is how the per-invocation `SPECDOWN_*` variables
/// (which differ between the shared run and each per-spec workspace) get
/// injected without rebuilding the whole factory. `working_dir` is the host
/// directory script actions should run in.
pub trait ExecutorFactory: Send + Sync {
    fn build(
        &self,
        label: &str,
        extra_env: &[(String, String)],
        working_dir: &Path,
    ) -> Result<Box<dyn Executor>, Error>;
}

pub struct ShellExecutorFactory {
    pub shell_cmd: String,
    pub base_env: Vec<(String, String)>,
    pub unset_env: Vec<String>,
    pub paths: Vec<String>,
}

impl ExecutorFactory for ShellExecutorFactory {
    fn build(
        &self,
        _label: &str,
        extra_env: &[(String, String)],
        working_dir: &Path,
    ) -> Result<Box<dyn Executor>, Error> {
        let mut env = self.base_env.clone();
        env.extend_from_slice(extra_env);
        ShellExecutor::new(&self.shell_cmd, &env, &self.unset_env, &self.paths)
            .map(|e| Box::new(e.with_working_dir(working_dir.to_path_buf())) as Box<dyn Executor>)
    }
}

#[cfg(feature = "container")]
pub struct ContainerExecutorFactory {
    pub image: String,
    pub shell_cmd: String,
    pub base_env: Vec<(String, String)>,
    pub unset_env: Vec<String>,
    pub paths: Vec<String>,
    pub container_volumes: Vec<String>,
}

#[cfg(feature = "container")]
impl ExecutorFactory for ContainerExecutorFactory {
    fn build(
        &self,
        label: &str,
        extra_env: &[(String, String)],
        _working_dir: &Path,
    ) -> Result<Box<dyn Executor>, Error> {
        let mut env = self.base_env.clone();
        env.extend_from_slice(extra_env);
        crate::runner::container_executor::ContainerExecutor::new::<String>(
            &self.image,
            &self.shell_cmd,
            &env,
            &self.unset_env,
            &self.paths,
            &self.container_volumes,
            label,
        )
        .map(|e| Box::new(e) as Box<dyn Executor>)
    }
}
