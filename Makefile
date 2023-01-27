shell := $(shell echo ${SHELL})

# cargo install cargo-watch
# cargo install cargo-edit
# rustup toolchain install nightly --allow-downgrade
# cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres

# curl -X POST \
#    --data 'name=le%20guin&email=ursula_le_guin%40gmail.com' \
#    https://zero2prod-xqqlo.ondigitalocean.app/subscriptions \
#    --verbose

# cargo watch -x check
# cargo watch -x check -x test -x run

# RUST_LOG=trace
# ulimit -n 1024

# export RUST_LOG="sqlx=error,info"
# export TEST_LOG=enabled
# cargo t subscribe_fails_if_there_is_a_fatal_database_error | bunyan

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
pgcli:
	pgcli postgres://postgres:password@localhost/newsletter

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
	cargo check
	docker build --tag zero2prod --file Dockerfile .

docker-run:
	docker run --rm -p 8000:8000 zero2prod | jq

# doctl apps list
# doctl app update {APP-ID} --spec=spec.yaml
# DATABASE_URL=DIGITAL-OCEAN-DB-CONN-STRING sqlx migrate run
deploy:
	doctl apps create --spec spec.yaml
deploy-update:
	doctl app update $$(doctl apps list | grep zero2prod | cut -f 1 -d ' ') --spec=spec.yaml

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
	cargo audit --ignore RUSTSEC-2020-0071 --ignore RUSTSEC-2023-0001

watch:
	cargo watch -x check -x test -x run
format:
	cargo fmt -- --check
