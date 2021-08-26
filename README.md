<p align="center">
  <img alt="specdown" src="./logo/logo.png">
</p>

<p align="center">A tool to test markdown files and drive development from documentation.</p>

## This document is an executable specification

When SpecDown is run with **this** document, it will execute the following shell command:

```shell,script(name="hello-specdown")
echo "Hello SpecDown"
```

It will then validate that the output from the above command matches the following codeblock:

```,verify(script_name="hello-specdown", stream=stdout)
Hello SpecDown
```

## Table Of Contents

- [Motivation](#motivation)
- [Installation](#installation)
- [Project Status](#project-status)
- [How does it work?](#how-does-it-work)
- [Full Documentation](#full-documentation)
- [Projects Using Specdown](#projects-using-specdown)

## Motivation

The motivation for this project has from two key places, these are:

1. Documentation on GitHub projects where the documented commands and output is out of date
2. Projects which use Cucumber but no one except the developers refer to the feature files

## Installation

The installation process of specdown is still in early stages.
Right now you two options are to download the compiled releases from GitHub, or clone the repository and build from source.
The recommended what is to download the compiled releases, you can do this from by using the following commands.

### Mac OS

You can download the binary and add it to your path

```shell,skip()
curl -L https://github.com/specdown/specdown/releases/latest/download/specdown-x86_64-apple-darwin -o /usr/local/bin/specdown
chmod +x /usr/local/bin/specdown
```

Alternatively you can also use [brew](https://brew.sh/)

```shell,skip()
brew install specdown/repo/specdown
```

### Linux

You can download the binary and add it to your path

```shell,skip()
curl -L https://github.com/specdown/specdown/releases/latest/download/specdown-x86_64-unknown-linux-gnu -o /usr/local/bin/specdown
chmod +x /usr/local/bin/specdown
```

You can also use [Homebrew on Linux](https://docs.brew.sh/Homebrew-on-Linux) to install the application

```shell,skip()
brew install specdown/repo/specdown
```

### Windows


You can download the binary and add it to your path

```powershell,skip()
Invoke-WebRequest -Uri "https://github.com/specdown/specdown/releases/latest/download/specdown-x86_64-pc-windows-msvc.exe" -OutFile "specdown.exe"
```

## How does it work?

The markdown for the example at the beginning of this document looks like this.
Let's save it to a file called `example.md`:

~~~markdown,file(path="example.md")
## This document is an executable specification

When SpecDown is run with this document, it will execute the following shell script.

```shell,script(name="hello-specdown")
echo "Hello SpecDown"
```

It will then validate that the output of the previous command matches the following codeblock.

```,verify(script_name="hello-specdown", stream=stdout)
Hello SpecDown
```
~~~

You can run this markdown specification by using the following command:

```shell,script(name="example")
specdown run example.md
```

This will produce the following output:

```text,verify(script_name="example", stream=stdout)
Running tests for example.md:

  - running script 'hello-specdown' succeeded
  - verifying stdout from 'hello-specdown' succeeded

  2 functions run (2 succeeded / 0 failed)

```

## Full Documentation

The documentation is written as executable specifications and can be read [here](./docs/index.md).

## Projects Using Specdown

_If your project is using specdown then feel free to add it to the list._

- [ed-system-search](https://github.com/PurpleBooth/ed-system-search)
  <br>
  A tool to find interesting systems in Elite: Dangerous.
- [ellipsis](https://github.com/PurpleBooth/ellipsis)
  <br>
  A dotfile manager.
- [git-mit](https://github.com/PurpleBooth/git-mit)
  <br>
  A suite of git hooks. It's aimed to make pair programming, adding issue numbers to your commits, and following good commit message practices something that happens without thinking about it.
- [specdown](https://github.com/specdown/specdown)
  <br>
  Specdown tests itself.
- [whatismyip](https://github.com/PurpleBooth/whatismyip)
  <br>
  Work out what your external ip is.
