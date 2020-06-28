use assert_cmd::Command;
use indoc::indoc;

#[test]
fn test_readme() {
    Command::cargo_bin("specdown")
        .unwrap()
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
        .arg("run")
        .arg("--running-dir")
        .arg(".specdown")
        .arg("README.md")
        .assert()
        .success();
}

#[test]
fn test_doc_display_help() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--running-dir")
        .arg(".specdown")
        .arg("docs/display_help.md")
        .assert()
        .success();
}

#[test]
fn test_doc_running_specs() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--running-dir")
        .arg(".specdown")
        .arg("docs/running_specs.md")
        .assert()
        .success();
}

#[test]
fn test_doc_creating_test_files() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--running-dir")
        .arg(".specdown")
        .arg("docs/creating_test_files.md")
        .assert()
        .success();
}

#[test]
fn test_verifying_script_output() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--running-dir")
        .arg(".specdown")
        .arg("docs/verifying_script_output.md")
        .assert()
        .success();
}

#[test]
fn test_verifying_exit_codes() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--running-dir")
        .arg(".specdown")
        .arg("docs/verifying_exit_codes.md")
        .assert()
        .success();
}

#[test]
fn test_errors() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--running-dir")
        .arg(".specdown")
        .arg("docs/errors.md")
        .assert()
        .success();
}

#[test]
fn test_skipping_code_blocks() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("--running-dir")
        .arg(".specdown")
        .arg("docs/skipping_code_blocks.md")
        .assert()
        .success();
}

#[test]
fn test_displays_error_when_required_args_are_missing() {
    Command::cargo_bin("specdown")
        .unwrap()
        .assert()
        .failure()
        .stderr(indoc!(
            "
            specdown 0.25.0
            A tool to test markdown files and drive devlopment from documentation.
            
            USAGE:
                specdown [SUBCOMMAND]
            
            FLAGS:
                -h, --help       Prints help information
                -V, --version    Prints version information
            
            SUBCOMMANDS:
                help    Prints this message or the help of the given subcommand(s)
                run     Runs a given Markdown Specification.
            "
        ));
}
