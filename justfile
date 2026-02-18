# https://just.systems

default:
    just --list

build:
    cargo build

run id='':
    cargo run -- {{ id }}

fmt:
    cargo fmt --all

lint:
    cargo clippy --all-targets --all-features -- -D warnings

audit:
    cargo audit

vet:
    cargo vet

ci: fmt lint audit vet

release: ci
    cargo build --release

clean:
    cargo clean

bump kind="patch":
    cargo set-version --bump {{ kind }}
