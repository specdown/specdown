.DEFAULT_GOAL := build

GH_PAGES_LOCATION?=gh-pages
DOC_FILES = $(shell find docs -type f)
GH_PAGES_FILES = $(patsubst %, $(GH_PAGES_LOCATION)/%, $(DOC_FILES))

.PHONY=build
build: dist/specdown

.PHONY=clean
clean:
	rm -rf target
	rm -rf dist
	rm -rf gh-pages

.PHONY=check
check:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings -Dclippy::all -D clippy::pedantic -D clippy::cargo
	cargo check

.PHONY=format
format:
	cargo fmt --all

.PHONY=test
test: check
	export PATH="$$(pwd)/target/debug:$$PATH"; cargo test -- --nocapture

dist:
	mkdir -p dist

dist/specdown: dist test
	cargo build --release
	cp target/release/specdown dist

$(GH_PAGES_LOCATION): $(GH_PAGES_LOCATION)/index.md $(GH_PAGES_LOCATION)/logo/logo.png $(GH_PAGES_FILES)

$(GH_PAGES_LOCATION)/index.md:
	mkdir -p "$(GH_PAGES_LOCATION)"
	echo "---\nlayout: page\n---\n" >"$(GH_PAGES_LOCATION)/index.md"
	specdown strip README.md >>"$(GH_PAGES_LOCATION)/index.md"

$(GH_PAGES_LOCATION)/logo/logo.png:
	mkdir -p "$(GH_PAGES_LOCATION)/logo"
	cp "$(subst $(GH_PAGES_LOCATION)/,,$@)" "$@"

$(GH_PAGES_LOCATION)/docs/%.md:
	mkdir -p "$(@D)"
	echo "---\nlayout: page\n---\n" >"$@"
	specdown strip "$(subst $(GH_PAGES_LOCATION)/,,$@)" >>"$@"
