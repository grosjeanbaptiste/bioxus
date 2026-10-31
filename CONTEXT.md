# bioxus — public state as of 2026-11-01

The detailed working state names private projects and lives in `CONTEXT.private.md`
(ignored by git). This file holds only what is safe to publish.

## History rewrite (2026-11-01)

The history was rewritten to remove references to other projects from commit messages and
tags, and force-pushed (`main` and all 16 tags `v0.0.0`..`v0.9.0`). Commit hashes changed.
Anyone with a clone or a `Cargo.lock` pinning an old hash must re-clone or run
`cargo update -p bioxus`. Verified on a fresh clone of the remote: no private name in any
reachable commit, message or tag.

## Current state

- Version 0.0.6 (tag `v0.0.6`). Dioxus 0.7 pinned in the workspace. Licence: PolyForm
  Noncommercial 1.0.0 from v0.0.6 on (earlier tags: proprietary notice); see `LICENSE`
  and `CONTRIBUTING.md`.
- A library, not a framework: blocks are pieces the app calls; nothing imposes structure,
  state, route, language or look. User-facing text is always a prop (`Dialog.close_label`,
  `AppShell.more_label`).
- `bioxus` is the facade apps import. Features: `design`, `widgets`, `charts`, `locale`,
  `locale-json`, `viewer3d`, `viewer3d-ui` (default); opt-in `store`, `store-sqlite`, `sync`,
  `icons` (lucide-dioxus), `persist` (dioxus-sdk-storage), `time` (dioxus-sdk-time).
- `bioxus-design`: tokens, themes, form factors, default app icon and iOS icon set.
- `bioxus-widgets`: `AppShell` (bottom bar / rail / sidebar by CSS; `NavItem::secondary`
  moves items to a drawer on the phone bar), fields, table, dialogs, tabs, breadcrumb,
  empty state, avatar, status indicator and more. SSR tests; the drawer's open/close click
  is not tested.
- `bioxus-charts`, `bioxus-locale`, `bioxus-store` (memory, SQLite, IndexedDB),
  `bioxus-sync` (hybrid logical clock, last-write-wins; HTTP transport not built),
  `bioxus-viewer3d`.

## Decisions

- Every possibility stays open: each block is a feature and a separate package; behaviour
  sits behind ports.
- Reuse maintained Dioxus crates before writing a block; only what none covers is written here.
- The repository is public so people can read and contribute; commercial use needs written
  permission.
- No reference to private projects in tracked files or commit messages.

## Gates

`make ci-local` (format, clippy `-D warnings`, wasm check, tests).
