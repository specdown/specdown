---
layout: page
---

# Output Expectations

You can add `expected_output` on the `run()` to cause it to fail if there is any
unexpected output. Valid values for `expected_output` are:

- `any` (default)
- `stdout`
- `stderr`
- `none`

## `any`

Given the file `output_expectation_any.md`:

```` markdown
# Any Example

Run a script which outputs some stdout and stderr.

```shell,script(name="stdout_and_stderr", expected_output=any)
echo "Good stdout"
echo "Good stderr" 1>&2
```
````

When you run the following:

``` shell
specdown run output_expectation_any.md
```

Then you will see the following output:

``` text
Running tests for output_expectation_any.md:

  ✓ running script 'stdout_and_stderr' succeeded

  1 functions run (1 succeeded / 0 failed)

```

## `stdout`

Given the file `output_expectation_stdout.md`:

```` markdown
# StdOut Example

Run a script which outputs some stdout and stderr.

```shell,script(name="stdout", expected_output=stdout)
echo "Good stdout"
```

```shell,script(name="stdout_and_stderr", expected_output=stdout)
echo "More good stdout"
echo "Bad stderr" 1>&2
```
````

When you run the following:

``` shell
specdown run output_expectation_stdout.md
```

Then you will see the following output:

``` text
Running tests for output_expectation_stdout.md:

  ✓ running script 'stdout' succeeded
  ✗ running script 'stdout_and_stderr' failed (unexpected stderr)

=== stdout:
More good stdout


=== stderr:
Bad stderr




  2 functions run (1 succeeded / 1 failed)

```

## `stderr`

Given the file `output_expectation_stderr.md`:

```` markdown
# StdErr Example

Run a script which outputs some stdout and stderr.

```shell,script(name="stderr", expected_output=stderr)
echo "Good stderr" 1>&2
```

```shell,script(name="stdout_and_stderr", expected_output=stderr)
echo "Bad stdout"
echo "More good stderr" 1>&2
```
````

When you run the following:

``` shell
specdown run output_expectation_stderr.md
```

Then you will see the following output:

``` text
Running tests for output_expectation_stderr.md:

  ✓ running script 'stderr' succeeded
  ✗ running script 'stdout_and_stderr' failed (unexpected stdout)

=== stdout:
Bad stdout


=== stderr:
More good stderr




  2 functions run (1 succeeded / 1 failed)

```

## `none`

Given the file `output_expectation_none.md`:

```` markdown
# None Example

Run a script which outputs some stdout and stderr.

```shell,script(name="no_output", expected_output=none)
exit 0
```

```shell,script(name="stdout", expected_output=none)
echo "Bad stdout"
```

```shell,script(name="stderr", expected_output=none)
echo "Bad stderr" 1>&2
```
````

When you run the following:

``` shell
specdown run output_expectation_none.md
```

Then you will see the following output:

``` text
Running tests for output_expectation_none.md:

  ✓ running script 'no_output' succeeded
  ✗ running script 'stdout' failed (unexpected output)

=== stdout:
Bad stdout


=== stderr:



  ✗ running script 'stderr' failed (unexpected output)

=== stdout:


=== stderr:
Bad stderr




  3 functions run (1 succeeded / 2 failed)

```

