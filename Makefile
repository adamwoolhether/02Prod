shell := $(shell echo ${SHELL})

####################################################################
# CI
####################################################################
ci:
	cargo test
	cargo tarpaulin --ignore-tests
	cargo clippy -- -D warnings
	cargo fmt -- --check
	cargo audi