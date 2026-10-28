# Same target names as melos_apps (format-check, lint, test, ci-local).
# Every build/test goes through the heavy queue; without `dotfiles` it fails, naming it.
HEAVY := $(shell command -v dotfiles >/dev/null 2>&1 && echo "dotfiles heavy run --quick --" || echo "")

.PHONY: need-dotfiles format-check format lint test ci-local

need-dotfiles:
	@command -v dotfiles >/dev/null 2>&1 || { echo "error: 'dotfiles' not found; builds must run through 'dotfiles heavy run'" >&2; exit 1; }

format-check: ## Check formatting without rewriting anything
	@cargo fmt --all --check

format: ## Format every package
	@cargo fmt --all

lint: need-dotfiles ## clippy, warnings are errors
	@$(HEAVY) cargo clippy --workspace --all-targets -- -D warnings

test: need-dotfiles ## Test every package
	@$(HEAVY) cargo test --workspace

ci-local: format-check lint test ## The CI gates, on this machine
