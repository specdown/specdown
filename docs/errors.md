---
layout: page
---
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

```text,verify(script_name="unknown_function_example", stream=stdout)
Unknown function: function
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

```text,verify(script_name="missing_function_argument_example", stream=stdout)
Function script requires argument name
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

```text,verify(script_name="invalid_argument_value_example", stream=stdout)
Function script requires argument name to be a string, got integer
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

```text,verify(script_name="invalid_token_option_example", stream=stdout)
Argument stream for function verify must be output, stdout or stderr, got unknown
```
