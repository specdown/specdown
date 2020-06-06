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

```text,verify(script_name="command_1", stream=output)
Hello world
```
~~~

You can run:

```shell,script(name="run_example")
target/debug/specdown run example-spec.md
```

And you will get the following output:

```text,verify(script_name="run_example", stream=output)
Script command_1 succeeded
Verify output from command_1 succeeded
```

## Command Help

You can display all the options available by using `--help` on the `run` sub-command.

```shell,script(name="run_help")
target/debug/specdown run --help 2>&1
```

```text,verify(script_name="run_help", stream=output)
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

