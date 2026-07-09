# Container Executor

Specdown supports running scripts inside a Docker container via the Docker
Engine socket API, as an alternative to the default shell executor.

## Default Executor

When no `--executor` flag is provided, the shell executor is used.

Given the file `default_executor.md`:

~~~markdown,file(path="default_executor.md")
# Default Executor Test

```shell,script(name="hello")
echo "hello"
```
~~~

```shell,script(name="default_executor")
specdown run --executor shell default_executor.md
```

```text,verify(script_name="default_executor")
Running tests for default_executor.md:

  ✓ running script 'hello' succeeded

  1 functions run (1 succeeded / 0 failed)

```

## Shell Executor Explicit

You can explicitly select the shell executor.

Given the file `shell_executor.md`:

~~~markdown,file(path="shell_executor.md")
# Shell Executor Test

```shell,script(name="hello")
echo "hello"
```
~~~

```shell,script(name="shell_executor")
specdown run --executor shell shell_executor.md
```

```text,verify(script_name="shell_executor")
Running tests for shell_executor.md:

  ✓ running script 'hello' succeeded

  1 functions run (1 succeeded / 0 failed)

```
