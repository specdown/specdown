# Verifying Exit Codes

You can add an `expected_exit_code` option to a `script()` function if you want to assert it exists with a particular value.

Given the file `exit_example.md`:

~~~markdown,file(path="exit_example.md")
# Example of testing exit codes

The following will success:

```shell,script(name="command_1", expected_exit_code=25)
exit 25
```

But the next one will fail:

```shell,script(name="command_2", expected_exit_code=0)
exit 1
```
~~~

When you run:

```shell,script(name="exit_example")
specdown run exit_example.md
```

Then you'll see:

```text,verify(script_name="exit_example", stream=stdout)
Running tests for exit_example.md:
  - script 'command_1' succeeded
  - script 'command_2' failed (expected exitcode 0, got 1)
=== stdout:


=== stderr:



```
