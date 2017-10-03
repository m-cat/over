run: format clippy test build graph clean
	cargo run

build:
	cargo build

format:
	cargo fmt # run rustfmt

test:
	cargo test

# Run compiler checks without building
check:
	cargo check

clippy:
	cargo +nightly-2017-07-20 clippy

doc:
	cargo doc

# Display module structure
modules:
	cargo modules

# Generate dependency graph
graph:
	cargo graph | dot -Tpng > deps.png

clean:
	rm **/*.bk
