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

# use cargo audit from rustsec to find vulnerabilities
audit:
    cargo audit

# use the mozilla cargo vet to protect from supply-chain attacks
vet:
    cargo vet

# run fmt, lint, audit and vet
ci: fmt lint audit vet

release: ci
    cargo build --release

clean:
    cargo clean

# bump version (patch, minor, major)
bump kind="patch":
    cargo set-version --bump {{ kind }}
