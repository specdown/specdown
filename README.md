# SpecDown

A tool to test markdown files and drive development from documentation.

## This document is an executable specification

When SpecDown is run with this document, it will execute the following shell script.

```shell,script(name="hello-specdown")
echo "Hello SpecDown"
```

It will then validate that the output of the previous command matches the following codeblock.

```,verify(script_name="hello-specdown", stream=output)
Hello SpecDown
```

## Project Status

This project is currently **pre-alpha**.
It is currently an experiment to see if the idea is worth following through.