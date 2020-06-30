---
layout: page
---
# Stripping Specs

When writing specdown documents, you have to add the addition specdown functions to your markdown.
The syntax used by specdown upsets some markdown parses, so the `strip` command can be used to generate a version of the documents with the specdown specific content removed.

## Example

Give a markdown spec called `strip_example.md`:

~~~markdown,file(path="strip_example.md")
# Strip Example

```shell,script(name="hello_world")
echo "Hello world"
```
~~~

You can run:

```shell, script(name="strip_example")
specdown strip strip_example.md
```

And you'll get get following output:

~~~markdown, verify(script_name="strip_example", stream=stdout)
# Strip Example

``` shell
echo "Hello world"
```

~~~

## Command Help

You can display all the options available by using `--help` on the `strip` sub-command.

```shell,script(name="run_help")
specdown strip --help
```

```text,verify(script_name="run_help", stream=stdout)
specdown-strip 
Outputs a version of the markdown with all specdown functions removed

USAGE:
    specdown strip <spec-file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <spec-file>    The spec file to strip specdown functions from
```

