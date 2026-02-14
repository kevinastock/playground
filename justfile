set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

default: check

check:
    cargo check --workspace

build:
    cargo build --workspace

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
    cargo test --workspace --all-targets

ci: fmt-check clippy test

run-server *args:
    cargo run -p server -- {{args}}

run-cli *args:
    cargo run -p cli -- {{args}}

clean:
    cargo clean
