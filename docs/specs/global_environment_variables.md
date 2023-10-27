# Global Environment Variables

When running scripts, some global environment variables are made available. All
these variables are prefixed with `SPECDOWN_`.

## `SPECDOWN_WORKSPACE_DIR`

This environment variable contains the path to the directory where running
context directory exists.

This can be demonstrated with the following spec:

~~~markdown,file(path="check_workspace_dir.md")
# Check Workspace Directory

Verify that `SPECDOWN_WORKSPACE_DIR` is set the current workspace directory.

```shell,script(name="check_workspace_directory", expected_exit_code=0)
echo "pwd: $(pwd)"
echo "SPECDOWN_WORKSPACE_DIR: $SPECDOWN_WORKSPACE_DIR"
test "$(pwd)" -ef "$SPECDOWN_WORKSPACE_DIR"
```
~~~

Works with current workspace dir:

```shell,script(name="specdown_workspace_dir_with_cwd", expected_exit_code=0)
specdown run check_workspace_dir.md
```

Works with `--workspace-dir`:

```shell,script(name="specdown_workspace_dir_with_running_dir", expected_exit_code=0)
mkdir specific_workspace_dir
specdown run --workspace-dir specific_workspace_dir check_workspace_dir.md
```

Works with `--temporary-workspace-dir`:

```shell,script(name="specdown_workspace_dir_with_temp_workspace_dir", expected_exit_code=0)
specdown run --temporary-workspace-dir check_workspace_dir.md
```

## `SPECDOWN_WORKING_DIR`

This environment variable contains the path to the directory where specdown is
running `script` actions.

This can be demonstrated with the following spec:

~~~markdown,file(path="check_working_dir.md")
# Check Running Directory

Verify that `SPECDOWN_WORKING_DIR` is set the current workspace directory.

```shell,script(name="check_workspace_directory", expected_exit_code=0)
echo "pwd: $(pwd)"
echo "SPECDOWN_WORKING_DIR: $SPECDOWN_WORKING_DIR"
test "$(pwd)" -ef "$SPECDOWN_WORKING_DIR"
```
~~~

When running with `--working-dir` set:

```shell,script(name="specdown_working_dir_with_workspace_dir", expected_exit_code=0)
mkdir -p workspace/specific_working_dir
specdown run --workspace-dir workspace --working-dir specific_working_dir check_working_dir.md
```

### `SPECDOWN_START_DIR`

This environment variable contains the path to where specdown was run from. This
is useful if you need to access files in the project repository direction but
have changed running directory.

To demonstrate this, we can create a file called `file_in_start_dir.md`:

~~~markdown,file(path="file_in_start_dir.md")
This file is in the start dir
~~~

And a spec file which checks the content of that file:

~~~markdown,file(path="check_file_in_start_dir.md")
# Check File Contents

```shell,script(name="check_workspace_directory")
cat "$SPECDOWN_START_DIR/file_in_start_dir.md"
```

```text,verify()
This file is in the start dir
```
~~~

We can now run that spec file in a temporary running directory and see that it
passes:

```shell,script(name="check_file_in_start_dir")
specdown run --temporary-workspace-dir check_file_in_start_dir.md
```

```text,verify()
Running tests for check_file_in_start_dir.md:

  ✓ running script 'check_workspace_directory' succeeded
  ✓ verifying stdout from 'check_workspace_directory' succeeded

  2 functions run (2 succeeded / 0 failed)

```
