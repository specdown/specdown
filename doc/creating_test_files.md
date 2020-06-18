# Creating Test Files

Specifications can create files to run tests against.
These files exist temporarily while the tests run.

You create a file using the `file` function:

```text,file(path="example.txt")
Example file content
```

The file path is then set in an environment variable which is available in future scripts.

```shell,script(name="cat-file")
cat example.txt
```

Will output:

```text,verify(script_name="cat-file", stream=stdout)
Example file content
```
