# Pitaya developer commands

default:
    @just check

dev:
    pnpm dev

check:
    just pre-commit

pre-commit:
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets -- -D warnings
    cargo check --workspace
    cargo test --workspace
    pnpm check
    cargo build -p pitaya-desktop

test-rust:
    cargo test --workspace

test-ui:
    pnpm check
