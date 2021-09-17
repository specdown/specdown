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

The you will see the following output:

```text,verify(script_name="verify_example")
Running tests for verify_example.md:

  - running script 'stdout_and_stderr' succeeded
  - verifying stdout from 'stdout_and_stderr' succeeded
  - verifying stderr from 'stdout_and_stderr' failed
===
< left / > right
<Good stderr
<
>Bad stderr
>

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

The you will see the following output:

```text,verify(script_name="omit_name_example")
Running tests for omit_name_example.md:

  - running script '<unnamed>' succeeded
  - verifying stdout from '<unnamed>' succeeded
  - running script 'script_with_name' succeeded
  - verifying stdout from 'script_with_name' succeeded

  4 functions run (4 succeeded / 0 failed)

```
