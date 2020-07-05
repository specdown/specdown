# Creating Test Files

Specifications can create files to run tests against.
These files exist temporarily while the tests run.

You create a file using the `file` function.
Let's look at this by creating a `writing_file_example.md` spec:

~~~markdown,file(path="writing_file_example.md")
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
~~~

We can now run this with:

```shell,script(name="writing_file_example")
specdown run writing_file_example.md
```

And we'll see

```text,verify(script_name="writing_file_example", stream=stdout)
- file example.txt created
- script 'cat-file' succeeded
- verify stdout from 'cat-file' succeeded
```