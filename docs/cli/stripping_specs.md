---
layout: page
---

# Stripping Specs

When writing specdown documents, you have to add the addition specdown functions to your markdown.
The syntax used by specdown upsets some markdown parses, so the `strip` command can be used to generate a version of the documents with the specdown specific content removed.

## Example

Give a markdown spec called `strip_example.md`:

```` markdown
# Strip Example

```shell,script(name="hello_world")
echo "Hello world"
```
````

You can run:

``` shell
specdown strip strip_example.md
```

And you'll get get following output:

```` markdown
# Strip Example

``` shell
echo "Hello world"
```

````

## Command Help

You can display all the options available by using `--help` on the `strip` sub-command.

``` shell
specdown strip --help
```

### Non-Windows Output

``` text
Outputs a version of the markdown with all specdown functions removed

Usage: specdown strip <SPEC_FILE>

Arguments:
  <SPEC_FILE>  The spec file to strip specdown functions from

Options:
  -h, --help  Print help information
```

### Windows Output

``` text
specdown-strip 
Outputs a version of the markdown with all specdown functions removed

USAGE:
    specdown strip <SPEC_FILE>

ARGS:
    <SPEC_FILE>    The spec file to strip specdown functions from

OPTIONS:
    -h, --help    Print help information
```

