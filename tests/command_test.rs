use assert_cmd::Command;
use indoc::formatdoc;

#[test]
fn test_readme() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("--no-colour")
        .arg("run")
        .arg("--running-dir")
        .arg(".specdown")
        .arg("README.md")
        .assert()
        .success();
}

#[test]
fn test_doc_index() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("--no-colour")
        .arg("run")
        .arg("--running-dir")
        .arg(".specdown")
        .arg("docs/index.md")
        .assert()
        .success();
}

#[cfg(not(windows))]
#[test]
fn test_doc_display_help() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("--no-colour")
        .arg("run")
        .arg("--running-dir")
        .arg(".specdown")
        .arg("docs/cli/display_help.md")
        .assert()
        .success();
}

#[cfg(windows)]
#[test]
fn test_doc_display_help() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("--no-colour")
        .arg("run")
        .arg("--running-dir")
        .arg(".specdown")
        .arg("docs/cli/display_help_windows.md")
        .assert()
        .success();
}

#[cfg(not(windows))]
#[test]
fn test_doc_running_specs() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("--no-colour")
        .arg("run")
        .arg("--running-dir")
        .arg(".specdown")
        .arg("docs/cli/running_specs.md")
        .assert()
        .success();
}

#[cfg(windows)]
#[test]
fn test_doc_running_specs() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("--no-colour")
        .arg("run")
        .arg("--running-dir")
        .arg(".specdown")
        .arg("docs/cli/running_specs_windows.md")
        .assert()
        .success();
}

#[test]
fn test_doc_creating_test_files() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("--no-colour")
        .arg("run")
        .arg("--running-dir")
        .arg(".specdown")
        .arg("docs/specs/creating_test_files.md")
        .assert()
        .success();
}

#[test]
fn test_doc_verifying_script_output() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("--no-colour")
        .arg("run")
        .arg("--running-dir")
        .arg(".specdown")
        .arg("docs/specs/verifying_script_output.md")
        .assert()
        .success();
}

#[test]
fn test_doc_verifying_exit_codes() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("--no-colour")
        .arg("run")
        .arg("--running-dir")
        .arg(".specdown")
        .arg("docs/specs/verifying_exit_codes.md")
        .assert()
        .success();
}

#[test]
fn test_doc_output_expectations() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("--no-colour")
        .arg("run")
        .arg("--running-dir")
        .arg(".specdown")
        .arg("docs/specs/output_expectations.md")
        .assert()
        .success();
}

#[cfg(not(windows))]
#[test]
fn test_doc_errors() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("--no-colour")
        .arg("run")
        .arg("--running-dir")
        .arg(".specdown")
        .arg("docs/errors.md")
        .assert()
        .success();
}

#[cfg(windows)]
#[test]
fn test_doc_errors() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("--no-colour")
        .arg("run")
        .arg("--running-dir")
        .arg(".specdown")
        .arg("docs/errors_windows.md")
        .assert()
        .success();
}

#[test]
fn test_doc_skipping_code_blocks() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("--no-colour")
        .arg("run")
        .arg("--running-dir")
        .arg(".specdown")
        .arg("docs/specs/skipping_code_blocks.md")
        .assert()
        .success();
}

#[cfg(not(windows))]
#[test]
fn test_doc_stripping_specs() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("--no-colour")
        .arg("run")
        .arg("--running-dir")
        .arg(".specdown")
        .arg("docs/cli/stripping_specs.md")
        .assert()
        .success();
}

#[cfg(windows)]
#[test]
fn test_doc_stripping_specs() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("--no-colour")
        .arg("run")
        .arg("--running-dir")
        .arg(".specdown")
        .arg("docs/cli/stripping_specs_windows.md")
        .assert()
        .success();
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
            specdown 0.48.0
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
