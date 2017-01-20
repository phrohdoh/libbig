.PHONY: build test
.DEFAULT_GOAL: build

build:
	@cargo rustc --release -- -D warnings

test:
	@cargo clippy --verbose -- -D warnings || echo "clippy not installed: skipping lints"
	@cargo test --release --verbose
	@cd cli-tools/sagebig && cargo test --release --verbose
	@cargo bench --verbose -- release
