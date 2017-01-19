.PHONY: test
.DEFAULT_GOAL: test

test:
	@cargo test --verbose
	@cd cli-tools/sagebig && cargo test -- verbose
