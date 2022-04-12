shell := $(shell echo ${SHELL})

# cargo install cargo-watch
# cargo install cargo-edit
# rustup toolchain install nightly --allow-downgrade
# cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres


# cargo watch -x check
# cargo watch -x check -x test -x run

# RUST_LOG=trace

####################################################################
# TEST
####################################################################
sub:
	curl -i -X POST -d 'email=thomas_mann@hotmail.com&name=Tom' http://127.0.0.1:8000/subscriptions

db-migrate:
	SKIP_DOCKER=true scripts/init_db.sh
db-init:
	scripts/init_db.sh


####################################################################
# CI
####################################################################
# cargo install cargo-tarpaulin
# rustup component add clippy
# rustup component add rustfmt
# cargo install cargo-audit
ci:
	cargo test
	@#cargo tarpaulin --ignore-tests
	cargo clippy -- -D warnings
	cargo fmt -- --check
	cargo audit

watch:
	cargo watch -x check -x test -x run
format:
	cargo fmt -- --check
