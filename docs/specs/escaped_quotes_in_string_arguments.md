# Escaped Quotes in String Arguments

You can include double quotes inside string arguments by escaping them with a backslash (`\"`).
Because markdown processes backslash escapes, you need to use `\\\"` in the info string
so that comrak produces `\"` for the function string parser.

This allows string values in code block attributes to contain double quote characters.

## Example

Given the file `escaped_quotes_example.md`:

~~~markdown,file(path="escaped_quotes_example.md")
# Escaped Quotes Example

Run a script with a name containing escaped quotes:

```shell,script(name="my \\\"quoted\\\" script")
echo hello
```

Verify the output:

```text,verify(script_name="my \\\"quoted\\\" script")
hello
```
~~~

When you run the following:

```shell,script(name="escaped_quotes_example")
specdown run escaped_quotes_example.md
```

Then you will see the following output:

```text,verify(script_name="escaped_quotes_example")
Running tests for escaped_quotes_example.md:

  ✓ running script 'my "quoted" script' succeeded
  ✓ verifying stdout from 'my "quoted" script' succeeded

  2 functions run (2 succeeded / 0 failed)

```
