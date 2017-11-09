run: format clippy test build graph clean
	cargo run

build:
	cargo build

format:
	cargo fmt

test:
	cargo test

check:
	cargo check

#TODO: Upgrade clippy version
clippy:
	cargo +nightly-2017-07-20 clippy

doc:
	cargo doc

# Display module structure
modules:
	cargo modules --orphans

# Generate dependency graph
graph:
	cargo graph | dot -Tpng > deps.png

# Run fuzz
fuzz: fuzz1 fuzz2
fuzz1:
	cargo fuzz run fuzz_target_1 -- -max_len=4128 -max_total_time=600
fuzz2:
	cargo fuzz run fuzz_target_write -- -max_len=4128 -max_total_time=600

clean:
	rm **/*.bk
