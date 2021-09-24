---
layout: page
---

# Errors

There are a number of errors that can occur when the if a markdown spec contains
some invalid content.

## Spec File Errors

### Unknown Function

Given `unknown_function_example.md`:

```` markdown
# Unknown Function Example

```shell,function()
echo "This script is annotated with an unknown function"
```
````

Running the following command will fail:

``` shell
specdown run unknown_function_example.md
```

With the following error message:

``` text
Running tests for unknown_function_example.md:

  ✗ Unknown function: function

  0 functions run (0 succeeded / 0 failed)

```

### Missing Function Arguments

Given `missing_function_argument_example.md`:

```` markdown
# Unknown Function Example

```shell,file()
This file has no path!
```
````

Running the following command will fail:

``` shell
specdown run missing_function_argument_example.md
```

With the following error message:

``` text
Running tests for missing_function_argument_example.md:

  ✗ Function file requires argument path

  0 functions run (0 succeeded / 0 failed)

```

### Invalid Argument Value

Given `invalid_argument_value_example.md`:

```` markdown
# Unknown Function Example

```shell,script(name=123)
echo "This script has an integer name"
```
````

Running the following command will fail:

``` shell
specdown run invalid_argument_value_example.md
```

With the following error message:

``` text
Running tests for invalid_argument_value_example.md:

  ✗ Function script requires argument name to be a string, got integer

  0 functions run (0 succeeded / 0 failed)

```

### Invalid Argument Option

Given `invalid_token_option_example.md`:

```` markdown
# Unknown Function Example

```shell,script(name="script")
echo "This script will work"
```

```text,verify(script_name="script", stream=unknown)
unknown is not a valid stream
```
````

Running the following command will fail:

``` shell
specdown run invalid_token_option_example.md
```

With the following error message:

``` text
Running tests for invalid_token_option_example.md:

  ✗ Argument stream for function verify must be output, stdout or stderr, got unknown

  0 functions run (0 succeeded / 0 failed)

```

### Verify Unknown Script

Given `verify_unknown_script_example.md`:

```` markdown
# Verify Unknown Script Example

```text,verify(script_name="unknown")
This doesn't matter
```
````

Running the following command will fail:

``` shell
specdown run verify_unknown_script_example.md
```

With the following error message:

``` text
Running tests for verify_unknown_script_example.md:

  ✗ Failed to verify the output of 'unknown': No script with that name has been executed yet.

  0 functions run (0 succeeded / 0 failed)

```

## Run Command Errors

### Setting `--running-dir` and `--temporary-running-dir`

Given `empty_spec.md`:

``` markdown
# Nothing to see here
```

Running the following command will fail:

``` shell
specdown run --running-dir dirname --temporary-running-dir empty_shell_command_example.md
```

With the following error message:

``` text
  ✗ --running-dir and --temporary-running-dir cannot be specified at the same time
```

### Shell Command Errors

#### Empty Shell Command

Given `empty_shell_command_example.md`:

``` markdown
# Nothing to see here
```

Running the following command will fail:

``` shell
specdown run --shell-command '' empty_shell_command_example.md
```

With the following error message:

``` text
  ✗ Invalid shell command provided:  (Error: Command is empty)
```

#### Invalid Shell Command

Given `invalid_shell_command_example.md`:

``` markdown
# Nothing to see here
```

Running the following command will fail:

``` shell
specdown run --shell-command 'invalid " command' invalid_shell_command_example.md
```

With the following error message:

``` text
  ✗ Invalid shell command provided: invalid " command (Error: Parse error : missing closing quote)
```

#### Shell Which Can't Be Run

Given `missing_shell_example.md`:

```` markdown
# Hello World

```shell,script(name="test")
echo "Hello world"
```
````

Running the following command will fail:

``` shell
specdown run --shell-command 'does-not-exist' missing_shell_example.md
```

With the following error message:

``` text
Running tests for missing_shell_example.md:

  ✗ Failed to run command: does-not-exist [] (Error: No such file or directory (os error 2))

  0 functions run (0 succeeded / 0 failed)

```

