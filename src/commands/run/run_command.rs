use std::path::{Path, PathBuf};
use std::sync::Mutex;

use rayon::prelude::*;

use crate::parsers;
use crate::results::Printer;
use crate::runner::{Error, Executor, RunEvent, Runner, State};
use crate::types::ScriptCode;
use crate::workspace::{TemporaryDirectory, Workspace};

use super::executor_factory::ExecutorFactory;
use super::file_reader::FileReader;
use super::specdown_env;

/// How workspaces and executors are provided to `RunCommand`.
pub enum RunMode {
    /// One workspace and one executor, shared by every spec file — the
    /// behaviour used whenever `--workspace-per-spec` is not set.
    SharedWorkspace {
        executor: Box<dyn Executor>,
        working_dir: PathBuf,
    },
    /// A fresh temporary workspace (and fresh executor instance) is created
    /// for every spec file; `workspace_init_command` is re-run each time.
    /// Used when `--workspace-per-spec` is set.
    PerSpecWorkspace {
        factory: Box<dyn ExecutorFactory>,
        start_dir: PathBuf,
        working_dir_suffix: Option<PathBuf>,
    },
}

pub struct RunCommand {
    pub spec_files: Vec<PathBuf>,
    pub run_mode: RunMode,
    pub workspace_init_command: Option<String>,
    pub file_reader: FileReader,
    /// The number of parallel jobs to use when running specs.
    ///
    /// A value of 0 has already been resolved to the CPU count by the CLI layer.
    /// When greater than 1, spec files are executed in parallel using rayon.
    pub jobs: usize,
}

impl RunCommand {
    /// Execute specs and print output via `printer` as results arrive.
    ///
    /// In parallel mode (`--jobs > 1`), each spec file's complete output is
    /// printed atomically under a mutex lock as soon as that file finishes,
    /// so output from different files never interleaves. A clear file header
    /// introduces each spec file's results. The events are also returned in
    /// original file order for exit-code computation.
    pub fn execute_with_printer(&self, printer: &Mutex<Box<dyn Printer>>) -> Vec<RunEvent> {
        match &self.run_mode {
            RunMode::SharedWorkspace {
                executor,
                working_dir,
            } => {
                self.initialise_workspace(executor.as_ref());

                if self.jobs > 1 {
                    self.execute_parallel_shared(printer, executor.as_ref(), working_dir)
                } else {
                    self.execute_sequential_shared(printer, executor.as_ref(), working_dir)
                }
            }
            RunMode::PerSpecWorkspace { .. } => {
                if self.jobs > 1 {
                    self.execute_parallel_per_spec(printer)
                } else {
                    self.execute_sequential_per_spec(printer)
                }
            }
        }
    }

    fn execute_sequential_shared(
        &self,
        printer: &Mutex<Box<dyn Printer>>,
        executor: &dyn Executor,
        working_dir: &Path,
    ) -> Vec<RunEvent> {
        let mut all_events = Vec::new();
        for spec_file in &self.spec_files {
            let events = self.run_spec_file_with_executor(spec_file, executor, working_dir);
            let mut guard = printer.lock().expect("printer mutex poisoned");
            for event in &events {
                guard.print(event);
            }
            drop(guard);
            all_events.extend(events);
        }
        all_events
    }

    fn execute_parallel_shared(
        &self,
        printer: &Mutex<Box<dyn Printer>>,
        executor: &dyn Executor,
        working_dir: &Path,
    ) -> Vec<RunEvent> {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.jobs)
            .build()
            .expect("Failed to create thread pool for parallel execution");

        let results: Vec<Vec<RunEvent>> = pool.install(|| {
            self.spec_files
                .par_iter()
                .map(|spec_file| {
                    // Clone the executor for each spec file so that
                    // stateful executors (e.g. ContainerExecutor) get
                    // their own isolated container/instance. The spec file
                    // path is passed as a label so the container executor
                    // can incorporate a file-hash into the container name.
                    let cloned_executor =
                        executor.clone_box(spec_file.to_str().unwrap_or("unknown"));
                    let events =
                        self.run_spec_file_with_executor(spec_file, &*cloned_executor, working_dir);
                    // Lock the printer so output from this spec file is printed
                    // atomically and never interleaves with output from another.
                    let mut guard = printer.lock().expect("printer mutex poisoned");
                    for event in &events {
                        guard.print(event);
                    }
                    events
                })
                .collect()
        });

        // `par_iter().collect()` preserves original order, so flattening
        // yields events in the original spec-file order.
        results.into_iter().flatten().collect()
    }

    fn initialise_workspace(&self, executor: &dyn Executor) {
        if let Some(command) = self.workspace_init_command.clone() {
            executor
                .execute(&ScriptCode(command))
                .expect("Failed to initialise workspace");
        }
    }

    fn execute_sequential_per_spec(&self, printer: &Mutex<Box<dyn Printer>>) -> Vec<RunEvent> {
        let mut all_events = Vec::new();
        for spec_file in &self.spec_files {
            let events = self.run_spec_file_per_spec(spec_file);
            let mut guard = printer.lock().expect("printer mutex poisoned");
            for event in &events {
                guard.print(event);
            }
            drop(guard);
            all_events.extend(events);
        }
        all_events
    }

    fn execute_parallel_per_spec(&self, printer: &Mutex<Box<dyn Printer>>) -> Vec<RunEvent> {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(self.jobs)
            .build()
            .expect("Failed to create thread pool for parallel execution");

        let results: Vec<Vec<RunEvent>> = pool.install(|| {
            self.spec_files
                .par_iter()
                .map(|spec_file| {
                    // Each spec file builds its own fresh workspace and
                    // executor here (see `build_spec_workspace`) — nothing
                    // is shared across threads, so this is safe under real
                    // parallel execution.
                    let events = self.run_spec_file_per_spec(spec_file);
                    let mut guard = printer.lock().expect("printer mutex poisoned");
                    for event in &events {
                        guard.print(event);
                    }
                    events
                })
                .collect()
        });

        // `par_iter().collect()` preserves original order, so flattening
        // yields events in the original spec-file order.
        results.into_iter().flatten().collect()
    }

    /// Creates a fresh temporary workspace and a fresh executor for a single
    /// spec file, in `RunMode::PerSpecWorkspace` mode.
    fn build_spec_workspace(
        &self,
        spec_file: &Path,
    ) -> Result<(Box<dyn Executor>, PathBuf), Error> {
        let RunMode::PerSpecWorkspace {
            factory,
            start_dir,
            working_dir_suffix,
        } = &self.run_mode
        else {
            unreachable!("build_spec_workspace requires RunMode::PerSpecWorkspace")
        };

        let mut workspace = TemporaryDirectory::create();
        workspace.initialize();

        let working_dir = working_dir_suffix.clone().map_or_else(
            || workspace.dir().clone(),
            |dir| workspace.dir().clone().join(dir),
        );

        let extra_env = specdown_env::build(start_dir, workspace.dir(), &working_dir);
        let label = spec_file.to_str().unwrap_or("unknown");
        let executor = factory.build(label, &extra_env, &working_dir)?;

        Ok((executor, working_dir))
    }

    /// Runs a single spec file in its own fresh workspace: builds the
    /// workspace and executor, re-runs `workspace_init_command` in it, then
    /// runs the spec file's own actions.
    fn run_spec_file_per_spec(&self, spec_file: &Path) -> Vec<RunEvent> {
        match self.build_spec_workspace(spec_file) {
            Ok((executor, working_dir)) => {
                self.initialise_workspace(executor.as_ref());
                self.run_spec_file_with_executor(spec_file, executor.as_ref(), &working_dir)
            }
            Err(err) => vec![
                RunEvent::SpecFileStarted(spec_file.to_path_buf()),
                RunEvent::ErrorOccurred(err),
                RunEvent::SpecFileCompleted { success: false },
            ],
        }
    }

    /// Run a single spec file using the given executor and working directory.
    ///
    /// This is used by parallel execution to pass a cloned executor
    /// (via `Executor::clone_box`) so each spec file gets its own
    /// isolated state (e.g. its own Docker container).
    fn run_spec_file_with_executor(
        &self,
        spec_file: &Path,
        executor: &dyn Executor,
        working_dir: &Path,
    ) -> Vec<RunEvent> {
        let mut state = State::new();
        let mut runner = Runner::create(executor, working_dir, &mut state);

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::run::exit_code;
    use crate::runner::Output;
    use std::fmt::Write;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use tempfile::tempdir;

    /// A `Printer` that does nothing — used in tests that only need the
    /// returned `Vec<RunEvent>` and don't care about printed output.
    struct NullPrinter;

    impl Printer for NullPrinter {
        fn print(&mut self, _event: &RunEvent) {}
    }

    /// A `Printer` that captures all output into a string for assertions.
    struct CapturingPrinter {
        output: Arc<Mutex<String>>,
    }

    impl CapturingPrinter {
        fn new_pair() -> (Self, Arc<Mutex<String>>) {
            let output = Arc::new(Mutex::new(String::new()));
            let printer = Self {
                output: Arc::clone(&output),
            };
            (printer, output)
        }
    }

    impl Printer for CapturingPrinter {
        fn print(&mut self, event: &RunEvent) {
            let mut guard = self.output.lock().expect("capture mutex poisoned");
            match event {
                RunEvent::SpecFileStarted(path) => {
                    let _ = writeln!(guard, "START: {}", path.display());
                }
                RunEvent::SpecFileCompleted { success } => {
                    let _ = writeln!(guard, "END: success={success}");
                }
                RunEvent::TestCompleted(result) => {
                    let _ = writeln!(guard, "TEST: success={}", result.success());
                }
                RunEvent::ErrorOccurred(error) => {
                    let _ = writeln!(guard, "ERROR: {error}");
                }
            }
        }
    }

    fn null_printer() -> Mutex<Box<dyn Printer>> {
        Mutex::new(Box::new(NullPrinter))
    }

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
            run_mode: RunMode::SharedWorkspace {
                executor,
                working_dir,
            },
            workspace_init_command: None,
            file_reader,
            jobs,
        }
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
        let printer = null_printer();
        let events = cmd.execute_with_printer(&printer);

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
        let printer = null_printer();
        let events = cmd.execute_with_printer(&printer);

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
        let printer = null_printer();
        let events = cmd.execute_with_printer(&printer);

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
        let printer = null_printer();
        let events = cmd.execute_with_printer(&printer);

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
        let printer = null_printer();
        let events = cmd.execute_with_printer(&printer);
        assert!(events.is_empty(), "No spec files should produce no events");
    }

    #[test]
    fn parallel_output_does_not_interleave_between_files() {
        let dir = tempdir().expect("Failed to create temp dir");
        let spec1 = write_spec_file(dir.path(), "first.md", SIMPLE_SPEC);
        let spec2 = write_spec_file(dir.path(), "second.md", SIMPLE_SPEC);
        let spec3 = write_spec_file(dir.path(), "third.md", SIMPLE_SPEC);

        let file_reader = FileReader::new(dir.path().to_path_buf());
        let cmd = make_run_command(
            vec![spec1, spec2, spec3],
            Box::new(CountingExecutor::new()),
            dir.path().to_path_buf(),
            file_reader,
            4,
        );
        let (printer, output) = CapturingPrinter::new_pair();
        let printer_mutex = Mutex::new(Box::new(printer) as Box<dyn Printer>);
        let _events = cmd.execute_with_printer(&printer_mutex);

        let captured = output.lock().expect("capture mutex poisoned");
        let captured_str = captured.as_str();

        // Each file's output block should be contiguous — no interleaving.
        // We check that "START: first" appears before any "START: second"
        // or "START: third" is NOT guaranteed (files complete in arbitrary
        // order), but each START..END block must be contiguous.
        //
        // Instead of asserting order (parallel completion order is
        // nondeterministic), we verify that each file's START and END
        // appear and that between a START and its corresponding END there
        // is no other START line (no interleaving).

        for file_name in &["first.md", "second.md", "third.md"] {
            let start_marker = format!("START: {file_name}");
            let start_pos = captured_str
                .find(file_name)
                .unwrap_or_else(|| panic!("should find START for {}", file_name));

            // Find the next START after this one
            let rest_after_start = &captured_str[start_pos + start_marker.len()..];
            let next_start = rest_after_start.find("START:");
            // Find the END for this file — it's the first END after START
            let end_pos = rest_after_start
                .find("END:")
                .unwrap_or_else(|| panic!("should find END after START for {}", file_name));

            // The END must come before any other START (no interleaving)
            if let Some(ns) = next_start {
                assert!(
                    end_pos < ns,
                    "Output for {} interleaves with another file's START",
                    file_name
                );
            }
        }
    }

    #[test]
    fn parallel_output_includes_file_headers() {
        let dir = tempdir().expect("Failed to create temp dir");
        let spec1 = write_spec_file(dir.path(), "alpha.md", SIMPLE_SPEC);
        let spec2 = write_spec_file(dir.path(), "beta.md", SIMPLE_SPEC);

        let file_reader = FileReader::new(dir.path().to_path_buf());
        let cmd = make_run_command(
            vec![spec1, spec2],
            Box::new(CountingExecutor::new()),
            dir.path().to_path_buf(),
            file_reader,
            4,
        );
        let (printer, output) = CapturingPrinter::new_pair();
        let printer_mutex = Mutex::new(Box::new(printer) as Box<dyn Printer>);
        let _events = cmd.execute_with_printer(&printer_mutex);

        let captured = output.lock().expect("capture mutex poisoned");

        // Both files should have their file paths in the output
        assert!(
            captured.contains("alpha.md"),
            "Output should contain alpha.md file header, got: {:?}",
            captured
        );
        assert!(
            captured.contains("beta.md"),
            "Output should contain beta.md file header, got: {:?}",
            captured
        );
    }

    #[test]
    fn sequential_output_with_printer_prints_all_events() {
        let dir = tempdir().expect("Failed to create temp dir");
        let spec1 = write_spec_file(dir.path(), "seq1.md", SIMPLE_SPEC);
        let spec2 = write_spec_file(dir.path(), "seq2.md", SIMPLE_SPEC);

        let file_reader = FileReader::new(dir.path().to_path_buf());
        let cmd = make_run_command(
            vec![spec1, spec2],
            Box::new(CountingExecutor::new()),
            dir.path().to_path_buf(),
            file_reader,
            1,
        );
        let (printer, output) = CapturingPrinter::new_pair();
        let printer_mutex = Mutex::new(Box::new(printer) as Box<dyn Printer>);
        let _events = cmd.execute_with_printer(&printer_mutex);

        let captured = output.lock().expect("capture mutex poisoned");

        // Sequential mode should print in order
        let seq1_pos = captured
            .find("seq1.md")
            .expect("should find START for seq1");
        let seq2_pos = captured
            .find("seq2.md")
            .expect("should find START for seq2");
        assert!(
            seq1_pos < seq2_pos,
            "Sequential output should print seq1 before seq2"
        );
    }

    // ------------------------------------------------------------------
    // `--workspace-per-spec` (`RunMode::PerSpecWorkspace`) tests
    // ------------------------------------------------------------------

    use super::super::executor_factory::ExecutorFactory;
    use std::collections::HashSet;

    type RecordedCall = (String, Vec<(String, String)>, PathBuf);

    /// An executor whose `execute()` calls all increment a *shared* counter,
    /// so tests can assert how many times `execute()` was called across
    /// every executor instance a factory builds (each spec file in
    /// `PerSpecWorkspace` mode gets its own executor instance).
    struct SharedCountingExecutor {
        count: Arc<AtomicUsize>,
    }

    impl Executor for SharedCountingExecutor {
        fn execute(&self, _script: &ScriptCode) -> Result<Output, Error> {
            self.count.fetch_add(1, Ordering::SeqCst);
            Ok(Output {
                stdout: String::new(),
                stderr: String::new(),
                exit_code: Some(0),
            })
        }
    }

    /// A mock `ExecutorFactory` that records every `build()` call (label,
    /// extra env, working dir) and hands back a `SharedCountingExecutor`
    /// wired to a shared call counter. The `calls` and `execute_count`
    /// handles are `Arc`s shared with the factory, so tests can inspect them
    /// after `RunCommand::execute_with_printer` runs without needing to
    /// downcast the `Box<dyn ExecutorFactory>` trait object stored on
    /// `RunCommand`.
    struct RecordingExecutorFactory {
        calls: Arc<Mutex<Vec<RecordedCall>>>,
        execute_count: Arc<AtomicUsize>,
    }

    impl RecordingExecutorFactory {
        fn new() -> (Self, Arc<Mutex<Vec<RecordedCall>>>, Arc<AtomicUsize>) {
            let calls = Arc::new(Mutex::new(Vec::new()));
            let execute_count = Arc::new(AtomicUsize::new(0));
            let factory = Self {
                calls: Arc::clone(&calls),
                execute_count: Arc::clone(&execute_count),
            };
            (factory, calls, execute_count)
        }
    }

    impl ExecutorFactory for RecordingExecutorFactory {
        fn build(
            &self,
            label: &str,
            extra_env: &[(String, String)],
            working_dir: &Path,
        ) -> Result<Box<dyn Executor>, Error> {
            self.calls.lock().expect("mutex poisoned").push((
                label.to_string(),
                extra_env.to_vec(),
                working_dir.to_path_buf(),
            ));
            Ok(Box::new(SharedCountingExecutor {
                count: Arc::clone(&self.execute_count),
            }))
        }
    }

    fn make_per_spec_run_command(
        spec_files: Vec<PathBuf>,
        factory: Box<dyn ExecutorFactory>,
        start_dir: PathBuf,
        file_reader: FileReader,
        jobs: usize,
        workspace_init_command: Option<String>,
    ) -> RunCommand {
        RunCommand {
            spec_files,
            run_mode: RunMode::PerSpecWorkspace {
                factory,
                start_dir,
                working_dir_suffix: None,
            },
            workspace_init_command,
            file_reader,
            jobs,
        }
    }

    #[test]
    fn per_spec_sequential_builds_a_fresh_workspace_for_every_spec_file() {
        let dir = tempdir().expect("Failed to create temp dir");
        let spec1 = write_spec_file(dir.path(), "spec1.md", SIMPLE_SPEC);
        let spec2 = write_spec_file(dir.path(), "spec2.md", SIMPLE_SPEC);
        let file_reader = FileReader::new(dir.path().to_path_buf());
        let (factory, calls, _execute_count) = RecordingExecutorFactory::new();

        let cmd = make_per_spec_run_command(
            vec![spec1, spec2],
            Box::new(factory),
            dir.path().to_path_buf(),
            file_reader,
            1,
            None,
        );
        let printer = null_printer();
        let _events = cmd.execute_with_printer(&printer);

        let recorded = calls.lock().expect("mutex poisoned");
        assert_eq!(
            recorded.len(),
            2,
            "factory.build should be called once per spec file"
        );
        assert!(recorded[0].0.ends_with("spec1.md"));
        assert!(recorded[1].0.ends_with("spec2.md"));
        assert_ne!(
            recorded[0].2, recorded[1].2,
            "each spec file should get a distinct working directory"
        );
    }

    #[test]
    fn per_spec_parallel_builds_isolated_workspace_per_spec_file() {
        let dir = tempdir().expect("Failed to create temp dir");
        let spec1 = write_spec_file(dir.path(), "spec1.md", SIMPLE_SPEC);
        let spec2 = write_spec_file(dir.path(), "spec2.md", SIMPLE_SPEC);
        let spec3 = write_spec_file(dir.path(), "spec3.md", SIMPLE_SPEC);
        let file_reader = FileReader::new(dir.path().to_path_buf());
        let (factory, calls, _execute_count) = RecordingExecutorFactory::new();

        let cmd = make_per_spec_run_command(
            vec![spec1, spec2, spec3],
            Box::new(factory),
            dir.path().to_path_buf(),
            file_reader,
            4,
            None,
        );
        let printer = null_printer();
        let _events = cmd.execute_with_printer(&printer);

        let recorded = calls.lock().expect("mutex poisoned");
        assert_eq!(
            recorded.len(),
            3,
            "factory.build should be called once per spec file, even in parallel"
        );

        let distinct_working_dirs: HashSet<&PathBuf> = recorded
            .iter()
            .map(|(_, _, working_dir)| working_dir)
            .collect();
        assert_eq!(
            distinct_working_dirs.len(),
            3,
            "every spec file should get its own distinct workspace directory"
        );
    }

    #[test]
    fn per_spec_reruns_workspace_init_command_for_every_spec_file() {
        let dir = tempdir().expect("Failed to create temp dir");
        let spec1 = write_spec_file(dir.path(), "spec1.md", SIMPLE_SPEC);
        let spec2 = write_spec_file(dir.path(), "spec2.md", SIMPLE_SPEC);
        let file_reader = FileReader::new(dir.path().to_path_buf());
        let (factory, _calls, execute_count) = RecordingExecutorFactory::new();

        let cmd = make_per_spec_run_command(
            vec![spec1, spec2],
            Box::new(factory),
            dir.path().to_path_buf(),
            file_reader,
            1,
            Some("echo init".to_string()),
        );
        let printer = null_printer();
        let _events = cmd.execute_with_printer(&printer);

        // Each spec file: 1 workspace_init_command execution + 1 script
        // execution from SIMPLE_SPEC = 2 executor.execute() calls per file.
        assert_eq!(
            execute_count.load(Ordering::SeqCst),
            4,
            "workspace_init_command should be re-run once per spec file, not once total"
        );
    }

    #[test]
    fn per_spec_workspace_env_vars_differ_per_spec_file() {
        let dir = tempdir().expect("Failed to create temp dir");
        let spec1 = write_spec_file(dir.path(), "spec1.md", SIMPLE_SPEC);
        let spec2 = write_spec_file(dir.path(), "spec2.md", SIMPLE_SPEC);
        let file_reader = FileReader::new(dir.path().to_path_buf());
        let (factory, calls, _execute_count) = RecordingExecutorFactory::new();

        let cmd = make_per_spec_run_command(
            vec![spec1, spec2],
            Box::new(factory),
            dir.path().to_path_buf(),
            file_reader,
            1,
            None,
        );
        let printer = null_printer();
        let _events = cmd.execute_with_printer(&printer);

        let recorded = calls.lock().expect("mutex poisoned");
        let workspace_dir_env = |extra_env: &[(String, String)]| -> String {
            extra_env
                .iter()
                .find(|(k, _)| k == "SPECDOWN_WORKSPACE_DIR")
                .map(|(_, v)| v.clone())
                .expect("SPECDOWN_WORKSPACE_DIR should be set")
        };

        let env1 = workspace_dir_env(&recorded[0].1);
        let env2 = workspace_dir_env(&recorded[1].1);
        assert_ne!(
            env1, env2,
            "SPECDOWN_WORKSPACE_DIR should differ between spec files"
        );
    }

    /// End-to-end test using the *real* `ShellExecutorFactory` and real
    /// `TemporaryDirectory` (no mocks), under real parallel execution
    /// (`jobs: 4`): spec A writes a file, spec B asserts it's absent. This
    /// is the strongest evidence that per-spec workspace isolation holds
    /// under genuine parallel execution, not just via mocks.
    #[cfg(not(windows))]
    #[test]
    fn per_spec_parallel_execution_isolates_files_between_spec_files_for_real() {
        use super::super::executor_factory::ShellExecutorFactory;

        let dir = tempdir().expect("Failed to create temp dir");
        let spec_a = write_spec_file(
            dir.path(),
            "a.md",
            "# A\n\n```shell,script(name=\"write\")\necho hi >mine.txt\n```\n",
        );
        let spec_b = write_spec_file(
            dir.path(),
            "b.md",
            "# B\n\n```shell,script(name=\"check\", expected_exit_code=1)\ntest -e mine.txt\n```\n",
        );
        let file_reader = FileReader::new(dir.path().to_path_buf());

        let factory = Box::new(ShellExecutorFactory {
            shell_cmd: "bash -c".to_string(),
            base_env: Vec::new(),
            unset_env: Vec::new(),
            paths: Vec::new(),
        });

        let cmd = make_per_spec_run_command(
            vec![spec_a, spec_b],
            factory,
            dir.path().to_path_buf(),
            file_reader,
            4,
            None,
        );
        let printer = null_printer();
        let events = cmd.execute_with_printer(&printer);

        let exit_code = exit_code::from_events(&events);
        assert_eq!(
            exit_code as i32, 0,
            "spec B should not see the file created by spec A's own workspace"
        );
    }
}
