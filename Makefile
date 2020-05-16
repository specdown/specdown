.DEFAULT_GOAL := build

.PHONY=build
build: dist/specdown

.PHONY=clean
clean:
	rm -rf target
	rm -rf dist

.PHONY=check
check:
	cargo fmt --all -- --check
	cargo clippy --all-features -- -D warnings -Dclippy::all -D clippy::pedantic
	cargo check

.PHONY=format
format:
	cargo fmt --all

.PHONY=test
test: check
	cargo test
	PATH="target/debug:${PATH}" specdown run README.md

dist:
	mkdir -p dist

dist/specdown: dist test
	cargo build --release
	cp target/release/specdown dist

