# Verifying Script Output

You can verify that a script returns a specific output by using the `verify()` function.
When verifying you can specify a stream; this can either be `stdout` or `stderr`.
If no `stream` argument is provided then `stdout` is used.

## Example

Given the file `verify_example.md`:

~~~markdown,file(path="verify_example.md")
# Verify Example

Run a script which outputs some stdout and stderr.

```shell,script(name="stdout_and_stderr")
echo "Good stdout"
echo "Bad stderr" 1>&2
```

Verify the stdout:

```text,verify(script_name="stdout_and_stderr", stream=stdout)
Good stdout
```

Verify the stderr:

```text,verify(script_name="stdout_and_stderr", stream=stderr)
Good stderr
```
~~~

When you run the following:

```shell,script(name="verify_example", expected_exit_code=1)
specdown run verify_example.md
```

Then you will see the following output:

```text,verify(script_name="verify_example")
Running tests for verify_example.md:

  ✓ running script 'stdout_and_stderr' succeeded
  ✓ verifying stdout from 'stdout_and_stderr' succeeded
  ✗ verifying stderr from 'stdout_and_stderr' failed
===
< expected / > actual
<Good stderr
>Bad stderr

===

  3 functions run (2 succeeded / 1 failed)

```

## Omitting the script name

If you leave out the `script_name` argument then `verify` will test
the output of of the last script run in the file. You can also omit
the `name` argument on `script` if you don't intent to reference it.

Given the file `omit_name_example.md`:

~~~markdown,file(path="omit_name_example.md")
# Omitting The Script Name Example

Run a script with no name:

```shell,script()
echo "Script with no name!"
```

Verify the output:

```text,verify()
Script with no name!
```

```shell,script(name="script_with_name")
echo "Script with name!"
```

Verify the output:

```text,verify()
Script with name!
```
~~~

When you run the following:

```shell,script(name="omit_name_example", expected_exit_code=0)
specdown run omit_name_example.md
```

Then you will see the following output:

```text,verify(script_name="omit_name_example")
Running tests for omit_name_example.md:

  ✓ running script '<unnamed>' succeeded
  ✓ verifying stdout from '<unnamed>' succeeded
  ✓ running script 'script_with_name' succeeded
  ✓ verifying stdout from 'script_with_name' succeeded

  4 functions run (4 succeeded / 0 failed)

```

## Making OS Specific verifications

An operating system can be specified for the verification to apply to. This is limited to the [values provided by rust](https://doc.rust-lang.org/std/env/consts/constant.OS.html)

Given the file `os_specific.md`:

~~~markdown,file(path="os_specific.md")
# OS Specific verifiction

Run a script with no name:

```shell,script(name="os_specific")
specdown -h
```

Verify the output:

```text,verify(script_name="os_specific",target_os="windows")
specdown 1.2.5
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
```

```text,verify(script_name="os_specific",target_os="linux")
specdown 1.2.5
A tool to test markdown files and drive devlopment from documentation.

USAGE:
    specdown [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help         Prints help information
        --no-colour    Disables coloured output
    -V, --version      Prints version information

SUBCOMMANDS:
    completion    Output completion for a shell of your choice
    help          Prints this message or the help of the given subcommand(s)
    run           Runs a given Markdown Specification
    strip         Outputs a version of the markdown with all specdown functions removed
```

```text,verify(script_name="os_specific",target_os="macos")
specdown 1.2.5
A tool to test markdown files and drive devlopment from documentation.

USAGE:
    specdown [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help         Prints help information
        --no-colour    Disables coloured output
    -V, --version      Prints version information

SUBCOMMANDS:
    completion    Output completion for a shell of your choice
    help          Prints this message or the help of the given subcommand(s)
    run           Runs a given Markdown Specification
    strip         Outputs a version of the markdown with all specdown functions removed
```
~~~

When you run the following:

```shell,script(name="os_specific", expected_exit_code=0)
specdown run os_specific.md
```

Then you will see the following output:

```text,verify(script_name="os_specific")
Running tests for os_specific.md:

  ✓ running script 'os_specific' succeeded
  ✓ verifying stdout from 'os_specific' succeeded

  2 functions run (2 succeeded / 0 failed)

```

You may also negate the target os
Given the file `os_specific_negation.md`:

~~~markdown,file(path="os_specific_negation.md")
# OS Specific nagative verifiction

Run a script with no name:

```shell,script(name="os_specific_negation")
specdown -h
```

Verify the output:

```text,verify(script_name="os_specific_negation",target_os="!windows")
specdown 1.2.5
A tool to test markdown files and drive devlopment from documentation.

USAGE:
    specdown [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help         Prints help information
        --no-colour    Disables coloured output
    -V, --version      Prints version information

SUBCOMMANDS:
    completion    Output completion for a shell of your choice
    help          Prints this message or the help of the given subcommand(s)
    run           Runs a given Markdown Specification
    strip         Outputs a version of the markdown with all specdown functions removed
```

```text,verify(script_name="os_specific_negation",target_os="windows")
specdown 1.2.5
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
```
~~~

When you run the following:

```shell,script(name="os_specific_negation", expected_exit_code=0)
specdown run os_specific_negation.md
```

Then you will see the following output:

```text,verify(script_name="os_specific_negation")
Running tests for os_specific_negation.md:

  ✓ running script 'os_specific_negation' succeeded
  ✓ verifying stdout from 'os_specific_negation' succeeded

  2 functions run (2 succeeded / 0 failed)

```
