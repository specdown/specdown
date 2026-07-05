use std::path::{Path, PathBuf};

use rayon::prelude::*;

use crate::parsers;
use crate::runner::{Error, Executor, RunEvent, Runner, State};
use crate::types::ScriptCode;

use super::file_reader::FileReader;

pub struct RunCommand {
    pub spec_files: Vec<PathBuf>,
    pub executor: Box<dyn Executor>,
    pub working_dir: PathBuf,
    pub workspace_init_command: Option<String>,
    pub file_reader: FileReader,
    /// The number of parallel jobs to use when running specs.
    ///
    /// A value of 0 has already been resolved to the CPU count by the CLI layer.
    /// When greater than 1, spec files are executed in parallel using rayon.
    pub jobs: usize,
}

impl RunCommand {
    pub fn execute(&self) -> Vec<RunEvent> {
        self.change_to_working_directory();

        self.initialise_workspace();

        if self.jobs > 1 {
            self.execute_parallel()
        } else {
            self.execute_sequential()
        }
    }

    fn execute_sequential(&self) -> Vec<RunEvent> {
        self.spec_files
            .iter()
            .flat_map(|spec_file| self.run_spec_file(spec_file))
            .collect()
    }

    fn execute_parallel(&self) -> Vec<RunEvent> {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.jobs)
            .build()
            .expect("Failed to create thread pool for parallel execution");

        let results: Vec<Vec<RunEvent>> = pool.install(|| {
            self.spec_files
                .par_iter()
                .map(|spec_file| self.run_spec_file(spec_file))
                .collect()
        });

        results.into_iter().flatten().collect()
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
        let run_events = parsers::parse(&contents)
            .map_err(Error::RunFailed)
            .map(|action_list| runner.run(&action_list))
            .or_else::<Error, _>(|err| Ok(vec![RunEvent::ErrorOccurred(err)]))
            .unwrap();
        let end_events = vec![RunEvent::SpecFileCompleted {
            success: state.is_success(),
        }];

        start_events
            .into_iter()
            .chain(run_events)
            .chain(end_events)
            .collect()
    }

    fn change_to_working_directory(&self) {
        std::env::set_current_dir(&self.working_dir).expect("Failed to set running directory");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::run::exit_code;
    use crate::runner::Output;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Mutex;
    use tempfile::tempdir;

    /// A mutex to serialize tests that change the process-wide CWD
    /// (`RunCommand::execute()` calls `std::env::set_current_dir`).
    /// Without this, parallel test execution causes data races on the CWD.
    static CWD_MUTEX: Mutex<()> = Mutex::new(());

    /// A mock executor that always succeeds.
    struct CountingExecutor {
        #[allow(dead_code)]
        call_count: AtomicUsize,
    }

    impl CountingExecutor {
        fn new() -> Self {
            Self {
                call_count: AtomicUsize::new(0),
            }
        }
    }

    impl Executor for CountingExecutor {
        fn execute(&self, _script: &ScriptCode) -> Result<Output, Error> {
            self.call_count.fetch_add(1, Ordering::SeqCst);
            Ok(Output {
                stdout: String::new(),
                stderr: String::new(),
                exit_code: Some(0),
            })
        }
    }

    /// A mock executor that always fails with a `CommandFailed` error.
    struct FailingExecutor;

    impl Executor for FailingExecutor {
        fn execute(&self, _script: &ScriptCode) -> Result<Output, Error> {
            Err(Error::CommandFailed {
                command: "mock-failing-command".to_string(),
                message: "intentional failure for testing".to_string(),
            })
        }
    }

    fn write_spec_file(dir: &std::path::Path, name: &str, content: &str) -> PathBuf {
        let path = dir.join(name);
        std::fs::write(&path, content).expect("Failed to write spec file");
        path
    }

    const SIMPLE_SPEC: &str = "# Test Spec\n\n```shell,script(name=\"test\")\necho hello\n```\n";

    fn make_run_command(
        spec_files: Vec<PathBuf>,
        executor: Box<dyn Executor>,
        working_dir: PathBuf,
        file_reader: FileReader,
        jobs: usize,
    ) -> RunCommand {
        RunCommand {
            spec_files,
            executor,
            working_dir,
            workspace_init_command: None,
            file_reader,
            jobs,
        }
    }

    /// Run a closure while holding the CWD mutex, saving and restoring the
    /// process-wide working directory around it.
    fn with_saved_cwd<F: FnOnce() -> Vec<RunEvent>>(f: F) -> Vec<RunEvent> {
        let _guard = CWD_MUTEX.lock().expect("CWD mutex poisoned");
        let original_dir = std::env::current_dir().expect("Failed to get current directory");
        let events = f();
        std::env::set_current_dir(&original_dir).expect("Failed to restore current directory");
        events
    }

    #[test]
    fn sequential_execution_runs_all_spec_files() {
        let dir = tempdir().expect("Failed to create temp dir");
        let spec1 = write_spec_file(dir.path(), "spec1.md", SIMPLE_SPEC);
        let spec2 = write_spec_file(dir.path(), "spec2.md", SIMPLE_SPEC);
        let executor = Box::new(CountingExecutor::new());
        let file_reader = FileReader::new(dir.path().to_path_buf());

        let cmd = make_run_command(
            vec![spec1.clone(), spec2.clone()],
            executor,
            dir.path().to_path_buf(),
            file_reader,
            1,
        );
        let events = with_saved_cwd(|| cmd.execute());

        // Each spec file produces: SpecFileStarted, TestCompleted(s), SpecFileCompleted
        assert!(
            events.len() >= 4,
            "Should have events for both spec files, got {}",
            events.len()
        );

        // Verify order: first event should be SpecFileStarted for spec1
        match &events[0] {
            RunEvent::SpecFileStarted(path) => {
                assert_eq!(path.file_name().unwrap(), "spec1.md");
            }
            _ => panic!("Expected SpecFileStarted for spec1"),
        }

        // The second group should start with spec2
        let spec2_start = events.iter().position(
            |e| matches!(e, RunEvent::SpecFileStarted(p) if p.file_name().unwrap() == "spec2.md"),
        );
        assert!(
            spec2_start.is_some(),
            "Should find SpecFileStarted for spec2"
        );
    }

    #[test]
    fn parallel_execution_preserves_file_order() {
        let dir = tempdir().expect("Failed to create temp dir");
        let spec1 = write_spec_file(dir.path(), "a_spec.md", SIMPLE_SPEC);
        let spec2 = write_spec_file(dir.path(), "b_spec.md", SIMPLE_SPEC);
        let spec3 = write_spec_file(dir.path(), "c_spec.md", SIMPLE_SPEC);

        let file_reader = FileReader::new(dir.path().to_path_buf());
        let cmd = make_run_command(
            vec![spec1, spec2, spec3],
            Box::new(CountingExecutor::new()),
            dir.path().to_path_buf(),
            file_reader,
            4,
        );
        let events = with_saved_cwd(|| cmd.execute());

        // Collect the SpecFileStarted events in order
        let started_paths: Vec<_> = events
            .iter()
            .filter_map(|e| match e {
                RunEvent::SpecFileStarted(p) => Some(p.file_name().unwrap().to_owned()),
                _ => None,
            })
            .collect();

        assert_eq!(
            started_paths,
            vec!["a_spec.md", "b_spec.md", "c_spec.md"],
            "Parallel execution should preserve original file order"
        );
    }

    #[test]
    fn parallel_execution_produces_nonzero_exit_when_any_spec_fails() {
        let dir = tempdir().expect("Failed to create temp dir");
        let spec1 = write_spec_file(dir.path(), "passing.md", SIMPLE_SPEC);
        let spec2 = write_spec_file(dir.path(), "failing.md", SIMPLE_SPEC);

        let file_reader = FileReader::new(dir.path().to_path_buf());
        let cmd = make_run_command(
            vec![spec1, spec2],
            Box::new(FailingExecutor),
            dir.path().to_path_buf(),
            file_reader,
            2,
        );
        let events = with_saved_cwd(|| cmd.execute());

        let exit_code = exit_code::from_events(&events);
        assert_ne!(
            exit_code as i32, 0,
            "Exit code should be non-zero when any spec fails"
        );
    }

    #[test]
    fn parallel_execution_with_single_spec_file() {
        let dir = tempdir().expect("Failed to create temp dir");
        let spec = write_spec_file(dir.path(), "only.md", SIMPLE_SPEC);

        let file_reader = FileReader::new(dir.path().to_path_buf());
        let cmd = make_run_command(
            vec![spec],
            Box::new(CountingExecutor::new()),
            dir.path().to_path_buf(),
            file_reader,
            4,
        );
        let events = with_saved_cwd(|| cmd.execute());

        assert!(
            events
                .iter()
                .any(|e| matches!(e, RunEvent::SpecFileStarted(_))),
            "Should have SpecFileStarted event"
        );
        assert!(
            events
                .iter()
                .any(|e| matches!(e, RunEvent::SpecFileCompleted { .. })),
            "Should have SpecFileCompleted event"
        );
    }

    #[test]
    fn parallel_execution_with_empty_spec_list() {
        let dir = tempdir().expect("Failed to create temp dir");
        let file_reader = FileReader::new(dir.path().to_path_buf());
        let cmd = make_run_command(
            vec![],
            Box::new(CountingExecutor::new()),
            dir.path().to_path_buf(),
            file_reader,
            4,
        );
        let events = with_saved_cwd(|| cmd.execute());
        assert!(events.is_empty(), "No spec files should produce no events");
    }
}
