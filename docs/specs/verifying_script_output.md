---
layout: page
---
# Verifying Script Output

You can verify that a script returns a specific output by using the `verify()` function.
When verifying you have to specify a stream; this can either be `stdout` or `stderr`.

## Example

Given the file `verify_example.md`:

```` markdown
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
````

When you run the following:

``` shell
specdown run verify_example.md
```

The you will see the following output:

``` text
Running tests for verify_example.md:
  - script 'stdout_and_stderr' succeeded
  - verify stdout from 'stdout_and_stderr' succeeded
  - verify stderr from 'stdout_and_stderr' failed
===
< left / > right
<Good stderr
<
>Bad stderr
>

===
```

