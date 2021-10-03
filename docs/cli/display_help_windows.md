# Displaying Help

You can run SpecDown with no sub-commands and it will display the help.

```shell,script(name="with-no-args")
specdown
```

Outputs:

```,verify(script_name="with-no-args", stream=stderr)
specdown 0.54.0
A tool to test markdown files and drive devlopment from documentation.

USAGE:
    specdown.exe [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help         Prints help information
        --no-colour    Disables coloured output
    -V, --version      Prints version information

SUBCOMMANDS:
    completion    Output completion for a shell of your choice
    help          Prints this message or the help of the given subcommand(s)
    run           Runs a given Markdown Specification
    strip         Outputs a version of the markdown with all specdown functions removed
```

## Sub-commands

You can also run a specific sub-command with the `--help` argument for help on that sub-command.
For example:

```shell,script(name="run-with-help")
specdown run --help
```

Displays:

```,verify(script_name="run-with-help")
specdown.exe-run 
Runs a given Markdown Specification

USAGE:
    specdown.exe run [FLAGS] [OPTIONS] <spec-files>...

FLAGS:
    -h, --help                     Prints help information
        --temporary-running-dir    Create a temporary directory to run the scripts in
    -V, --version                  Prints version information

OPTIONS:
        --add-path <add-path>...           Adds the given directory to PATH
        --env <env>...                     Set an environment variable (format: 'VAR_NAME=value')
        --running-dir <running-dir>        The directory where commands will be executed
        --shell-command <shell-command>    The shell command used to execute script blocks [default: bash -c]
        --unset-env <unset-env>...         Unset an environment variable

ARGS:
    <spec-files>...    The spec files to run
```
