use std::collections::HashMap;
use std::process::Command;

use shell_words::ParseError;

use crate::types::ScriptCode;

use super::error::Error;
use crate::runner::executor::Output;
use crate::runner::Executor;

#[derive(Debug, PartialEq)]
pub struct ShellExecutor {
    command: String,
    args: Vec<String>,
    env: HashMap<String, String>,
}

impl ShellExecutor {
    pub fn new(shell_command: &str, env: &[(String, String)]) -> Result<Self, Error> {
        shell_words::split(shell_command)
            .map_err(|err| ShellExecutor::parse_error_to_error(shell_command, err))
            .and_then(|words| ShellExecutor::check_is_not_empty(shell_command, &words))
            .map(|words| ShellExecutor::create_shell_instance(&words, env))
    }

    fn create_shell_instance(words: &[String], env: &[(String, String)]) -> ShellExecutor {
        let (command, args) = words.split_at(1);
        Self {
            command: command.first().unwrap().to_string(),
            args: Vec::from(args),
            env: env.to_vec().into_iter().collect(),
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

impl Executor for ShellExecutor {
    fn execute(&self, script: &ScriptCode) -> Result<Output, Error> {
        let ScriptCode(code_string) = script;

        Command::new(&self.command)
            .args(&self.args)
            .arg(code_string)
            .envs(&self.env)
            .output()
            .map(Output::from)
            .map_err(|err| Error::CommandFailed {
                command: format!("{} {:?}", self.command, self.args),
                message: err.to_string(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{Executor, ShellExecutor};

    mod shell {
        use crate::runner::Error;
        use crate::types::ScriptCode;

        use super::{Executor, ShellExecutor};

        #[cfg(not(windows))]
        #[test]
        fn new_with_command_with_arguments() {
            let shell = ShellExecutor::new("bash -c", &[]).expect("shell to be created");
            let output = shell
                .execute(&ScriptCode("echo $0".to_string()))
                .expect("success");
            assert_eq!(output.stdout, "bash\n");
        }

        #[cfg(windows)]
        #[test]
        fn new_with_command_with_arguments() {
            let shell = ShellExecutor::new("cmd.exe /c", &[]).expect("shell to be created");
            let output = shell
                .execute(&ScriptCode("echo cmd.exe".to_string()))
                .expect("success");
            assert_eq!(output.stdout, "cmd.exe\r\n");
        }

        #[test]
        fn new_with_command_without_arguments() {
            let shell = ShellExecutor::new("echo", &[]).expect("shell to be created");
            let output = shell
                .execute(&ScriptCode("hello".to_string()))
                .expect("success");
            let expected = "hello\n";
            assert_eq!(output.stdout, expected);
        }

        #[test]
        fn new_with_empty_command_string() {
            assert_eq!(
                ShellExecutor::new("", &[]),
                Err(Error::BadShellCommand {
                    command: "".to_string(),
                    message: "Command is empty".to_string(),
                })
            );
        }

        #[test]
        fn new_invalid_command() {
            assert_eq!(
                ShellExecutor::new("broken \" command", &[]),
                Err(Error::BadShellCommand {
                    command: "broken \" command".to_string(),
                    message: "Parse error : missing closing quote".to_string(),
                })
            );
        }

        #[cfg(not(windows))]
        #[test]
        fn returning_utf8_chars() {
            let shell = ShellExecutor::new("bash -c", &[]).expect("shell to be created");
            let output = shell
                .execute(&ScriptCode("echo '\u{2550}'".to_string()))
                .expect("success");
            let expected = "\u{2550}\n";
            assert_eq!(output.stdout, expected);
        }

        #[cfg(not(windows))]
        #[test]
        fn returning_stderr() {
            let shell = ShellExecutor::new("bash -c", &[]).expect("shell to be created");
            let output = shell
                .execute(&ScriptCode("echo 'test' >&2".to_string()))
                .expect("success");
            let expected = "test\n";
            assert_eq!(output.stderr, expected);
        }

        #[cfg(not(windows))]
        #[test]
        fn returning_exit_code() {
            let shell = ShellExecutor::new("bash -c", &[]).expect("shell to be created");
            let output = shell
                .execute(&ScriptCode("exit 12".to_string()))
                .expect("success");
            assert_eq!(output.exit_code, Some(12));
        }

        #[cfg(not(windows))]
        #[test]
        fn with_environment_variable() {
            let shell =
                ShellExecutor::new("bash -c", &[("MESSAGE".to_string(), "hello".to_string())])
                    .expect("shell to be created");
            let output = shell
                .execute(&ScriptCode("echo $MESSAGE".to_string()))
                .expect("success");
            assert_eq!("hello\n", output.stdout);
        }
    }
}
