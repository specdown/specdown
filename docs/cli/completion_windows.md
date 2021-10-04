---
layout: page
---

# Completions

Completions make it a little easier to run specdown, allowing you to press tab and have a half typed command be completed. Completions can be generated in specdown by running the completion command.

## Example

### Bash

We support assorted shells

``` shell
specdown completion bash
```

To load it run

``` shell
source <(specdown completion bash)
```

### Zsh

``` shell
specdown completion zsh
```

To load it run

``` shell
source <(specdown completion zsh)
```

### Fish

``` shell
specdown completion fish
```

To load it run

``` shell
specdown completion fish > ~/.config/fish/completions/specdown.fish
```

### PowerShell

``` shell
specdown completion powershell
```

To load it run

``` shell
. <(specdown completion powershell)
```

### Elvish

``` shell
specdown completion elvish
```

## Command Help

You can display all the options available by using `--help` on the `completion` sub-command.

``` shell
specdown completion --help
```

``` text
specdown.exe-completion 
Output completion for a shell of your choice

USAGE:
    specdown.exe completion <completion-shell>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <completion-shell>    The shell to generate completions for [possible values: bash, fish, elvish, powershell,
                          zsh]
```

