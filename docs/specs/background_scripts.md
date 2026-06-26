# Background Scripts

Specifications can run scripts in the background that are automatically stopped
at the end of the spec file. This is useful for starting servers or other
long-running processes that subsequent scripts interact with.

You start a background script using the `background` function.

## Example

Given the file `background_example.md`:

~~~markdown,file(path="background_example.md")
# Background Example

Start a background server that listens for connections:

```shell,background(name="server")
while true; do
  echo "server ready" > server_output.txt
  sleep 60
done
```

Wait for the server to be ready, then verify the output:

```shell,script(name="check_server")
sleep 1
cat server_output.txt
```

```text,verify(script_name="check_server")
server ready
```
~~~

When you run the following:

```shell,script(name="background_example", expected_exit_code=0)
specdown run background_example.md
```

Then you will see the following output:

```text,verify(script_name="background_example")
Running tests for background_example.md:

  ✓ starting background script 'server' succeeded
  ✓ running script 'check_server' succeeded
  ✓ verifying stdout from 'check_server' succeeded
  ✓ stopping background script 'server' succeeded

  4 functions run (4 succeeded / 0 failed)

```

## Background scripts are stopped at the end of the spec

Even if there are no more scripts after the background script starts, it will
be stopped when the spec file completes.

Given the file `background_stopped.md`:

~~~markdown,file(path="background_stopped.md")
# Background Stopped Example

```shell,background(name="long_running")
sleep 60
```
~~~

When you run the following:

```shell,script(name="background_stopped", expected_exit_code=0)
specdown run background_stopped.md
```

Then you will see the following output:

```text,verify(script_name="background_stopped")
Running tests for background_stopped.md:

  ✓ starting background script 'long_running' succeeded
  ✓ stopping background script 'long_running' succeeded

  2 functions run (2 succeeded / 0 failed)

```

## Background scripts without a name

If you omit the `name` argument, the background script will be shown as
`<unnamed>` in the output.

Given the file `background_unnamed.md`:

~~~markdown,file(path="background_unnamed.md")
# Background Unnamed Example

```shell,background()
sleep 60
```
~~~

When you run the following:

```shell,script(name="background_unnamed", expected_exit_code=0)
specdown run background_unnamed.md
```

Then you will see the following output:

```text,verify(script_name="background_unnamed")
Running tests for background_unnamed.md:

  ✓ starting background script '<unnamed>' succeeded
  ✓ stopping background script '<unnamed>' succeeded

  2 functions run (2 succeeded / 0 failed)

```

## Background script that exits before the spec file ends

If a background script exits on its own (with a zero exit code) before the spec
file ends, specdown detects the early exit and reports it as succeeded. The
process is not killed because it has already exited.

Given the file `background_exits.md`:

~~~markdown,file(path="background_exits.md")
# Background Exits Example

```shell,background(name="quick_exit")
echo "done"
```

```shell,script(name="check_exit")
sleep 1
```
~~~

When you run the following:

```shell,script(name="background_exits", expected_exit_code=0)
specdown run background_exits.md
```

Then you will see the following output:

```text,verify(script_name="background_exits")
Running tests for background_exits.md:

  ✓ starting background script 'quick_exit' succeeded
  ✓ running script 'check_exit' succeeded
  ✓ stopping background script 'quick_exit' succeeded

  3 functions run (3 succeeded / 0 failed)

```

## Background script that exits with a non-zero exit code

If a background script exits on its own with a non-zero exit code before the
spec file ends, specdown detects the crash and reports the stop as failed.

Given the file `background_crash.md`:

~~~markdown,file(path="background_crash.md")
# Background Crash Example

```shell,background(name="crashing")
exit 1
```

```shell,script(name="check_crash")
sleep 1
```
~~~

When you run the following:

```shell,script(name="background_crash", expected_exit_code=1)
specdown run background_crash.md
```

Then you will see the following output:

```text,verify(script_name="background_crash")
Running tests for background_crash.md:

  ✓ starting background script 'crashing' succeeded
  ✓ running script 'check_crash' succeeded
  ✗ stopping background script 'crashing' failed (exited with code 1)

  3 functions run (2 succeeded / 1 failed)

```

## Background script that is still running at spec end

If a background script is still running when the spec file ends, specdown kills
the process and reports the stop as succeeded. This is the expected behavior for
long-running processes like servers.

Given the file `background_killed.md`:

~~~markdown,file(path="background_killed.md")
# Background Killed Example

```shell,background(name="long_running")
sleep 60
```
~~~

When you run the following:

```shell,script(name="background_killed", expected_exit_code=0)
specdown run background_killed.md
```

Then you will see the following output:

```text,verify(script_name="background_killed")
Running tests for background_killed.md:

  ✓ starting background script 'long_running' succeeded
  ✓ stopping background script 'long_running' succeeded

  2 functions run (2 succeeded / 0 failed)

```
