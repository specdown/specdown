class Specdown < Formula
  desc "Use your markdown documentation as tests"
  homepage "https://github.com/{{ github_repo }}"
  url "https://github.com/{{ github_repo }}/archive/{{ git_tag }}.tar.gz"
  sha256 "{{ file_sha }}"

  depends_on "rust" => :build

  resource("testdata") do
    url "https://raw.githubusercontent.com/{{ github_repo }}/{{ git_tag }}/README.md"
    sha256 "{{ readme_sha }}"
  end

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    system "#{bin}/specdown", "-h"
    system "#{bin}/specdown", "-V"

    resource("testdata").stage do
      assert_match "5 functions run (5 succeeded / 0 failed)", shell_output("#{bin}/specdown run README.md")
    end
  end
end
