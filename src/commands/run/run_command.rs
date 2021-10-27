use std::path::{Path, PathBuf};

use crate::parser;
use crate::runner::{Error, Executor, RunEvent, Runner, State};
use crate::types::ScriptCode;

use super::file_reader::FileReader;

pub struct RunCommand {
    pub spec_files: Vec<PathBuf>,
    pub executor: Box<dyn Executor>,
    pub working_dir: PathBuf,
    pub workspace_init_command: Option<String>,
    pub file_reader: FileReader,
}

impl RunCommand {
    pub fn execute(&self) -> Vec<RunEvent> {
        self.change_to_working_directory();

        self.initialise_workspace();

        self.spec_files
            .iter()
            .flat_map(|spec_file| self.run_spec_file(spec_file))
            .collect()
    }

    fn initialise_workspace(&self) {
        if let Some(command) = self.workspace_init_command.clone() {
            self.executor
                .execute(&ScriptCode(command))
                .expect("Failed to initialise workspace");
        }
    }

    fn run_spec_file(&self, spec_file: &Path) -> Vec<RunEvent> {
        let mut state = State::new();
        let mut runner = Runner::create(&*self.executor, &mut state);

        let start_events = vec![RunEvent::SpecFileStarted(spec_file.to_path_buf())];
        let contents = self.file_reader.read_file(spec_file);
        let run_events = parser::parse(&contents)
            .map_err(|err| Error::RunFailed {
                message: err.to_string(),
            })
            .map(|action_list| runner.run(&action_list))
            .or_else::<Error, _>(|err| Ok(vec![RunEvent::ErrorOccurred(err)]))
            .unwrap();
        let end_events = vec![RunEvent::SpecFileCompleted {
            success: state.is_success(),
        }];

        start_events
            .into_iter()
            .chain(run_events.into_iter())
            .chain(end_events.into_iter())
            .collect()
    }

    fn change_to_working_directory(&self) {
        std::env::set_current_dir(&self.working_dir).expect("Failed to set running directory");
    }
}
