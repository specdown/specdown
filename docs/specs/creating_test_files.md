---
layout: page
---

# Creating Test Files

Specifications can create files to run tests against.
These files exist temporarily while the tests run.

You create a file using the `file` function.
Let's look at this by creating a `writing_file_example.md` spec:

```` markdown
```text,file(path="example.txt")
Example file content
```

The file path is then set in an environment variable which is available in future scripts.

```shell,script(name="cat-file")
cat example.txt
```

Will output:

```text,verify(script_name="cat-file")
Example file content
```
````

We can now run this with:

``` shell
specdown run writing_file_example.md
```

And we'll see

``` text
Running tests for writing_file_example.md:

  - creating file example.txt succeeded
  - running script 'cat-file' succeeded
  - verifying stdout from 'cat-file' succeeded

  3 functions run (3 succeeded / 0 failed)

```

