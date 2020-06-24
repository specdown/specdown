---
layout: page
---
# Running Specs

Markdown specs are run by executing the `specdown run <spec-file>`.

## Example

Given a file `example-spec.md`

~~~markdown,file(path="example-spec.md")
# This is a spec

```shell,script(name="command_1")
echo "Hello world"
```

Outputs:

```text,verify(script_name="command_1", stream=stdout)
Hello world
```
~~~

You can run:

```shell,script(name="run_example")
specdown run example-spec.md
```

And you will get the following output:

```text,verify(script_name="run_example", stream=stdout)
- script 'command_1' succeeded
- verify output from 'command_1' succeeded
```

## Setting the Running Directory

You can set the directory for the commands to be executed in using the `--running-dir` argument.

To demontstrate this, we can make a new directory with a file in it:

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

```text,verify(script_name="ls", stream=stdout)
test_file.txt
```

## Displaying the file contents

```shell,script(name="cat")
cat test_file.txt
```

```text,verify(script_name="cat", stream=stdout)
file in working dir
```
~~~

Now we can run specdown using the following command:

```shell,script(name="running_dir_example")
specdown run --running-dir running_dir running_dir_example.md
```

```text,verify(script_name="running_dir_example", stream=stdout)
- script 'ls' succeeded
- verify output from 'ls' succeeded
- script 'cat' succeeded
- verify output from 'cat' succeeded
```

## Command Help

You can display all the options available by using `--help` on the `run` sub-command.

```shell,script(name="run_help")
specdown run --help
```

```text,verify(script_name="run_help", stream=stdout)
specdown-run 
Runs a given Markdown Specification.

USAGE:
    specdown run [OPTIONS] <spec-file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --running-dir <running-dir>    The directory where commands will be executed

ARGS:
    <spec-file>    The spec file to run
```

