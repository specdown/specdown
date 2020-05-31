use std::fs::File;
use std::io::Write;

use super::error;
use crate::results::test_result::TestResult;
use crate::types::{FileContent, FilePath};

pub fn run(path: &FilePath, content: &FileContent) -> Result<TestResult, error::Error> {
    let FilePath(path_string) = path;
    let FileContent(content_string) = content;

    // TODO: Nice error handling
    let mut file = File::create(path_string).expect("Failed to create file");
    write!(file, "{}", content_string).expect("Failed to write to file");
    Ok(TestResult::File {
        path: path_string.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_run_creates_a_file() {
        let file_path = "test_file.txt";
        fs::remove_file(file_path).ok();

        run(
            &FilePath(file_path.to_string()),
            &FileContent("example content".to_string()),
        )
        .expect("Run failed");

        if let Ok(content) = fs::read_to_string(file_path) {
            assert_eq!(content, "example content");
        } else {
            panic!("File could not be read");
        }

        fs::remove_file(file_path).expect("Failed to delete file");
    }

    #[test]
    fn test_run_returns_a_file_result() {
        let file_path = "test_file.txt";
        fs::remove_file(file_path).ok();
        let result = run(
            &FilePath(file_path.to_string()),
            &FileContent("example content".to_string()),
        );
        assert_eq!(
            result,
            Ok(TestResult::File {
                path: "test_file.txt".to_string()
            })
        );
    }
}
