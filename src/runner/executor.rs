use std::process::Command;

use crate::types::ScriptCode;

use super::error::Error;
use shell_words::ParseError;

pub struct Output {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

pub trait Executor {
    fn execute(&self, script: &ScriptCode) -> Result<Output, Error>;
}

#[derive(Debug, PartialEq)]
pub struct Shell {
    command: String,
    args: Vec<String>,
}

impl Shell {
    pub fn new(shell_command: &str) -> Result<Self, Error> {
        shell_words::split(shell_command)
            .map_err(|err| Shell::parse_error_to_error(shell_command, err))
            .and_then(|words| Shell::check_is_not_empty(shell_command, &words))
            .map(|words| Shell::create_shell_instance(&words))
    }

    fn create_shell_instance(words: &[String]) -> Shell {
        let (command, args) = words.split_at(1);
        Self {
            command: command.first().unwrap().to_string(),
            args: Vec::from(args),
        }
    }

    fn parse_error_to_error(shell_command: &str, err: ParseError) -> Error {
        Error::BadShellCommand {
            command: shell_command.to_string(),
            message: format!("Parse error : {}", err),
        }
    }

    fn check_is_not_empty(shell_command: &str, words: &[String]) -> Result<Vec<String>, Error> {
        if words.is_empty() {
            Err(Error::BadShellCommand {
                command: shell_command.to_string(),
                message: "Command is empty".to_string(),
            })
        } else {
            Ok(words.to_vec())
        }
    }
}

impl Executor for Shell {
    fn execute(&self, script: &ScriptCode) -> Result<Output, Error> {
        let ScriptCode(code_string) = script;

        let command_result = Command::new(&self.command)
            .args(&self.args)
            .arg(code_string)
            .output();

        match command_result {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let exit_code = output.status.code();

                Ok(Output {
                    stdout,
                    stderr,
                    exit_code,
                })
            }
            Err(err) => Err(Error::CommandFailed {
                command: format!("{} {:?}", self.command, self.args),
                message: err.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Executor, Shell};

    mod shell {
        use crate::runner::Error;
        use crate::types::ScriptCode;

        use super::{Executor, Shell};

        #[test]
        fn new_with_command_with_arguments() {
            let shell = Shell::new("bash -c").expect("shell to be created");
            let output = shell
                .execute(&ScriptCode("echo $0".to_string()))
                .expect("success");
            assert_eq!(output.stdout, "bash\n");
        }

        #[test]
        fn new_with_command_without_arguments() {
            let shell = Shell::new("echo").expect("shell to be created");
            let output = shell
                .execute(&ScriptCode("hello".to_string()))
                .expect("success");
            assert_eq!(output.stdout, "hello\n");
        }

        #[test]
        fn new_with_empty_command_string() {
            assert_eq!(
                Shell::new(""),
                Err(Error::BadShellCommand {
                    command: "".to_string(),
                    message: "Command is empty".to_string(),
                })
            );
        }

        #[test]
        fn new_invalid_command() {
            assert_eq!(
                Shell::new("broken \" command"),
                Err(Error::BadShellCommand {
                    command: "broken \" command".to_string(),
                    message: "Parse error : missing closing quote".to_string(),
                })
            );
        }
    }
}
