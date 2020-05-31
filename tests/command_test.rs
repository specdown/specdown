use assert_cmd::Command;
use indoc::indoc;

#[test]
fn test_displays_help() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(indoc!(
            "specdown 
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

#[test]
fn test_readme() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("README.md")
        .assert()
        .success();
}

#[test]
fn test_doc_index() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("README.md")
        .assert()
        .success();
}

#[test]
fn test_doc_display_help() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("doc/display_help.md")
        .assert()
        .success();
}

#[test]
fn test_doc_files() {
    Command::cargo_bin("specdown")
        .unwrap()
        .arg("run")
        .arg("doc/files.md")
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
            "specdown 
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
