use assert_cmd::output::OutputResult;
use assert_cmd::Command;
use indoc::formatdoc;

fn assert_ok(result: &OutputResult) {
    let output = match result {
        Ok(out) => out,
        Err(err) => err.as_output().unwrap(),
    };

    println!("Output:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("Error:\n{}", String::from_utf8_lossy(&output.stderr));

    assert!(result.is_ok());
}

#[test]
fn test_readme() {
    let result = Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--temporary-running-dir")
        .arg("README.md")
        .ok();

    assert_ok(&result);
}

#[test]
fn test_doc_index() {
    let result = Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--temporary-running-dir")
        .arg("docs/index.md")
        .ok();

    assert_ok(&result);
}

#[cfg(not(windows))]
#[test]
fn test_doc_display_help() {
    let result = Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--temporary-running-dir")
        .arg("docs/cli/display_help.md")
        .ok();

    assert_ok(&result);
}

#[cfg(windows)]
#[test]
fn test_doc_display_help() {
    let result = Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--temporary-running-dir")
        .arg("docs/cli/display_help_windows.md")
        .ok();

    assert_ok(&result);
}

#[cfg(not(windows))]
#[test]
fn test_doc_running_specs() {
    let result = Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--temporary-running-dir")
        .arg("docs/cli/running_specs.md")
        .ok();

    assert_ok(&result);
}

#[cfg(windows)]
#[test]
fn test_doc_running_specs() {
    let result = Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--temporary-running-dir")
        .arg("docs/cli/running_specs_windows.md")
        .ok();

    assert_ok(&result);
}

#[test]
fn test_doc_creating_test_files() {
    let result = Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--temporary-running-dir")
        .arg("docs/specs/creating_test_files.md")
        .ok();

    assert_ok(&result);
}

#[test]
fn test_doc_verifying_script_output() {
    let result = Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--temporary-running-dir")
        .arg("docs/specs/verifying_script_output.md")
        .ok();

    assert_ok(&result);
}

#[test]
fn test_doc_verifying_exit_codes() {
    let result = Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--temporary-running-dir")
        .arg("docs/specs/verifying_exit_codes.md")
        .ok();

    assert_ok(&result);
}

#[test]
fn test_doc_output_expectations() {
    let result = Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--temporary-running-dir")
        .arg("docs/specs/output_expectations.md")
        .ok();

    assert_ok(&result);
}

#[cfg(not(windows))]
#[test]
fn test_doc_errors() {
    let result = Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--temporary-running-dir")
        .arg("docs/errors.md")
        .ok();

    assert_ok(&result);
}

#[cfg(windows)]
#[test]
fn test_doc_errors() {
    let result = Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--temporary-running-dir")
        .arg("docs/errors_windows.md")
        .ok();

    assert_ok(&result);
}

#[test]
fn test_doc_skipping_code_blocks() {
    let result = Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--temporary-running-dir")
        .arg("docs/specs/skipping_code_blocks.md")
        .ok();

    assert_ok(&result);
}

#[cfg(not(windows))]
#[test]
fn test_doc_stripping_specs() {
    let result = Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--temporary-running-dir")
        .arg("docs/cli/stripping_specs.md")
        .ok();

    assert_ok(&result);
}

#[cfg(windows)]
#[test]
fn test_doc_stripping_specs() {
    let result = Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--temporary-running-dir")
        .arg("docs/cli/stripping_specs_windows.md")
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
            specdown 0.52.0
            A tool to test markdown files and drive devlopment from documentation.

            USAGE:
                {} [FLAGS] [SUBCOMMAND]

            FLAGS:
                -h, --help         Prints help information
                    --no-colour    Disables coloured output
                -V, --version      Prints version information

            SUBCOMMANDS:
                help     Prints this message or the help of the given subcommand(s)
                run      Runs a given Markdown Specification
                strip    Outputs a version of the markdown with all specdown functions removed
            ",
            BINARY_NAME
        ));
}
