# Displaying Help

You can run SpecDown with no sub-commands and it will display the help.

```shell,script(name="with-no-args")
specdown
```

## Non-Windows Output

```,verify(stream=stderr,target_os="!windows")
A tool to test markdown files and drive development from documentation.

Usage: specdown [OPTIONS] <COMMAND>

Commands:
  completion  Output completion for a shell of your choice
  run         Runs a given Markdown Specification
  strip       Outputs a version of the markdown with all specdown functions removed
  help        Print this message or the help of the given subcommand(s)

Options:
      --no-colour  Disables coloured output
  -h, --help       Print help
  -V, --version    Print version
```

## Windows Output

```,verify(stream=stderr,target_os="windows")
A tool to test markdown files and drive development from documentation.

Usage: specdown [OPTIONS] <COMMAND>

Commands:
  completion  Output completion for a shell of your choice
  run         Runs a given Markdown Specification
  strip       Outputs a version of the markdown with all specdown functions removed
  help        Print this message or the help of the given subcommand(s)
  
Options:
      --no-colour  Disables coloured output
  -h, --help       Print help
  -V, --version    Print version
```

## Sub-commands

You can also run a specific sub-command with the `--help` argument for help on that sub-command.
For example:

```shell,script(name="strip-with-help")
specdown strip --help
```

Displays:

```,verify()
Outputs a version of the markdown with all specdown functions removed

Usage: specdown strip <SPEC_FILE>

Arguments:
  <SPEC_FILE>  The spec file to strip specdown functions from

Options:
  -h, --help  Print help
```
