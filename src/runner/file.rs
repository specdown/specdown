use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::results::{ActionResult, CreateFileResult};
use crate::types::{CreateFileAction, FileContent, FilePath};

pub fn run(action: &CreateFileAction, working_dir: &Path) -> ActionResult {
    let CreateFileAction {
        file_path: FilePath(path_string),
        file_content: FileContent(content_string),
    } = action;

    // TODO: Nice error handling
    let mut file = File::create(working_dir.join(path_string)).expect("Failed to create file");
    write!(file, "{content_string}").expect("Failed to write to file");
    ActionResult::CreateFile(CreateFileResult {
        action: action.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::{run, ActionResult, FileContent, FilePath};
    use crate::results::CreateFileResult;
    use crate::types::CreateFileAction;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_run_creates_a_file() {
        fs::create_dir_all(".tests").expect("Failed to create test directory");

        let file_path = ".tests/test_file1.txt";
        fs::remove_file(file_path).ok();

        let action = CreateFileAction {
            file_path: FilePath(file_path.to_string()),
            file_content: FileContent("example content".to_string()),
        };

        run(&action, Path::new("."));

        fs::read_to_string(file_path).map_or_else(
            |_| {
                panic!("File could not be read");
            },
            |content| {
                assert_eq!(content, "example content");
            },
        );

        fs::remove_file(file_path).expect("Failed to delete file");
    }

    #[test]
    fn test_run_returns_a_file_result() {
        fs::create_dir_all(".tests").expect("Failed to create test directory");

        let file_path = ".tests/test_file2.txt";
        fs::remove_file(file_path).ok();

        let action = CreateFileAction {
            file_path: FilePath(file_path.to_string()),
            file_content: FileContent("example content".to_string()),
        };
        let result = run(&action, Path::new("."));

        assert_eq!(
            result,
            ActionResult::CreateFile(CreateFileResult { action })
        );
    }

    #[test]
    fn test_run_resolves_relative_paths_against_working_dir() {
        let dir = tempfile::tempdir().expect("failed to create temp dir");

        let action = CreateFileAction {
            file_path: FilePath("nested/test_file3.txt".to_string()),
            file_content: FileContent("example content".to_string()),
        };

        fs::create_dir_all(dir.path().join("nested")).expect("failed to create nested dir");
        run(&action, dir.path());

        let content = fs::read_to_string(dir.path().join("nested/test_file3.txt"))
            .expect("file should have been created inside working_dir");
        assert_eq!(content, "example content");
    }
}
