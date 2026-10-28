# bioxus

Project-level instructions for Claude Code. Loaded on top of the user-wide
`~/.claude/CLAUDE.md` in any session opened inside this repo.

Keep this file focused on what is *specific* to this project. Cross-machine
rules belong in the user-wide file; conventions that only apply here belong
here.

## Start of every session

Read `CONTEXT.md` first: it holds the verified current state, deadlines and
pending items, so the repository does not have to be re-read to rebuild
them. Update it (and its date) after any act that changes that state.

## What this is

Shared Dioxus 0.7 crates (theme, i18n, widgets, charts, shell) consumed by the
user's Dioxus apps (ll, Actif, actrack, Converge, acteble_app, ss) by git tag.

## Model

Rust twin of `~/Source/Acteble/melos_apps`: one workspace, role-named packages
under `packages/`, path dependencies between them, root-level gates run by
`make` (same target names). Apps stay in their own repos and consume packages
by git tag.

| Package | Role (melos_apps twin) |
|---|---|
| `bioxus` | Facade apps import: `bioxus::{design,widgets,charts,locale,store,sync}` |
| `bioxus-charts` | Bar charts, pure geometry → `bioxus::charts` |
| `bioxus-design` | Design system: tokens, constants, shared CSS (`act_design`) |
| `bioxus-widgets` | Responsive `AppShell` (bottom bar / rail / sidebar by CSS), `Badge` (`act_widgets`) → `bioxus::widgets` |
| `bioxus-store` | Optional DB behind a `Store` port (`act_storage`) → `bioxus::store` |
| `bioxus-sync` | HLC + last-write-wins sync over a `Transport` port (`act_acteble`, `act_p2p`) → `bioxus::sync` |
| `bioxus-locale` | i18n, open locale tags (`act_locale`) → `bioxus::locale` |

## Conventions

- Keep every possibility open (owner, 2026-10-28): each block is a feature and a separate package; no router, renderer, DB or backend is imposed; behaviour sits behind ports; layout is CSS-driven with an explicit override prop.
- Every app always has an app icon (owner, 2026-10-28): the repo's own, else the BIOXUS icon (`bioxus::design::icon`). Regenerate the default PNG with `scripts/render-icon.sh`.
- One Dioxus version, pinned in the workspace.
- Crates stay small; no px units; each public widget has a test.

## Commands worth knowing

| Want | Run |
|---|---|
| All gates | `make ci-local` |
| Test | `dotfiles heavy run --quick -- cargo test --workspace` |
| Lint | `dotfiles heavy run --quick -- cargo clippy --workspace --all-targets -- -D warnings` |

## When in doubt

Open the README, or grep the repo. The source is the documentation.
