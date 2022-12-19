---
layout: page
---

# Displaying Help

You can run SpecDown with no sub-commands and it will display the help.

``` shell
specdown
```

## Non-Windows Output

    A tool to test markdown files and drive development from documentation.
    
    Usage: specdown [OPTIONS] <COMMAND>
    
    Commands:
      completion  Output completion for a shell of your choice
      run         Runs a given Markdown Specification
      strip       Outputs a version of the markdown with all specdown functions removed
      help        Print this message or the help of the given subcommand(s)
    
    Options:
          --no-colour  Disables coloured output
      -h, --help       Print help information
      -V, --version    Print version information

## Windows Output

    A tool to test markdown files and drive development from documentation.
    
    Usage: specdown [OPTIONS] <COMMAND>
    
    Commands:
      completion  Output completion for a shell of your choice
      run         Runs a given Markdown Specification
      strip       Outputs a version of the markdown with all specdown functions removed
      help        Print this message or the help of the given subcommand(s)
      
    Options:
          --no-colour  Disables coloured output
      -h, --help       Print help information
      -V, --version    Print version information

## Sub-commands

You can also run a specific sub-command with the `--help` argument for help on that sub-command.
For example:

``` shell
specdown strip --help
```

Displays:

    Outputs a version of the markdown with all specdown functions removed
    
    Usage: specdown strip <SPEC_FILE>
    
    Arguments:
      <SPEC_FILE>  The spec file to strip specdown functions from
    
    Options:
      -h, --help  Print help information

