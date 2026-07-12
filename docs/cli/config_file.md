# Configuration File

Instead of passing every flag to `specdown run` on the command line, you can
put the same settings under a `[run]` table in a `specdown.toml` file.

- If a `specdown.toml` file exists in the current directory, it's loaded
  automatically.
- A different location can be loaded with the global `--config <PATH>` flag.
- Any value given on the command line overrides the same value from the file.

## Loading Settings Automatically

If `specdown.toml` exists in the current directory when `specdown run` is
invoked, its `[run]` table is used without needing any flags.

```shell,script(name="default_discovery_setup")
mkdir -p default-discovery
```

Given a `specdown.toml` that sets an environment variable:

```toml,file(path="default-discovery/specdown.toml")
[run.env]
GREETING = "World"
```

And a spec file that depends on that variable:

~~~markdown,file(path="default-discovery/greet.md")
# Greet

```shell,script(name="greet")
echo "Hello, $GREETING"
```

```text,verify(script_name="greet")
Hello, World
```
~~~

Running `specdown run` with no flags, from the directory containing
`specdown.toml`, picks up the `GREETING` environment variable from the file:

```shell,script(name="default_discovery_example")
cd default-discovery && specdown --no-colour run greet.md
```

```text,verify(script_name="default_discovery_example")
Running tests for greet.md:

  ✓ running script 'greet' succeeded
  ✓ verifying stdout from 'greet' succeeded

  2 functions run (2 succeeded / 0 failed)

```

## Overriding File Settings from the Command Line

Any setting passed on the command line takes priority over the same setting
in `specdown.toml`.

```shell,script(name="cli_overrides_file_setup")
mkdir -p cli-overrides-file
```

Given the same `specdown.toml` as above (setting `GREETING` to `World` via
`[run.env]`), but a spec expecting a different greeting:

~~~markdown,file(path="cli-overrides-file/greet.md")
# Greet

```shell,script(name="greet")
echo "Hello, $GREETING"
```

```text,verify(script_name="greet")
Hello, Specdown
```
~~~

```toml,file(path="cli-overrides-file/specdown.toml")
[run.env]
GREETING = "World"
```

Passing `--env` on the command line overrides the file's `env` setting
entirely:

```shell,script(name="cli_overrides_file_example")
cd cli-overrides-file && specdown --no-colour run --env GREETING=Specdown greet.md
```

```text,verify(script_name="cli_overrides_file_example")
Running tests for greet.md:

  ✓ running script 'greet' succeeded
  ✓ verifying stdout from 'greet' succeeded

  2 functions run (2 succeeded / 0 failed)

```

## Loading a Config File from a Specific Location

The global `--config <PATH>` flag loads settings from a specific file
instead of looking for `specdown.toml` in the current directory. This works
even when no `specdown.toml` exists in the current directory at all.

```shell,script(name="explicit_config_setup")
mkdir -p explicit-config/configs
```

```toml,file(path="explicit-config/configs/custom.toml")
[run.env]
GREETING = "Explicit"
```

~~~markdown,file(path="explicit-config/greet.md")
# Greet

```shell,script(name="greet")
echo "Hello, $GREETING"
```

```text,verify(script_name="greet")
Hello, Explicit
```
~~~

```shell,script(name="explicit_config_example")
cd explicit-config && specdown --no-colour --config configs/custom.toml run greet.md
```

```text,verify(script_name="explicit_config_example")
Running tests for greet.md:

  ✓ running script 'greet' succeeded
  ✓ verifying stdout from 'greet' succeeded

  2 functions run (2 succeeded / 0 failed)

```

## A Complete Example

The `[run]` table accepts every setting `specdown run` supports on the
command line. Here's a single `specdown.toml` combining most of them:

```shell,script(name="full_example_setup")
mkdir -p full-example/bin
echo "echo 'CUSTOM COMMAND OUTPUT'" >full-example/bin/custom-cmd
chmod +x full-example/bin/custom-cmd
```

```toml,file(path="full-example/specdown.toml")
[run]
workspace_dir = "."
working_dir = "."
workspace_init_command = "echo initialised > marker.txt"
shell_command = "bash -c"
unset_env = ["UNSET_ME"]
add_path = ["bin"]
jobs = 1

[run.env]
GREETING = "World"

[run.executor]
executor = "shell"
```

- `workspace_dir` / `working_dir` set the directory commands run in
  (`"."` here as a no-op, just to show the syntax — see
  [Running Specs](running_specs.md) for non-trivial values).
- `workspace_init_command` runs once before the specs; here it writes
  `marker.txt`.
- `shell_command` picks the shell used to run script blocks.
- `[run.env]` sets environment variables as a table (see above), while
  `unset_env` removes one from the inherited environment.
- `add_path` prepends a directory to `$PATH`.
- `jobs` controls how many spec files run in parallel.
- `[run.executor]` selects the executor backend (`shell`, shown here, or
  `container` — see below).

A spec that exercises all of the above:

~~~markdown,file(path="full-example/example.md")
# Full Example

```shell,script(name="example")
custom-cmd
echo "Hello, $GREETING"
cat marker.txt
env | grep -c '^UNSET_ME=' || true
```

```text,verify(script_name="example")
CUSTOM COMMAND OUTPUT
Hello, World
initialised
0
```
~~~

Running it, with `UNSET_ME` set beforehand so `unset_env` has something to
remove:

```shell,script(name="full_example_example")
cd full-example && UNSET_ME=1 specdown --no-colour run example.md
```

```text,verify(script_name="full_example_example")
Running tests for example.md:

  ✓ running script 'example' succeeded
  ✓ verifying stdout from 'example' succeeded

  2 functions run (2 succeeded / 0 failed)

```

Note that `workspace_dir` and `temporary_workspace_dir` can't be set
together (specdown will error), so use one or the other.

### The Container Executor

`executor = "container"` runs scripts inside a Docker container instead of
the host shell. It requires specdown to be built with the `container`
feature and a running Docker daemon, so it isn't exercised here — but the
config looks like this:

```toml
[run.executor]
executor = "container"
container_image = "bash:5"
container_volumes = ["/host/data:/data"]
```

## Errors in the Config File

An unknown key under `[run]` (for example, a typo) is treated as an error
rather than being silently ignored:

```shell,script(name="bad_config_setup")
mkdir -p bad-config
```

```toml,file(path="bad-config/bad.toml")
[run]
shell_comand = "typo"
```

```shell,script(name="bad_config_example", expected_exit_code=2)
cd bad-config && specdown --no-colour --config bad.toml run
```

```text,verify(script_name="bad_config_example")
  ✗ Failed to load config file 'bad.toml': TOML parse error at line 2, column 1
  |
2 | shell_comand = "typo"
  | ^^^^^^^^^^^^
unknown field `shell_comand`, expected one of `files`, `workspace_dir`, `temporary_workspace_dir`, `working_dir`, `workspace_init_command`, `shell_command`, `env`, `unset_env`, `add_path`, `jobs`, `executor`, `follow_links`, `workspace_per_spec`

```
