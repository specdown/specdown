# Completions

Completions make it a little easier to run specdown, allowing you to press tab and have a half typed command be completed. Completions can be generated in specdown by running the completion command.

## Example

We support assorted shells

### Bash

```shell,script(expected_exit_code=0)
specdown completion bash
```

To load it run 

```shell, skip()
source <(specdown completion bash)
```

### Zsh

```shell,script(expected_exit_code=0)
specdown completion zsh
```

To load it run

```shell, skip()
source <(specdown completion zsh)
```

### Fish

```shell,script(expected_exit_code=0)
specdown completion fish
```

To load it run

```shell, skip()
specdown completion fish > ~/.config/fish/completions/specdown.fish
```

### PowerShell

```shell,script(expected_exit_code=0)
specdown completion powershell
```

To load it run

```shell, skip()
. <(specdown completion powershell)
```

### Elvish

```shell,script(expected_exit_code=0)
specdown completion elvish
```


## Command Help

You can display all the options available by using `--help` on the `completion` sub-command.

```shell,script(name="run_help")
specdown completion --help
```

```text,verify(script_name="run_help")
specdown-completion 
Output completion for a shell of your choice

USAGE:
    specdown completion <completion-shell>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <completion-shell>    The shell to generate completions for [possible values: bash, fish, elvish, powershell,
                          zsh]
```

