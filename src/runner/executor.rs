use std::process::Command;

use crate::types::ScriptCode;

use super::error::Error;

pub struct Output {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
}

pub trait Executor {
    fn execute(&self, script: &ScriptCode) -> Result<Output, Error>;
}

pub struct Bash {}

impl Bash {
    pub fn new() -> Self {
        Self {}
    }
}

impl Executor for Bash {
    fn execute(&self, script: &ScriptCode) -> Result<Output, Error> {
        let ScriptCode(code_string) = script;

        let command_result = Command::new("bash").arg("-c").arg(code_string).output();

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
            Err(_err) => Err(Error::CommandFailed),
        }
    }
}
