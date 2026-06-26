---
layout: page
---

# Background Scripts

Specifications can run scripts in the background that are automatically stopped
at the end of the spec file. This is useful for starting servers or other
long-running processes that subsequent scripts interact with.

You start a background script using the `background` function.

## Example

Given the file `background_example.md`:

````markdown
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
````

When you run the following:

```shell
specdown run background_example.md
```

Then you will see the following output:

```text
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

````markdown
# Background Stopped Example

```shell,background(name="long_running")
sleep 60
```
````

When you run the following:

```shell
specdown run background_stopped.md
```

Then you will see the following output:

```text
Running tests for background_stopped.md:

  ✓ starting background script 'long_running' succeeded
  ✓ stopping background script 'long_running' succeeded

  2 functions run (2 succeeded / 0 failed)

```

## Background scripts without a name

If you omit the `name` argument, the background script will be shown as
`<unnamed>` in the output.

Given the file `background_unnamed.md`:

````markdown
# Background Unnamed Example

```shell,background()
sleep 60
```
````

When you run the following:

```shell
specdown run background_unnamed.md
```

Then you will see the following output:

```text
Running tests for background_unnamed.md:

  ✓ starting background script '<unnamed>' succeeded
  ✓ stopping background script '<unnamed>' succeeded

  2 functions run (2 succeeded / 0 failed)

```

