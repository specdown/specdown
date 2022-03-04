---
layout: page
---

# Displaying Help

You can run SpecDown with no sub-commands and it will display the help.

``` shell
specdown
```

Outputs:

    specdown 1.1.10
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

## Sub-commands

You can also run a specific sub-command with the `--help` argument for help on that sub-command.
For example:

``` shell
specdown strip --help
```

Displays:

    specdown.exe-strip 
    Outputs a version of the markdown with all specdown functions removed
    
    USAGE:
        specdown.exe strip <spec-file>
    
    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information
    
    ARGS:
        <spec-file>    The spec file to strip specdown functions from

