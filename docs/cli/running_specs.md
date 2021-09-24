# Running Specs

Markdown specs are run by executing the `specdown run <spec-files>`.

## Example

Given a file `example-spec.md`

~~~markdown,file(path="example-spec.md")
# This is a spec

```shell,script(name="command_1")
echo "Hello world"
```

Outputs:

```text,verify(script_name="command_1")
Hello world
```
~~~

You can run:

```shell,script(name="run_example")
specdown run example-spec.md
```

And you will get the following output:

```text,verify(script_name="run_example")
Running tests for example-spec.md:

  ✓ running script 'command_1' succeeded
  ✓ verifying stdout from 'command_1' succeeded

  2 functions run (2 succeeded / 0 failed)

```

## Multiple Files

Given a file `example-file1.md`

~~~markdown,file(path="example-file1.md")
# This is a spec

```shell,script(name="command_1")
echo "Spec 1"
```
~~~

Given a file `example-file2.md`

~~~markdown,file(path="example-file2.md")
# This is another spec

```shell,script(name="command_2")
echo "Spec 2"
```
~~~

You can run:

```shell,script(name="run_example")
specdown run example-file1.md example-file2.md
```

And you will get the following output:

```text,verify(script_name="run_example")
Running tests for example-file1.md:

  ✓ running script 'command_1' succeeded

  1 functions run (1 succeeded / 0 failed)

Running tests for example-file2.md:

  ✓ running script 'command_2' succeeded

  1 functions run (1 succeeded / 0 failed)

```

## Setting the Running Directory

You can set the directory for the commands to be executed in using the
`--running-dir` argument.

To demonstrate this, we can make a new directory with a file in it:

```shell,script(name="running_dir_file_setup")
mkdir running_dir
echo "file in working dir" >running_dir/test_file.txt
```

And we can create a spec called `running_dir_example.md`:

~~~markdown,file(path="running_dir_example.md")
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
~~~

Now we can run specdown using the following command:

```shell,script(name="running_dir_example")
specdown run --running-dir running_dir running_dir_example.md
```

```text,verify(script_name="running_dir_example")
Running tests for running_dir_example.md:

  ✓ running script 'ls' succeeded
  ✓ verifying stdout from 'ls' succeeded
  ✓ running script 'cat' succeeded
  ✓ verifying stdout from 'cat' succeeded

  4 functions run (4 succeeded / 0 failed)

```

## Setting the Shell

By default, specdown runs commands with `bash -c`. You can override this with
the `--shell-command` option.

To demonstrate this, let's take the following `setting_the_shell_example.md` spec:

~~~markdown,file(path="setting_the_shell_example.md")
# Setting the Shell Example

```shell,script(name="get_shell_name")
basename "$0"
```

```text,verify(script_name="get_shell_name")
bash
```
~~~

This will succeed when we run the following:

```shell,script(name="setting_the_shell_example_bash", expected_exit_code=0)
specdown run setting_the_shell_example.md
```

However, if we now run the following command it will fail:

```shell,script(name="setting_the_shell_example_sh", expected_exit_code=1)
specdown run --shell-command 'sh -c' setting_the_shell_example.md
```

And it will give the following output:

```text,verify(script_name="setting_the_shell_example_sh")
Running tests for setting_the_shell_example.md:

  ✓ running script 'get_shell_name' succeeded
  ✗ verifying stdout from 'get_shell_name' failed
===
< left / > right
<bash
<
>sh
>

===

  2 functions run (1 succeeded / 1 failed)

```

## Environment

### Setting Environment Variables

You can provide environment variable to the `run` command. These variables are
then available in all `script` actions:

~~~markdown,file(path="environment_variables.md")
# Setting Environment Variables Example

```shell,script(name="environment_variables")
echo "$GREETING, $SUBJECT"
```

```text,verify(script_name="environment_variables")
Hello, World
```
~~~

```shell,script(name="run_with_environment_variables", expected_exit_code=0)
specdown run --env 'GREETING=Hello' --env 'SUBJECT=World' environment_variables.md
```

### Unsetting Environment Variables

You can provide environment variable to the `run` command. These variables are
then available in all `script` actions:

~~~markdown,file(path="unset_environment_variables.md")
# Unsetting Variables Example

`grep` will return an exit code of `1` if it does not find the string it is
searching for. The follow script asserts that `UNSET_ME` is not set.

```shell,script(expected_exit_code=1)
env | grep UNSET_ME
```
~~~

```shell,script(name="run_with_unset_environment_variables", expected_exit_code=0)
UNSET_ME=1 specdown run --unset-env UNSET_ME unset_environment_variables.md
```

### Adding to `$PATH`

If you want addition paths to be added to your running environment you can use
`--add-path`. To demonstrate this, let's create a couple of scripts in different
directories:

```shell,script(name="create_scripts", expected_exit_code=0)
mkdir -p vendor/bin
echo "echo 'PHP COMMAND OUTPUT'" >vendor/bin/php-cmd
chmod +x vendor/bin/php-cmd

mkdir -p node_modules/.bin
echo "echo 'NODE COMMAND OUTPUT'" >node_modules/.bin/node-cmd
chmod +x node_modules/.bin/node-cmd
```

The we can write a spec to test these scripts:

~~~markdown,file(path="add_path.md")
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
~~~

Now we can run the spec providing the additional paths:

```shell,script(expected_exit_code=0)
specdown run --add-path "$PWD/vendor/bin" --add-path "$PWD/node_modules/.bin" add_path.md
```

## Command Help

You can display all the options available by using `--help` on the `run`
sub-command.

```shell,script(name="run_help")
specdown run --help
```

```text,verify(script_name="run_help")
specdown-run 
Runs a given Markdown Specification

USAGE:
    specdown run [OPTIONS] <spec-files>...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --add-path <add-path>...           Adds the given directory to PATH
        --env <env>...                     Set an environment variable (format: 'VAR_NAME=value')
        --running-dir <running-dir>        The directory where commands will be executed
        --shell-command <shell-command>    The shell command used to execute script blocks [default: bash -c]
        --unset-env <unset-env>...         Unset an environment variable

ARGS:
    <spec-files>...    The spec files to run
```

