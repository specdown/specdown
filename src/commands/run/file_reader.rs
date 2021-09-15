use std::fs;
use std::path::{Path, PathBuf};

pub struct FileReader {
    dir: PathBuf,
}

impl FileReader {
    pub fn create(directory: PathBuf) -> Self {
        FileReader { dir: directory }
    }

    pub fn read_file(&self, spec_file: &Path) -> String {
        fs::read_to_string(self.to_absolute(spec_file)).expect("failed to read spec file")
    }

    pub fn to_absolute(&self, path: &Path) -> PathBuf {
        if path.has_root() {
            path.to_path_buf()
        } else {
            self.dir.join(path)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FileReader;

    mod to_absolute {
        use super::FileReader;
        use std::path::Path;

        fn reader() -> FileReader {
            FileReader {
                dir: Path::new("/usr/local/specdown").to_path_buf(),
            }
        }

        #[cfg(not(windows))]
        #[test]
        fn test_returns_the_path_when_it_is_absolute() {
            let path = Path::new("/home/user/file");
            assert_eq!(path, reader().to_absolute(path));
        }

        #[cfg(not(windows))]
        #[test]
        fn test_returns_the_working_dir_prepended_when_path_is_relative() {
            let path = Path::new("./file");
            assert_eq!(
                Path::new("/usr/local/specdown/file"),
                reader().to_absolute(path)
            );
        }

        #[cfg(not(windows))]
        #[test]
        fn test_returns_the_working_dir_prepended_when_path_contains_parent() {
            let path = Path::new("../file");
            assert_eq!(
                Path::new("/usr/local/specdown/../file"),
                reader().to_absolute(path)
            );
        }
    }
}
