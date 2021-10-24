# Global Environment Variables

When running scripts, some global environment variables are made available. All
these variables are prefixed with `SPECDOWN_`.

## `SPECDOWN_WORKING_DIR`

This environment variable contains the path to the directory where specdown is
running `script` actions.

This can be demonstrated with the following spec:

~~~markdown,file(path="check_running_dir.md")
# Check Running Directory

Verify that `SPECDOWN_WORKING_DIR` is set the current working directory.

```shell,script(name="check_working_directory", expected_exit_code=0)
echo "pwd: $(pwd)"
echo "SPECDOWN_WORKING_DIR: $SPECDOWN_WORKING_DIR"
test "$(pwd)" -ef "$SPECDOWN_WORKING_DIR"
```
~~~

Works with current working dir:

```shell,script(name="specdown_working_dir_with_cwd", expected_exit_code=0)
specdown run check_running_dir.md
```

Works with `--running-dir`:

```shell,script(name="specdown_working_dir_with_running_dir", expected_exit_code=0)
mkdir specific_running_dir
specdown run --running-dir specific_running_dir check_running_dir.md
```

Works with `--temporary-running-dir`:

```shell,script(name="specdown_working_dir_with_temp_running_dir", expected_exit_code=0)
specdown run --temporary-running-dir check_running_dir.md
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

```shell,script(name="check_working_directory")
cat "$SPECDOWN_START_DIR/file_in_start_dir.md"
```

```text,verify()
This file is in the start dir
```
~~~

We can now run that spec file in a temporary running directory and see that it
passes:

```shell,script(name="check_file_in_start_dir")
specdown run --temporary-running-dir check_file_in_start_dir.md
```

```text,verify()
Running tests for check_file_in_start_dir.md:

  ✓ running script 'check_working_directory' succeeded
  ✓ verifying stdout from 'check_working_directory' succeeded

  2 functions run (2 succeeded / 0 failed)

```
