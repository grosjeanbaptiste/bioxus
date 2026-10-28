# bioxus — state as of 2026-10-28

## Current state

- Workspace with `bioxus-design` (tokens + reset, rem only, light/dark), 4 tests; Dioxus pinned 0.7 in `Cargo.toml`. Source: `packages/bioxus-design`.
- Layout follows melos_apps: `packages/`, root Makefile gates (`make ci-local`).
- Planned: bioxus-locale, bioxus-widgets, bioxus-charts, bioxus-shell (not started).
- Not ported from melos_apps: coverage floors, lint ceilings, test-impacted, pre-commit gate.

## Decisions

- 2026-10-28 — consumers are Converge, ss, actrack_deck only; apps stay in their repos and consume by git tag (owner).
- 2026-10-28 — renamed dioxus-kit to bioxus and made public with a clean history, because the first history tracked target/ (owner asked for public + the name).
- Token vocabulary: bg surface fg muted dim border accent warn danger; ss renamed --error->--danger, --pre-bg->--surface; actrack will rename --fg-muted->--muted, --fg-dim->--dim.
- Emvi apps are out of scope (formal prohibition).

## Consumers

- ss: PR #2 on grosjeanbaptiste/ss (stacked on #1), switching to bioxus-design. Converge, actrack_deck: not started.
