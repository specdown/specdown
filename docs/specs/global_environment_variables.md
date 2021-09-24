# Global Environment Variables

When running scripts, some global environment variables are made available. All
these variables are prefixed with `SPECDOWN_`.

## `SPECDOWN_RUNNING_DIR`

This environment variable contains the path to the directory where specdown is
running `script` actions.

This can be demonstrated with the following spec:

~~~markdown,file(path="check_running_dir.md")
# Check Running Directory

Verify that `SPECDOWN_RUNNING_DIR` is set the current working directory.

```shell,script(name="check_working_directory", expected_exit_code=0)
echo "pwd: $(pwd)"
echo "SPECDOWN_RUNNING_DIR: $SPECDOWN_RUNNING_DIR"
test "$(pwd)" -ef "$SPECDOWN_RUNNING_DIR"
```
~~~

Works with current working dir:

```shell,script(name="specdown_running_dir_with_cwd", expected_exit_code=0)
specdown run check_running_dir.md
```

Works with `--running-dir`:

```shell,script(name="specdown_running_dir_with_running_dir", expected_exit_code=0)
mkdir specific_running_dir
specdown run --running-dir specific_running_dir check_running_dir.md
```

Works with `--temporary-running-dir`:

```shell,script(name="specdown_running_dir_with_temp_running_dir", expected_exit_code=0)
specdown run --temporary-running-dir check_running_dir.md
```
