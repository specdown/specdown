---
layout: docs
---

# Jobs Flag

The `--jobs` / `-j` flag controls how many specs can run in parallel.

## Default value

When `--jobs` is not specified, the default is `1` (sequential execution).

```shell
specdown run --help 2>&1 | grep '\-\-jobs'
```

```text
  -j, --jobs <JOBS>
```

## Test spec file

We need a simple spec file to run the jobs flag against:

````markdown
# Simple Spec

```shell,script(name="simple_test",expected_exit_code=0)
echo hello
```

```text,verify(script_name="simple_test")
hello
```
````

## Explicit job count

Specifying `--jobs 4` should be accepted:

```shell
specdown run --jobs 4 --temporary-workspace-dir simple_spec.md
```

## Short flag

The `-j` short flag should also work:

```shell
specdown run -j 2 --temporary-workspace-dir simple_spec.md
```

## Zero means parallel

Specifying `-j 0` means "use all CPUs" and should be accepted:

```shell
specdown run -j 0 --temporary-workspace-dir simple_spec.md
```

## Negative values are rejected

Negative values should produce an error:

```shell
specdown run --jobs -1 --temporary-workspace-dir simple_spec.md
```

```text
error: invalid value '-1' for '--jobs <JOBS>': -1 is not in 0..=4294967295

For more information, try '--help'.
```

## Parallel execution with multiple spec files

When `--jobs > 1`, multiple spec files should run in parallel.
We create two spec files and run them together with `-j 2`:

````markdown
# Spec A

```shell,script(name="spec_a_test", expected_exit_code=0)
echo "from spec a"
```
````

````markdown
# Spec B

```shell,script(name="spec_b_test", expected_exit_code=0)
echo "from spec b"
```
````

```shell
specdown run -j 2 --temporary-workspace-dir parallel_spec_a.md parallel_spec_b.md
```

## Parallel execution preserves failure reporting

When running multiple specs in parallel and one fails, the exit code
should be non-zero:

````markdown
# Passing Spec

```shell,script(name="passing_test", expected_exit_code=0)
echo ok
```
````

````markdown
# Failing Spec

```shell,script(name="failing_test", expected_exit_code=0)
exit 1
```
````

```shell
specdown run -j 2 --temporary-workspace-dir passing_spec.md failing_spec.md
```

