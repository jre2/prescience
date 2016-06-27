RFLAGS=RUSTFLAGS="-C target-cpu=native"

build:
	@$(RFLAGS) cargo build

run:
	@$(RFLAGS) cargo run --release

test:
	@echo "Testing"
	@$(RFLAGS) cargo test

bench:
	@echo "Benching"
	@$(RFLAGS) cargo bench

clean:
	@cargo clean

.PHONY: build run test bench clean
