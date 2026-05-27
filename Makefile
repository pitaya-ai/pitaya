.PHONY: verify verify-rust verify-frontend dev fmt lint-comments bindings

verify: verify-rust verify-frontend lint-comments

verify-rust:
	cargo fmt --all -- --check
	cargo clippy --workspace --all-targets -- -D warnings
	cargo test --workspace
	cargo build -p pitaya-desktop

verify-frontend:
	pnpm check
	pnpm lint
	pnpm build

dev:
	pnpm dev

fmt:
	cargo fmt --all

bindings:
	cargo test export_bindings_match -p pitaya-desktop -- --nocapture

lint-comments:
	@! rg -n 'P[1-5]|ROADMAP' crates/ apps/pitaya-desktop/src apps/pitaya-desktop/src-tauri/src 2>/dev/null || (echo "Phase or ROADMAP references found in source comments or UI copy" && exit 1)
