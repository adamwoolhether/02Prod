shell := $(shell echo ${SHELL})

# cargo install cargo-watch
# cargo install cargo-edit
# rustup toolchain install nightly --allow-downgrade
# cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres

# cargo watch -x check
# cargo watch -x check -x test -x run

# RUST_LOG=trace

sub:
	curl -i -X POST -d 'email=thomas_mann@hotmail.com&name=Tom' http://127.0.0.1:8000/subscriptions

####################################################################
# DB
####################################################################
db-migrate:
	SKIP_DOCKER=true scripts/init_db.sh
db-init:
	scripts/init_db.sh
db-down:
	docker stop zero2prod_dev
	docker rm zero2prod_dev

####################################################################
# DEV
####################################################################
# cargo install cargo-udeps

udeps:
	cargo +nightly udeps

test-verbose:
	TEST_LOG=true cargo test health_check_works

####################################################################
# BUILD
####################################################################
build:
	#cargo sqlx prepare -- --lib
	docker build --tag zero2prod --file Dockerfile .

docker-run:
	docker run -r -p 8000:8000 zero2prod

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
