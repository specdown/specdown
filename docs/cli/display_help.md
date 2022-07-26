---
layout: page
---

# Displaying Help

You can run SpecDown with no sub-commands and it will display the help.

``` shell
specdown
```

## Non-Windows Output

    specdown 1.2.24
    A tool to test markdown files and drive development from documentation.
    
    USAGE:
        specdown [OPTIONS] <SUBCOMMAND>
    
    OPTIONS:
        -h, --help         Print help information
            --no-colour    Disables coloured output
        -V, --version      Print version information
    
    SUBCOMMANDS:
        completion    Output completion for a shell of your choice
        help          Print this message or the help of the given subcommand(s)
        run           Runs a given Markdown Specification
        strip         Outputs a version of the markdown with all specdown functions removed

## Windows Output

    specdown 1.2.24
    A tool to test markdown files and drive development from documentation.
    
    USAGE:
        specdown [OPTIONS] <SUBCOMMAND>
    
    OPTIONS:
        -h, --help         Print help information
            --no-colour    Disables coloured output
        -V, --version      Print version information
    
    SUBCOMMANDS:
        completion    Output completion for a shell of your choice
        help          Print this message or the help of the given subcommand(s)
        run           Runs a given Markdown Specification
        strip         Outputs a version of the markdown with all specdown functions removed

## Sub-commands

You can also run a specific sub-command with the `--help` argument for help on that sub-command.
For example:

``` shell
specdown strip --help
```

Displays:

    specdown-strip 
    Outputs a version of the markdown with all specdown functions removed
    
    USAGE:
        specdown strip <SPEC_FILE>
    
    ARGS:
        <SPEC_FILE>    The spec file to strip specdown functions from
    
    OPTIONS:
        -h, --help    Print help information

