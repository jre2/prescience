build:
	@cargo build

run:
	@cargo run --release

test:
	@echo "Testing"
	@cargo test

bench:
	@echo "Benching"
	@cargo bench

.PHONY: build run test bench
