use std::path::PathBuf;

pub trait Workspace {
    fn initialize(&mut self);
    fn dir(&self) -> &PathBuf;
}
