# Errors

There are a number of errors that can occur when the if a markdown spec contains some invalid content.

## Unknown Function

Given `unknown_function_example.md`:

~~~markdown,file(path="unknown_function_example.md")
# Unknown Function Example

```shell,function()
echo "This script is annotated with an unknown function"
```
~~~

Running the following command will fail:

```shell,script(name="unknown_function_example", expected_exit_code=1)
specdown run unknown_function_example.md
```

With the following error message:

```text,verify(script_name="unknown_function_example")
Running tests for unknown_function_example.md:

  - Unknown function: function
```

## Missing Function Arguments

Given `missing_function_argument_example.md`:

~~~markdown,file(path="missing_function_argument_example.md")
# Unknown Function Example

```shell,script()
echo "This script is missing a name"
```
~~~

Running the following command will fail:

```shell,script(name="missing_function_argument_example", expected_exit_code=1)
specdown run missing_function_argument_example.md
```

With the following error message:

```text,verify(script_name="missing_function_argument_example")
Running tests for missing_function_argument_example.md:

  - Function script requires argument name
```

## Invalid Argument Value

Given `invalid_argument_value_example.md`:

~~~markdown,file(path="invalid_argument_value_example.md")
# Unknown Function Example

```shell,script(name=123)
echo "This script has an integer name"
```
~~~

Running the following command will fail:

```shell,script(name="invalid_argument_value_example", expected_exit_code=1)
specdown run invalid_argument_value_example.md
```

With the following error message:

```text,verify(script_name="invalid_argument_value_example")
Running tests for invalid_argument_value_example.md:

  - Function script requires argument name to be a string, got integer
```

## Invalid Argument Option

Given `invalid_token_option_example.md`:

~~~markdown,file(path="invalid_token_option_example.md")
# Unknown Function Example

```shell,script(name="script")
echo "This script will work"
```

```text,verify(script_name="script", stream=unknown)
unknown is not a valid stream
```
~~~

Running the following command will fail:

```shell,script(name="invalid_token_option_example", expected_exit_code=1)
specdown run invalid_token_option_example.md
```

With the following error message:

```text,verify(script_name="invalid_token_option_example")
Running tests for invalid_token_option_example.md:

  - Argument stream for function verify must be output, stdout or stderr, got unknown
```

## Verify Unknown Script

Given `verify_unknown_script_example.md`:

~~~markdown,file(path="verify_unknown_script_example.md")
# Verify Unknown Script Example

```text,verify(script_name="unknown")
This doesn't matter
```
~~~

Running the following command will fail:

```shell,script(name="verify_unknown_script_example", expected_exit_code=2)
specdown run verify_unknown_script_example.md
```

With the following error message:

```text,verify(script_name="verify_unknown_script_example")
Running tests for verify_unknown_script_example.md:

  - Failed to verify the output of 'unknown': No script with that name has been executed yet.
```


## Empty Shell Command

Given `empty_shell_command_example.md`:

~~~markdown,file(path="empty_shell_command_example.md")
# Nothing to see here
~~~

Running the following command will fail:

```shell,script(name="empty_shell_command_example", expected_exit_code=2)
specdown run --shell-command '' empty_shell_command_example.md
```

With the following error message:

```text,verify(script_name="empty_shell_command_example")
Running tests for empty_shell_command_example.md:

  - Invalid shell command provided:  (Error: Command is empty)
```

## Invalid Shell Command

Given `invalid_shell_command_example.md`:

~~~markdown,file(path="invalid_shell_command_example.md")
# Nothing to see here
~~~

Running the following command will fail:

```shell,script(name="invalid_shell_command_example", expected_exit_code=2)
specdown run --shell-command 'invalid " command' invalid_shell_command_example.md
```

With the following error message:

```text,verify(script_name="invalid_shell_command_example")
Running tests for invalid_shell_command_example.md:

  - Invalid shell command provided: invalid " command (Error: Parse error : missing closing quote)
```

## Shell Which Can't Be Run

Given `missing_shell_example.md`:

~~~markdown,file(path="missing_shell_example.md")
# Hello World

```shell,script(name="test")
echo "Hello world"
```
~~~

Running the following command will fail:

```shell,script(name="missing_shell_example", expected_exit_code=2)
specdown run --shell-command 'does-not-exist' missing_shell_example.md
```

With the following error message:

```text,verify(script_name="missing_shell_example")
Running tests for missing_shell_example.md:

  - Failed to run command: does-not-exist [] (Error: The system cannot find the file specified. (os error 2))
```
