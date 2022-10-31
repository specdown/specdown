use std::collections::HashMap;
use std::process::Command;

use shell_words::ParseError;

use crate::types::ScriptCode;

use super::executor::Output;
use super::{Error, Executor};
use std::env;
use std::env::JoinPathsError;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq)]
pub struct ShellExecutor {
    command: String,
    args: Vec<String>,
    env: HashMap<String, String>,
    unset_env: Vec<String>,
    paths: Vec<PathBuf>,
}

impl ShellExecutor {
    pub fn new<P>(
        shell_command: &str,
        env: &[(String, String)],
        unset_env: &[String],
        paths: &[P],
    ) -> Result<Self, Error>
    where
        P: AsRef<OsStr>,
    {
        shell_words::split(shell_command)
            .map_err(|err| Self::parse_error_to_error(shell_command, err))
            .and_then(|words| Self::check_is_not_empty(shell_command, &words))
            .map(|words| Self::create_shell_instance(&words, env, unset_env, paths))
    }

    fn create_shell_instance<P>(
        words: &[String],
        env: &[(String, String)],
        unset_env: &[String],
        paths: &[P],
    ) -> Self
    where
        P: AsRef<OsStr>,
    {
        let (command, args) = words.split_at(1);
        Self {
            command: command.first().unwrap().to_string(),
            args: Vec::from(args),
            env: env.iter().cloned().collect(),
            unset_env: unset_env.to_vec(),
            paths: paths.iter().map(PathBuf::from).collect(),
        }
    }

    fn parse_error_to_error(shell_command: &str, err: ParseError) -> Error {
        Error::BadShellCommand {
            command: shell_command.to_string(),
            message: format!("Parse error : {err}"),
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

    fn path_env_var(&self) -> Result<OsString, JoinPathsError> {
        let mut paths: Vec<PathBuf> = self.paths.clone();

        if let Ok(current_path) = env::var("PATH") {
            let mut s = env::split_paths(&current_path).collect();
            paths.append(&mut s);
        }

        env::join_paths(paths)
    }
}

impl Executor for ShellExecutor {
    fn execute(&self, script: &ScriptCode) -> Result<Output, Error> {
        let ScriptCode(code_string) = script;

        let path = self.path_env_var();

        let mut command = Command::new(&self.command);

        command
            .args(&self.args)
            .arg(code_string)
            .envs(&self.env)
            .env("PATH", path.expect("Failed to construct PATH"));

        for name in &self.unset_env {
            command.env_remove(name);
        }

        let output = command.output();

        output
            .map(Output::from)
            .map_err(|err| Error::CommandFailed {
                command: format!("{} {:?}", self.command, self.args),
                message: err.to_string(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, Executor, ScriptCode, ShellExecutor};

    mod shell {
        use super::{Error, Executor, ScriptCode, ShellExecutor};
        #[cfg(not(windows))]
        use std::env;
        use std::path::PathBuf;

        #[cfg(not(windows))]
        #[test]
        fn new_with_command_with_arguments() {
            let shell = ShellExecutor::new::<PathBuf>("bash -c", &[], &[], &[])
                .expect("shell to be created");
            let output = shell
                .execute(&ScriptCode("echo $0".to_string()))
                .expect("success");
            assert_eq!(output.stdout, "bash\n");
        }

        #[cfg(windows)]
        #[test]
        fn new_with_command_with_arguments() {
            let shell = ShellExecutor::new::<PathBuf>("cmd.exe /c", &[], &[], &[])
                .expect("shell to be created");
            let output = shell
                .execute(&ScriptCode("echo cmd.exe".to_string()))
                .expect("success");
            assert_eq!(output.stdout, "cmd.exe\r\n");
        }

        #[test]
        fn new_with_command_without_arguments() {
            let shell =
                ShellExecutor::new::<PathBuf>("echo", &[], &[], &[]).expect("shell to be created");
            let output = shell
                .execute(&ScriptCode("hello".to_string()))
                .expect("success");
            let expected = "hello\n";
            assert_eq!(output.stdout, expected);
        }

        #[test]
        fn new_with_empty_command_string() {
            assert_eq!(
                ShellExecutor::new::<PathBuf>("", &[], &[], &[]),
                Err(Error::BadShellCommand {
                    command: String::new(),
                    message: "Command is empty".to_string(),
                })
            );
        }

        #[test]
        fn new_invalid_command() {
            assert_eq!(
                ShellExecutor::new::<PathBuf>("broken \" command", &[], &[], &[]),
                Err(Error::BadShellCommand {
                    command: "broken \" command".to_string(),
                    message: "Parse error : missing closing quote".to_string(),
                })
            );
        }

        #[cfg(not(windows))]
        #[test]
        fn returning_utf8_chars() {
            let shell = ShellExecutor::new::<PathBuf>("bash -c", &[], &[], &[])
                .expect("shell to be created");
            let output = shell
                .execute(&ScriptCode("echo '\u{2550}'".to_string()))
                .expect("success");
            let expected = "\u{2550}\n";
            assert_eq!(output.stdout, expected);
        }

        #[cfg(not(windows))]
        #[test]
        fn returning_stderr() {
            let shell = ShellExecutor::new::<PathBuf>("bash -c", &[], &[], &[])
                .expect("shell to be created");
            let output = shell
                .execute(&ScriptCode("echo 'test' >&2".to_string()))
                .expect("success");
            let expected = "test\n";
            assert_eq!(output.stderr, expected);
        }

        #[cfg(not(windows))]
        #[test]
        fn returning_exit_code() {
            let shell = ShellExecutor::new::<PathBuf>("bash -c", &[], &[], &[])
                .expect("shell to be created");
            let output = shell
                .execute(&ScriptCode("exit 12".to_string()))
                .expect("success");
            assert_eq!(output.exit_code, Some(12));
        }

        #[cfg(not(windows))]
        #[test]
        fn with_environment_variable() {
            let shell = ShellExecutor::new::<PathBuf>(
                "bash -c",
                &[("MESSAGE".to_string(), "hello".to_string())],
                &[],
                &[],
            )
            .expect("shell to be created");
            let output = shell
                .execute(&ScriptCode("echo $MESSAGE".to_string()))
                .expect("success");
            assert_eq!("hello\n", output.stdout);
        }

        #[cfg(not(windows))]
        #[test]
        fn with_unset_environment_variable() {
            env::set_var("UNSET_ME", "value");

            let shell = ShellExecutor::new::<PathBuf>(
                "bash -c",
                &[("MESSAGE".to_string(), "hello".to_string())],
                &["UNSET_ME".to_string()],
                &[],
            )
            .expect("shell to be created");

            let output = shell
                .execute(&ScriptCode("echo $UNSET_ME".to_string()))
                .expect("success");

            assert_eq!("\n", output.stdout);
        }

        #[cfg(not(windows))]
        #[test]
        fn with_added_paths() {
            let shell = ShellExecutor::new("bash -c", &[], &[], &["my/bin", "other/bin"])
                .expect("shell to be created");
            let path = env::var("PATH").expect("PATH environment variable must be set");
            let output = shell
                .execute(&ScriptCode("echo -n $PATH".to_string()))
                .expect("success");
            assert_eq!(format!("my/bin:other/bin:{path}"), output.stdout);
        }
    }
}
