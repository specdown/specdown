# Jobs Flag

The `--jobs` / `-j` flag controls how many specs can run in parallel.

## Default value

When `--jobs` is not specified, the default is `1` (sequential execution).

```shell,script(name="jobs_default")
specdown run --help 2>&1 | grep '\-\-jobs'
```

```text,verify(script_name="jobs_default")
  -j, --jobs <JOBS>
```

## Test spec file

We need a simple spec file to run the jobs flag against:

~~~markdown,file(path="simple_spec.md")
# Simple Spec

```shell,script(name="simple_test",expected_exit_code=0)
echo hello
```

```text,verify(script_name="simple_test")
hello
```
~~~

## Explicit job count

Specifying `--jobs 4` should be accepted:

```shell,script(name="jobs_explicit", expected_exit_code=0)
specdown run --jobs 4 --temporary-workspace-dir simple_spec.md
```

## Short flag

The `-j` short flag should also work:

```shell,script(name="jobs_short", expected_exit_code=0)
specdown run -j 2 --temporary-workspace-dir simple_spec.md
```

## Zero means parallel

Specifying `-j 0` means "use all CPUs" and should be accepted:

```shell,script(name="jobs_zero", expected_exit_code=0)
specdown run -j 0 --temporary-workspace-dir simple_spec.md
```

## Negative values are rejected

Negative values should produce an error:

```shell,script(name="jobs_negative", expected_exit_code=2)
specdown run --jobs -1 --temporary-workspace-dir simple_spec.md
```

```text,verify(script_name="jobs_negative", stream=stderr)
error: invalid value '-1' for '--jobs <JOBS>': -1 is not in 0..=4294967295

For more information, try '--help'.
```

## Parallel execution with multiple spec files

When `--jobs > 1`, multiple spec files should run in parallel.
We create two spec files and run them together with `-j 2`:

~~~markdown,file(path="parallel_spec_a.md")
# Spec A

```shell,script(name="spec_a_test", expected_exit_code=0)
echo "from spec a"
```
~~~

~~~markdown,file(path="parallel_spec_b.md")
# Spec B

```shell,script(name="spec_b_test", expected_exit_code=0)
echo "from spec b"
```
~~~

```shell,script(name="parallel_multiple", expected_exit_code=0)
specdown run -j 2 --temporary-workspace-dir parallel_spec_a.md parallel_spec_b.md
```

## Parallel execution preserves failure reporting

When running multiple specs in parallel and one fails, the exit code
should be non-zero:

~~~markdown,file(path="passing_spec.md")
# Passing Spec

```shell,script(name="passing_test", expected_exit_code=0)
echo ok
```
~~~

~~~markdown,file(path="failing_spec.md")
# Failing Spec

```shell,script(name="failing_test", expected_exit_code=0)
exit 1
```
~~~

```shell,script(name="parallel_failure", expected_exit_code=1)
specdown run -j 2 --temporary-workspace-dir passing_spec.md failing_spec.md
```
