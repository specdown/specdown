use super::Workspace;
use std::path::PathBuf;

pub struct TemporaryDirectory {
    directory: Option<PathBuf>,
}

impl TemporaryDirectory {
    pub const fn create() -> Self {
        Self { directory: None }
    }
}

impl Workspace for TemporaryDirectory {
    fn initialize(&mut self) {
        let workspace_dir = tempfile::tempdir()
            .expect("Failed to create temporary workspace directory")
            .path()
            .to_path_buf();

        std::fs::create_dir_all(&workspace_dir).expect("Failed to create workspace directory");
        self.directory = Some(workspace_dir);
    }

    fn dir(&self) -> &PathBuf {
        self.directory
            .as_ref()
            .expect("Temporary workspace has not been initialized")
    }
}
