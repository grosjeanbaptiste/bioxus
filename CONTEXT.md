# bioxus — state as of 2026-10-28

## Current state

- Workspace with `bioxus-design` (tokens + reset, rem only, light/dark), 4 tests; Dioxus pinned 0.7 in `Cargo.toml`. Source: `packages/bioxus-design`.
- `bioxus` (packages/bioxus) is the facade apps import (owner, 2026-10-28): `bioxus::design` behind feature `design`; new blocks are added as sub-packages re-exported there. Tag v0.2.0.
- Layout follows melos_apps: `packages/`, root Makefile gates (`make ci-local`).
- `bioxus-design` v0.3.0: 3 ready-made themes (ocean, forest, sunset; `data-theme` attribute, light+dark), form factors Phone/Tablet/Desktop by width (`form_factor`, breakpoints 40rem/64rem, also in the CSS), `--gutter`/`--columns` per breakpoint. Tests tie CSS to the Rust constants.

- `bioxus-widgets` v0.4.0: `AppShell` (CSS media-query layout, `layout` prop forces one, no JS/router), `Badge`/`Tone`, `NavItem`/`NavLayout`; SSR tests (dioxus-ssr) + token/px test. Facade features: design, widgets (default both).

- `bioxus-charts` v0.6.0: `BarChart`/`Bar` (tone from widgets::Tone), pure `scale::fill/max_of` (NaN, negative, zero max, clamp tested). Replaces ss graph bars, actrack Bars. The collapsible tree of ss ResourceGraph stays app-side (needs ss domain types).

## Roadmap (owner request 2026-10-28: ready-made designs, optional DB, sync, every design element for web/desktop/tablet/phone; proposed, not started)

Apps import only `bioxus` (the melos_apps `act_root` rule); every block is a feature.
- `bioxus-widgets` — AppShell, Badge, InputField (text/number/date, native types), Table, Dialog (native <dialog>, Esc via oncancel), Stat done (v0.5.0). Lists skipped (Table covers it; add on demand). Charts are in bioxus-charts.
- `bioxus-store` (feature `store`, optional) — hexagonal `Store` port; adapters behind features: `sqlite` (rusqlite, desktop/mobile), `indexeddb` (web/wasm).
- `bioxus-sync` (feature `sync`, needs `store`) — outbox/inbox pipeline like act_acteble, last-write-wins on a hybrid logical clock, `Transport` port (HTTP first, p2p later like act_p2p).
- `bioxus-locale` — i18n.
Open choice: web storage backend (IndexedDB vs sqlite-wasm) before bioxus-store starts.
- Not ported from melos_apps: coverage floors, lint ceilings, test-impacted, pre-commit gate.

## Decisions

- 2026-10-28 — consumers are Converge, ss, actrack_deck only; apps stay in their repos and consume by git tag (owner).
- 2026-10-28 — renamed dioxus-kit to bioxus and made public with a clean history, because the first history tracked target/ (owner asked for public + the name).
- Token vocabulary: bg surface fg muted dim border accent warn danger; ss renamed --error->--danger, --pre-bg->--surface; actrack will rename --fg-muted->--muted, --fg-dim->--dim.
- Emvi apps are out of scope (formal prohibition).

## Consumers

- ss: PR #2 on grosjeanbaptiste/ss (stacked on #1), switching to `bioxus` (bioxus::design). Converge, actrack_deck: not started.
