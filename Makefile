shell := $(shell echo ${SHELL})

# cargo install cargo-watch
# cargo install cargo-edit
# rustup toolchain install nightly --allow-downgrade


# cargo watch -x check
# cargo watch -x check -x test -x run

####################################################################
# CI
####################################################################
# cargo install cargo-tarpaulin
# rustup component add clippy
# rustup component add rustfmt
# cargo install cargo-audit
ci:
	cargo test
	cargo tarpaulin --ignore-tests
	cargo clippy -- -D warnings
	cargo fmt -- --check
	cargo audi