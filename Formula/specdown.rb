class Specdown < Formula
  desc "Tool to test markdown files and drive development from documentation"
  homepage "https://github.com/specdown/specdown"
  url "https://github.com/specdown/specdown/archive/refs/tags/v0.24.0.tar.gz"
  sha256 "ccc711316b42bbf2208c5d74dc505c544b62c9ce020dc13b68f4b5416c7a4579"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    test_file = testpath/"test.md"
    File.write(test_file, <<-EOM
    # SpecDown

    A tool to test markdown files and drive development from documentation.

    ## This document is an executable specification

    When SpecDown is run with this document, it will execute the following shell script.

    ```shell,script(name="hello-specdown")
    echo "Hello SpecDown"
    ```

    It will then validate that the output of the previous command matches the following codeblock.

    ```,verify(script_name="hello-specdown", stream=stdout)
    Hello SpecDown
    ```

    ## Project Status

    This project is currently **pre-alpha**.
    It is currently an experiment to see if the idea is worth following through.

    ## Documentation

    The documentation is written as executable specifications and can be read [here](./doc/index.md).
    EOM
    )
    system "#{bin}/specdown", "-h"
    system "#{bin}/specdown", "-V"
    system "#{bin}/specdown", "run", test_file
  end
end
