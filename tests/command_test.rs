use assert_cmd::Command;
use indoc::indoc;

#[test]
fn test_displays_help() {
    Command::cargo_bin("specdown").unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(
            indoc!("specdown 
            A tool to test markdown files and drive devlopment from documentation.

            USAGE:
                specdown --output-file <output-file> --spec-file <spec-file>

            FLAGS:
                -h, --help       Prints help information
                -V, --version    Prints version information

            OPTIONS:
                    --output-file <output-file>    The generated output file
                    --spec-file <spec-file>        The spec file to run
            ")
        );
}

#[test]
fn test_displays_error_when_required_args_are_missing() {
    Command::cargo_bin("specdown").unwrap()
        .assert()
        .failure()
        .stderr(
            indoc!("error: The following required arguments were not provided:
                --output-file <output-file>
                --spec-file <spec-file>
            
            USAGE:
                specdown --output-file <output-file> --spec-file <spec-file>
            
            For more information try --help
            ")
        );
}