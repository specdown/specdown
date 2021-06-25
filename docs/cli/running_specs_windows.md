---
layout: page
---

# Running Specs

Markdown specs are run by executing the `specdown run <spec-file>`.

## Example

Given a file `example-spec.md`

```` markdown
# This is a spec

```shell,script(name="command_1")
echo "Hello world"
```

Outputs:

```text,verify(script_name="command_1")
Hello world
```
````

You can run:

``` shell
specdown run example-spec.md
```

And you will get the following output:

``` text
Running tests for example-spec.md:

  - script 'command_1' succeeded
  - verify stdout from 'command_1' succeeded

  2 functions run (2 succeeded / 0 failed)
```

## Setting the Running Directory

You can set the directory for the commands to be executed in using the `--running-dir` argument.

To demontstrate this, we can make a new directory with a file in it:

``` shell
mkdir running_dir
echo "file in working dir" >running_dir/test_file.txt
```

And we can create a spec called `running_dir_example.md`:

```` markdown
# Demo Spec

## Listing the directory

```shell,script(name="ls")
ls
```

```text,verify(script_name="ls")
test_file.txt
```

## Displaying the file contents

```shell,script(name="cat")
cat test_file.txt
```

```text,verify(script_name="cat")
file in working dir
```
````

Now we can run specdown using the following command:

``` shell
specdown run --running-dir running_dir running_dir_example.md
```

``` text
Running tests for running_dir_example.md:

  - script 'ls' succeeded
  - verify stdout from 'ls' succeeded
  - script 'cat' succeeded
  - verify stdout from 'cat' succeeded

  4 functions run (4 succeeded / 0 failed)
```

## Setting the Shell

By default, specdown runs commands with `bash -c`. You can override this with the `--shell-command` option.

To demonstrate this, let's take the following `setting_the_shell_example.md` spec:

```` markdown
# Setting the Shell Example

```shell,script(name="get_shell_name")
echo $0
```

```text,verify(script_name="get_shell_name")
bash
```
````

This will succeed when we run the following:

``` shell
specdown run setting_the_shell_example.md
```

However, if we now run the following command it will fail:

``` shell
specdown run --shell-command 'sh -c' setting_the_shell_example.md
```

And it will give the following output:

``` text
Running tests for setting_the_shell_example.md:

  - script 'get_shell_name' succeeded
  - verify stdout from 'get_shell_name' failed
===
< left / > right
<bash
<
>sh
>

===

  2 functions run (1 succeeded / 1 failed)
```

## Command Help

You can display all the options available by using `--help` on the `run` sub-command.

``` shell
specdown run --help
```

``` text
specdown.exe-run 
Runs a given Markdown Specification

USAGE:
    specdown.exe run [OPTIONS] <spec-file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --running-dir <running-dir>        The directory where commands will be executed
        --shell-command <shell-command>    The shell command used to execute script blocks [default: bash -c]

ARGS:
    <spec-file>    The spec file to run
```

