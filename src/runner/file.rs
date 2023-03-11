use std::fs::File;
use std::io::Write;

use crate::results::{ActionResult, CreateFileResult};
use crate::types::{CreateFileAction, FileContent, FilePath};

pub fn run(action: &CreateFileAction) -> ActionResult {
    let CreateFileAction {
        file_path: FilePath(path_string),
        file_content: FileContent(content_string),
    } = action;

    // TODO: Nice error handling
    let mut file = File::create(path_string).expect("Failed to create file");
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

    #[test]
    fn test_run_creates_a_file() {
        fs::create_dir_all(".tests").expect("Failed to create test directory");

        let file_path = ".tests/test_file1.txt";
        fs::remove_file(file_path).ok();

        let action = CreateFileAction {
            file_path: FilePath(file_path.to_string()),
            file_content: FileContent("example content".to_string()),
        };

        run(&action);

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
        let result = run(&action);

        assert_eq!(
            result,
            ActionResult::CreateFile(CreateFileResult { action })
        );
    }
}
