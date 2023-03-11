use super::Workspace;
use std::path::PathBuf;

pub struct ExistingDir {
    directory: PathBuf,
}

impl ExistingDir {
    pub const fn create(directory: PathBuf) -> Self {
        Self { directory }
    }
}

impl Workspace for ExistingDir {
    fn initialize(&mut self) {
        std::fs::create_dir_all(&self.directory).expect("Failed to create workspace directory");

        self.directory = std::fs::canonicalize(&self.directory)
            .unwrap_or_else(|_| panic!("Failed to canonicalize {:?}", &self.directory));
    }

    fn dir(&self) -> &PathBuf {
        &self.directory
    }
}
