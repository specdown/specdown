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

dist/specdown: test
	cargo build --release dist

.PHONY=build
build: dist/specdown