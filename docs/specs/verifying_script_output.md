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
