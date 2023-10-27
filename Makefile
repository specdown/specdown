.DEFAULT_GOAL := build

SOURCE_FILES := $(shell find src -type f)

ifeq ($(OS),Windows_NT)
	TARGET_EXTENSION := .exe
else
	TARGET_EXTENSION :=
endif

TARGET := specdown$(TARGET_EXTENSION)

GH_PAGES_LOCATION?=gh-pages
DOC_FILES = $(shell find docs -type f)
GH_PAGES_FILES = $(patsubst %, $(GH_PAGES_LOCATION)/%, $(DOC_FILES))

.PHONY=build
build: check test dist/$(TARGET)

.PHONY=clean
clean:
	rm -rf target
	rm -rf dist
	rm -rf gh-pages

.PHONY=check
check:
	cargo fmt --all -- --check
	cargo clippy --all-targets --all-features -- -D warnings -D clippy::all -D clippy::pedantic -D clippy::cargo -A clippy::multiple-crate-versions
	cargo check

.PHONY=format
format:
	cargo fix --allow-dirty --allow-staged
	cargo fmt --all

.PHONY=test
test: target/release/$(TARGET)
	export PATH="$$(pwd)/$<:$$PATH"; cargo test -- --nocapture

dist:
	mkdir -p dist

target/debug/$(TARGET): Cargo.toml Cargo.lock $(SOURCE_FILES)
	cargo build

target/release/$(TARGET): Cargo.toml Cargo.lock $(SOURCE_FILES)
	cargo build --release

dist/$(TARGET): target/release/$(TARGET) dist
	cp -f "$<" "$@"

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
