# bioxus — state as of 2026-10-28

## Current state

- Workspace with `bioxus-design` (tokens + reset, rem only, light/dark), 4 tests; Dioxus pinned 0.7 in `Cargo.toml`. Source: `packages/bioxus-design`.
- `bioxus` (packages/bioxus) is the facade apps import (owner, 2026-10-28): `bioxus::design` behind feature `design`; new blocks are added as sub-packages re-exported there. Tag v0.2.0.
- Layout follows melos_apps: `packages/`, root Makefile gates (`make ci-local`).
- `bioxus-design` v0.3.0: 3 ready-made themes (ocean, forest, sunset; `data-theme` attribute, light+dark), form factors Phone/Tablet/Desktop by width (`form_factor`, breakpoints 40rem/64rem, also in the CSS), `--gutter`/`--columns` per breakpoint. Tests tie CSS to the Rust constants.

- `bioxus-widgets` v0.4.0: `AppShell` (CSS media-query layout, `layout` prop forces one, no JS/router), `Badge`/`Tone`, `NavItem`/`NavLayout`; SSR tests (dioxus-ssr) + token/px test. Facade features: design, widgets (default both).

- `bioxus-charts` v0.6.0: `BarChart`/`Bar` (tone from widgets::Tone), pure `scale::fill/max_of` (NaN, negative, zero max, clamp tested). Replaces ss graph bars, actrack Bars. The collapsible tree of ss ResourceGraph stays app-side (needs ss domain types).

- `bioxus-locale` v0.7.0: open `Locale` tags (chain fr-be -> fr -> fallback), `Catalog` (pairs/JSON flat+nested, key-by-key merge), `Translator` (missing key = visible key + handler, default stderr warn), `{name}` interpolation with `{{ }}` escapes. No UI dep; JSON behind feature `json`. ll-ui keeps its own typed `Strings` struct for now (migration only if the owner wants ll in scope).

- `bioxus-store` v0.8.0: async `Store` port (get/put/list/changes_since; opaque string value, caller `stamp`, tombstones; older stamp ignored, equal replaces), `MemoryStore`, `SqliteStore` (feature `sqlite`, rusqlite bundled, stamp > i64::MAX is an error), shared `contract::check` + `block_on` for any adapter. Facade features `store`, `store-sqlite` (opt-in, not default). Gates now run with --all-features. IndexedDB adapter: NOT done (step 4b, needs wasm32 + web-sys; wasm32-unknown-unknown target is installed).

- `bioxus-sync` v0.9.0: `Clock` (HLC stamp = 44-bit ms | 12-bit counter | 8-bit node, so max 256 nodes per account; drift guard 24 h; injected wall clock), `Transport` port with server arrival token (not stamp) as pull cursor, `MemoryTransport` hub, `Engine` (open seeds clock from store; put/delete/sync; pushes only own-node records, no echo; failed sync leaves markers untouched). 14 scenario tests: convergence, concurrent tie-break, late old-stamp push, tombstones, retry, idempotence, poisoned stamp, clock jump back. Facade feature `sync` (opt-in). HTTP transport NOT done: no server contract exists yet (step 5b).

- Version policy (owner 2026-10-28): the project is at 0.0.0. Workspace version set to 0.0.0 locally; tags v0.1.0..v0.9.0 already public and pinned by ss PR#2 and a peer actrack_deck change — keep until the owner decides. Local commit not pushed; push/tag was denied by the permission classifier, awaiting an explicit owner go.

- `bioxus-store` IndexedDB adapter (feature `indexeddb`, wasm32 only, `idb` crate): stamp stored as 20-digit string (JS numbers hold 53 bits), `stamp` index drives `changes_since`, put = one readwrite transaction. Compiles and passes clippy `-D warnings` on wasm32 (`make wasm-check`, part of ci-local). **Its browser tests (`tests/indexeddb.rs`: shared contract + >2^53 stamps) have NOT been run**: `make wasm-test` needs chromedriver and the sandbox could not resolve storage.googleapis.com (DNS). Run `make wasm-test` on a networked shell before trusting it.

- App icon rule (owner 2026-10-28): every app has an icon, the repo's own else BIOXUS's. Done in `bioxus-design::icon`: default SVG+PNG (512) embedded, `resolve`, `resolve_png`, `emit`, `emit_png` for build.rs; corrupt/unreadable provided icon = error. 7 tests. Inspired by ll and actrack (checked 2026-10-28): both keep an iOS set `icon_<n>x<n>.png` (13 sizes) in `stores/ios/AppIcon.appiconset` (ll) / `ui/mobile/assets/icons` (actrack) and each has its own Python script to embed it in the .app (`dx bundle` writes no AppIcon entry) — duplicated, candidates to absorb into bioxus later. `icon::ios` reads those exact locations + names; `resolve*` also treat a set's 1024 as the app icon. Run against the real repos: ll = provided master + 13-file set; actrack = provided set (its icons have an ALPHA channel, which the App Store rejects: not fixed, not mine to change); ss and Converge = BIOXUS default. NOT yet wired into any app: ss (PR #2) needs a published tag containing this; Converge and actrack_deck are being migrated by peer sessions.

- 2026-10-28 — other Claude sessions commit straight into this checkout (e.g. 6ceed9b "forced scheme, ok and on-accent tokens", made while I worked); my push of main published it with mine. Check `git log` before assuming a commit is yours.
- Icons beyond iOS (decided, not built): web favicon uses the master SVG/PNG (`icon::emit`/`emit_png`), Android launcher sizes derive from the master with the platform tooling; ll and actrack keep no web or Android icon convention to follow, so no extra sets were invented.

## Roadmap (owner request 2026-10-28: ready-made designs, optional DB, sync, every design element for web/desktop/tablet/phone; proposed, not started)

Apps import only `bioxus` (the melos_apps `act_root` rule); every block is a feature.
- `bioxus-widgets` — AppShell, Badge, InputField (text/number/date, native types), Table, Dialog (native <dialog>, Esc via oncancel), Stat done (v0.5.0). Lists skipped (Table covers it; add on demand). Charts are in bioxus-charts.
- `bioxus-store` — port, memory and sqlite done; `indexeddb` adapter written, browser test pending.
- `bioxus-sync` — engine, clock and in-process transport done; HTTP and p2p transports to do once a server contract exists.
Open choice: web storage backend (IndexedDB vs sqlite-wasm) before bioxus-store starts.
- Not ported from melos_apps: coverage floors, lint ceilings, test-impacted, pre-commit gate.

## Decisions

- 2026-10-28 — consumers are Converge, ss, actrack_deck only; apps stay in their repos and consume by git tag (owner).
- 2026-10-28 — renamed dioxus-kit to bioxus and made public with a clean history, because the first history tracked target/ (owner asked for public + the name).
- Token vocabulary: bg surface fg muted dim border accent warn danger; ss renamed --error->--danger, --pre-bg->--surface; actrack will rename --fg-muted->--muted, --fg-dim->--dim.
- Emvi apps are out of scope (formal prohibition).

## Consumers

- ss: PR #2 on grosjeanbaptiste/ss (stacked on #1), switching to `bioxus` (bioxus::design). Converge, actrack_deck: not started.
