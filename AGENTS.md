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
| `bioxus` | Facade apps import: `bioxus::design`, later `::widgets`, `::locale` |
| `bioxus-design` | Design system: tokens, constants, shared CSS (`act_design`) |
| `bioxus-widgets` | Reusable widgets (`act_widgets`), planned |
| `bioxus-locale` | i18n (`act_locale`), planned |

## Conventions

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
