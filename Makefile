.PHONY: run
run:
	cargo run ./sample.txt

.PHONY: fmt
fmt:
	cargo fmt
	cargo clippy