use assert_cmd::output::OutputResult;
use assert_cmd::Command;
use indoc::formatdoc;

fn assert_ok(result: &OutputResult) {
    let output = match result {
        Ok(out) => out,
        Err(err) => err.as_output().expect("failed to get output from error"),
    };

    println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("Error:\n{}", String::from_utf8_lossy(&output.stderr));

    assert!(result.is_ok());
}

/// Build a `specdown run` command with `--add-path` pointing at the debug binary
/// so integration tests can find the freshly-built specdown binary.
fn specdown_run_with_path() -> Command {
    let bin_dir = std::env::current_dir()
        .expect("failed to get current directory")
        .join("target")
        .join("debug");
    let mut cmd = Command::cargo_bin("specdown").expect("failed to find specdown cargo binary");
    cmd.arg("run")
        .arg("--temporary-workspace-dir")
        .arg("--add-path")
        .arg(
            bin_dir
                .to_str()
                .expect("failed to convert bin_dir to UTF-8 string"),
        );
    cmd
}

#[test]
fn test_readme() {
    let result = specdown_run_with_path().arg("README.md").ok();

    assert_ok(&result);
}

#[test]
fn test_doc_index() {
    let result = Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--temporary-workspace-dir")
        .arg("docs/index.md")
        .ok();

    assert_ok(&result);
}

#[cfg(not(windows))]
#[test]
fn test_doc_display_help() {
    let result = specdown_run_with_path()
        .arg("docs/cli/display_help.md")
        .ok();

    assert_ok(&result);
}

#[cfg(not(windows))]
#[test]
fn test_doc_running_specs() {
    let result = specdown_run_with_path()
        .arg("docs/cli/running_specs.md")
        .ok();

    assert_ok(&result);
}

#[cfg(not(windows))]
#[test]
fn test_doc_config_file() {
    let result = specdown_run_with_path().arg("docs/cli/config_file.md").ok();

    assert_ok(&result);
}

#[test]
fn test_doc_creating_test_files() {
    let result = specdown_run_with_path()
        .arg("docs/specs/creating_test_files.md")
        .ok();

    assert_ok(&result);
}

#[test]
fn test_doc_verifying_script_output() {
    let result = specdown_run_with_path()
        .arg("docs/specs/verifying_script_output.md")
        .ok();

    assert_ok(&result);
}

#[test]
fn test_doc_verifying_exit_codes() {
    let result = specdown_run_with_path()
        .arg("docs/specs/verifying_exit_codes.md")
        .ok();

    assert_ok(&result);
}

#[test]
fn test_doc_global_environment_variables() {
    let result = specdown_run_with_path()
        .arg("docs/specs/global_environment_variables.md")
        .ok();

    assert_ok(&result);
}

#[test]
fn test_doc_output_expectations() {
    let result = specdown_run_with_path()
        .arg("docs/specs/output_expectations.md")
        .ok();

    assert_ok(&result);
}

#[cfg(not(windows))]
#[test]
fn test_doc_errors() {
    let bin_dir = std::env::current_dir()
        .expect("failed to get current directory")
        .join("target")
        .join("debug");
    let result = Command::cargo_bin("specdown")
        .unwrap()
        .arg("--no-colour")
        .arg("run")
        .arg("--temporary-workspace-dir")
        .arg("--add-path")
        .arg(
            bin_dir
                .to_str()
                .expect("failed to convert bin_dir to UTF-8 string"),
        )
        .arg("docs/errors.md")
        .ok();

    assert_ok(&result);
}

#[test]
fn test_doc_escaped_quotes_in_string_arguments() {
    let bin_dir = std::env::current_dir()
        .unwrap()
        .join("target")
        .join("debug");
    let result = Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--temporary-workspace-dir")
        .arg("--add-path")
        .arg(bin_dir.to_str().unwrap())
        .arg("docs/specs/escaped_quotes_in_string_arguments.md")
        .ok();

    assert_ok(&result);
}

#[test]
fn test_doc_skipping_code_blocks() {
    let result = specdown_run_with_path()
        .arg("docs/specs/skipping_code_blocks.md")
        .ok();

    assert_ok(&result);
}

#[cfg(not(windows))]
#[test]
fn test_doc_completion() {
    let result = specdown_run_with_path().arg("docs/cli/completion.md").ok();

    assert_ok(&result);
}

#[cfg(not(windows))]
#[test]
fn test_doc_stripping_specs() {
    let result = specdown_run_with_path()
        .arg("docs/cli/stripping_specs.md")
        .ok();

    assert_ok(&result);
}

#[test]
fn test_doc_background_scripts() {
    let result = specdown_run_with_path()
        .arg("docs/specs/background_scripts.md")
        .ok();

    assert_ok(&result);
}

#[test]
fn test_doc_container_executor() {
    let result = specdown_run_with_path()
        .arg("docs/specs/container_executor.md")
        .ok();

    assert_ok(&result);
}

#[test]
fn test_displays_error_when_required_args_are_missing() {
    #[cfg(windows)]
    const BINARY_NAME: &str = "specdown.exe";
    #[cfg(not(windows))]
    const BINARY_NAME: &str = "specdown";

    Command::cargo_bin("specdown")
        .unwrap()
        .assert()
        .failure()
        .stderr(formatdoc!(
            "
            A tool to test markdown files and drive development from documentation.

            Usage: {} [OPTIONS] <COMMAND>

            Commands:
              completion  Output completion for a shell of your choice
              run         Runs a given Markdown Specification
              strip       Outputs a version of the markdown with all specdown functions removed
              help        Print this message or the help of the given subcommand(s)

            Options:
                  --no-colour      Disables coloured output
                  --config <PATH>  Load settings from a specific config file instead of looking for `specdown.toml` in the current directory
              -h, --help           Print help
              -V, --version        Print version
            ",
            BINARY_NAME
        ));
}
