---
layout: page
---
# Creating Test Files

Specifications can create files to run tests against.
These files exist temporarily while the tests run.

You create a file using the `file` function:

``` text
Example file content
```

The file path is then set in an environment variable which is available in future scripts.

``` shell
cat example.txt
```

Will output:

``` text
Example file content
```

