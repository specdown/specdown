---
layout: page
---
# Skipping Code Blocks

Not all codeblocks in your markdown need to be tested by specdown.
If you you want some that are just informative, use the function `skip()`.

Given the following markdown file `skip_example.md`:

```` markdown
# Skipping Code Blocks Example

```test,skip()
This codeblock is not executed
```
````

When running:

``` shell
specdown run skip_example.md
```

Then you should see the following:

``` text
Running tests for skip_example.md:


  0 functions run (0 succeeded / 0 failed)
```

