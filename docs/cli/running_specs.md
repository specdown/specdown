---
layout: page
---

# Running Specs

Markdown specs are run by executing the `specdown run <spec-files>`.

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

  ✓ running script 'command_1' succeeded
  ✓ verifying stdout from 'command_1' succeeded

  2 functions run (2 succeeded / 0 failed)

```

## Multiple Files

Given a file `example-file1.md`

```` markdown
# This is a spec

```shell,script(name="command_1")
echo "Spec 1"
```
````

Given a file `example-file2.md`

```` markdown
# This is another spec

```shell,script(name="command_2")
echo "Spec 2"
```
````

You can run:

``` shell
specdown run example-file1.md example-file2.md
```

And you will get the following output:

``` text
Running tests for example-file1.md:

  ✓ running script 'command_1' succeeded

  1 functions run (1 succeeded / 0 failed)

Running tests for example-file2.md:

  ✓ running script 'command_2' succeeded

  1 functions run (1 succeeded / 0 failed)

```

## Setting the Working Directory

There are three directories that you need to know about when running specdown:

1.  The **start directory**: The current working directory when specdown was run
    (e.g. this might be the root of your repository).
2.  The **workspace directory**: A directory with context which is required to
    run the scripts. This defaults to the **start directory**.
3.  The **working directory**: A sub-directory of the workspace directory where
    script actions will be run. This defaults to the **workspace directory**.

### Setting a Specific Working Directory: `--workspace-dir`

You can set the directory for the commands to be executed in using the
`--workspace-dir` argument.

To demonstrate this, we can make a new directory with a file in it:

``` shell
mkdir workspace_dir
echo "file in workspace dir" >workspace_dir/test_file.txt
```

And we can create a spec called `workspace_dir_example.md`:

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
file in workspace dir
```
````

Now we can run specdown using the following command:

``` shell
specdown run --workspace-dir workspace_dir workspace_dir_example.md
```

``` text
Running tests for workspace_dir_example.md:

  ✓ running script 'ls' succeeded
  ✓ verifying stdout from 'ls' succeeded
  ✓ running script 'cat' succeeded
  ✓ verifying stdout from 'cat' succeeded

  4 functions run (4 succeeded / 0 failed)

```

### Using a Temporary Working Directory: `--temporary-workspace-directory`

You ask specdown to create a temporary workspace directory for the commands to be
executed in using the `--temporary-workspace-directory` argument.

And we can create a spec called `temporary_workspace_dir_example.md`:

```` markdown
# Demo Spec

## Listing a file which does not exist

`test -e` will return `1` if the file does not exist.

```shell,script(name="ls", expected_exit_code=1)
test -e "missing-file.txt"
```
````

Now we can run specdown using the following command:

``` shell
echo "this file is in the current workspace directory" >missing-file.txt
specdown run --temporary-workspace-dir temporary_workspace_dir_example.md
```

``` text
Running tests for temporary_workspace_dir_example.md:

  ✓ running script 'ls' succeeded

  1 functions run (1 succeeded / 0 failed)

```

### Setting the Working Directory: `--working-dir`

The working directory is a sub-directory or the workspace where script actions
are run.

If we create a file called `workspace/working_dir/message.txt`:

``` shell
mkdir -p workspace/working_dir
echo "Message Example" >workspace/working_dir/message.txt
```

And a spec file which checks the contents of a `message.txt`

```` markdown
# Working Directory Example

```shell,script(name="cat_message_file")
cat message.txt
```

```text,verify()
Message Example
```
````

We can set the working directoy by calling specdown with the following arguments:

``` shell
specdown run --workspace-dir workspace --working-dir working_dir working_dir_example.md
```

## Setting the Shell

By default, specdown runs commands with `bash -c`. You can override this with
the `--shell-command` option.

To demonstrate this, let's take the following `setting_the_shell_example.md` spec:

```` markdown
# Setting the Shell Example

```shell,script(name="get_shell_name")
basename "$0"
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

  ✓ running script 'get_shell_name' succeeded
  ✗ verifying stdout from 'get_shell_name' failed
===
< expected / > actual
<bash
>sh

===

  2 functions run (1 succeeded / 1 failed)

```

## Environment

### Setting Environment Variables

You can provide environment variable to the `run` command. These variables are
then available in all `script` actions:

```` markdown
# Setting Environment Variables Example

```shell,script(name="environment_variables")
echo "$GREETING, $SUBJECT"
```

```text,verify(script_name="environment_variables")
Hello, World
```
````

``` shell
specdown run --env 'GREETING=Hello' --env 'SUBJECT=World' environment_variables.md
```

### Unsetting Environment Variables

You can provide environment variable to the `run` command. These variables are
then available in all `script` actions:

```` markdown
# Unsetting Variables Example

`grep` will return an exit code of `1` if it does not find the string it is
searching for. The follow script asserts that `UNSET_ME` is not set.

```shell,script(expected_exit_code=1)
env | grep UNSET_ME
```
````

``` shell
UNSET_ME=1 specdown run --unset-env UNSET_ME unset_environment_variables.md
```

### Adding to `$PATH`

If you want addition paths to be added to your running environment you can use
`--add-path`. To demonstrate this, let's create a couple of scripts in different
directories:

``` shell
mkdir -p vendor/bin
echo "echo 'PHP COMMAND OUTPUT'" >vendor/bin/php-cmd
chmod +x vendor/bin/php-cmd

mkdir -p node_modules/.bin
echo "echo 'NODE COMMAND OUTPUT'" >node_modules/.bin/node-cmd
chmod +x node_modules/.bin/node-cmd
```

The we can write a spec to test these scripts:

```` markdown
# Add Path Example

```shell,script(expected_exit_code=0)
set -e

php-cmd
node-cmd
echo "ECHO OUTPUT"
```

```text,verify()
PHP COMMAND OUTPUT
NODE COMMAND OUTPUT
ECHO OUTPUT
```
````

Now we can run the spec providing the additional paths:

``` shell
specdown run --add-path "$PWD/vendor/bin" --add-path "$PWD/node_modules/.bin" add_path.md
```

## Command Help

You can display all the options available by using `--help` on the `run`
sub-command.

``` shell
specdown run --help
```

``` text
specdown-run 
Runs a given Markdown Specification

USAGE:
    specdown run [FLAGS] [OPTIONS] <spec-files>...

FLAGS:
    -h, --help                       Prints help information
        --temporary-workspace-dir    Create a temporary workspace directory
    -V, --version                    Prints version information

OPTIONS:
        --add-path <add-path>...           Adds the given directory to PATH
        --env <env>...                     Set an environment variable (format: 'VAR_NAME=value')
        --shell-command <shell-command>    The shell command used to execute script blocks [default: bash -c]
        --unset-env <unset-env>...         Unset an environment variable
        --working-dir <working-dir>        The directory where commands will be executed. This is relative to the
                                           workspace dir
        --workspace-dir <workspace-dir>    Set the workspace directory

ARGS:
    <spec-files>...    The spec files to run
```

