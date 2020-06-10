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
	cargo clippy --all-targets --all-features -- -D warnings -Dclippy::all -D clippy::pedantic -D clippy::cargo
	cargo check

.PHONY=format
format:
	cargo fmt --all

.PHONY=test
test: check
	mkdir -p .specdown
	export PATH="$$(pwd)/target/debug:$$PATH"; cargo test

dist:
	mkdir -p dist

dist/specdown: dist test
	cargo build --release
	cp target/release/specdown dist

