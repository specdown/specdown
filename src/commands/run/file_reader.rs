use std::fs;
use std::path::{Path, PathBuf};

pub struct FileReader {
    directory: PathBuf,
}

impl FileReader {
    pub const fn new(directory: PathBuf) -> Self {
        Self { directory }
    }

    pub fn read_file(&self, spec_file: &Path) -> String {
        fs::read_to_string(self.to_absolute(spec_file)).expect("failed to read spec file")
    }

    fn to_absolute(&self, path: &Path) -> PathBuf {
        if path.has_root() {
            path.to_path_buf()
        } else {
            self.directory.join(path)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FileReader;

    mod to_absolute {
        use super::FileReader;
        use std::fs::File;
        use std::io::Write;
        use std::path::Path;

        #[test]
        fn test_reads_a_file_when_the_path_is_absolute() {
            let directory = tempfile::tempdir().expect("Failed to create a temporary directory");
            let full_path = directory.path().join("example.txt");
            File::create(full_path.clone())
                .and_then(|mut file| file.write_all(b"example content"))
                .unwrap_or_else(|err| panic!("Failed to write file: {}", err));

            let reader = FileReader::new("/home".into());
            let content = reader.read_file(&full_path);
            assert_eq!("example content", content);
        }

        #[test]
        fn test_reads_a_file_when_the_path_is_relative() {
            let directory = tempfile::tempdir().expect("Failed to create a temporary directory");
            let full_path = directory.path().join("example.txt");
            File::create(full_path)
                .and_then(|mut file| file.write_all(b"example content"))
                .unwrap_or_else(|err| panic!("Failed to write file: {}", err));

            let reader = FileReader::new(directory.path().to_path_buf());
            let content = reader.read_file(Path::new("example.txt"));
            assert_eq!("example content", content);
        }
    }
}
