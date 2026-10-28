# Same target names as melos_apps (format-check, lint, test, ci-local).
# Every build/test goes through the heavy queue; without `dotfiles` it fails, naming it.
HEAVY := $(shell command -v dotfiles >/dev/null 2>&1 && echo "dotfiles heavy run --quick --" || echo "")

.PHONY: need-dotfiles format-check format lint test wasm-check wasm-test ci-local

need-dotfiles:
	@command -v dotfiles >/dev/null 2>&1 || { echo "error: 'dotfiles' not found; builds must run through 'dotfiles heavy run'" >&2; exit 1; }

format-check: ## Check formatting without rewriting anything
	@cargo fmt --all --check

format: ## Format every package
	@cargo fmt --all

lint: need-dotfiles ## clippy, warnings are errors
	@$(HEAVY) cargo clippy --workspace --all-targets --all-features -- -D warnings

test: need-dotfiles ## Test every package
	@$(HEAVY) cargo test --workspace --all-features

wasm-check: need-dotfiles ## Lint the browser adapter and compile its tests for wasm32
	@$(HEAVY) cargo clippy -p bioxus-store --features indexeddb --target wasm32-unknown-unknown --all-targets -- -D warnings

wasm-test: need-dotfiles ## Run the IndexedDB contract in headless Chrome (needs network for chromedriver)
	@wasm-pack test --headless --chrome packages/bioxus-store --features indexeddb

ci-local: format-check lint wasm-check test ## The CI gates, on this machine
