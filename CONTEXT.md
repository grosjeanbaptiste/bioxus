# bioxus — state as of 2026-10-30

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

- Version policy (owner 2026-10-28): the project is at 0.0.0. Workspace version set to 0.0.0 locally; tags v0.1.0..v0.9.0 already public and pinned by ss PR#2 and a peer actrack_deck change — keep until the owner decides. The workspace version commit is now public: see the 2026-10-30 v0.0.1 entry.

- `bioxus-store` IndexedDB adapter (feature `indexeddb`, wasm32 only, `idb` crate): stamp stored as 20-digit string (JS numbers hold 53 bits), `stamp` index drives `changes_since`, put = one readwrite transaction. Compiles and passes clippy `-D warnings` on wasm32 (`make wasm-check`, part of ci-local). Its browser tests (`tests/indexeddb.rs`: shared contract + >2^53 stamps) pass in headless Chrome: `make wasm-test`, 2 passed, 2026-10-30. `make wasm-test` needs network to fetch chromedriver.

- App icon rule (owner 2026-10-28): every app has an icon, the repo's own else BIOXUS's. Done in `bioxus-design::icon`: default SVG+PNG (512) embedded, `resolve`, `resolve_png`, `emit`, `emit_png` for build.rs; corrupt/unreadable provided icon = error. 7 tests. Inspired by ll and actrack (checked 2026-10-28): both keep an iOS set `icon_<n>x<n>.png` (13 sizes) in `stores/ios/AppIcon.appiconset` (ll) / `ui/mobile/assets/icons` (actrack) and each has its own Python script to embed it in the .app (`dx bundle` writes no AppIcon entry) — duplicated, candidates to absorb into bioxus later. `icon::ios` reads those exact locations + names; `resolve*` also treat a set's 1024 as the app icon. Run against the real repos: ll = provided master + 13-file set; actrack = provided set (its icons have an ALPHA channel, which the App Store rejects: not fixed, not mine to change); ss and Converge = BIOXUS default. Wired into ss (`kit-design`, tag v0.0.0) and ll (`ll-desktop`); Converge and actrack_deck were being migrated by peer sessions (not rechecked).

- 2026-10-28 — other Claude sessions commit straight into this checkout (e.g. 6ceed9b "forced scheme, ok and on-accent tokens", made while I worked); my push of main published it with mine. Check `git log` before assuming a commit is yours.
- Icons beyond iOS (decided, not built): web favicon uses the master SVG/PNG (`icon::emit`/`emit_png`), Android launcher sizes derive from the master with the platform tooling; ll and actrack keep no web or Android icon convention to follow, so no extra sets were invented.

- 2026-10-30 — `template/` + `scripts/new-app.sh <name> <dir>`: multiplatform app (web/desktop/mobile features, AppShell, icon build.rs, gates, AGENTS/CONTEXT). Verified: generated app passes fmt, clippy `-D warnings` and its SSR test offline with `--no-default-features --features store,sync` against the local checkout. Verified 2026-10-30 on an app generated against tag v0.0.1: `cargo check` passes for `web` (wasm32-unknown-unknown), `desktop` and `mobile` (aarch64-apple-ios), and `dx build --platform web` succeeds. NOT verified: running the app (`dx serve`, simulator, desktop window).

- 2026-10-30 — pushed main and tag v0.0.1 (workspace 0.0.1): first tag containing the template, widgets TopBar/Panel/Section/Spinner/BusyButton/Notice/Chip, viewer3d and the README structure. v0.0.0 is untouched (older commit); `new-app.sh` pins v0.0.1.

- 2026-10-30 — pushed tag v0.0.2 (workspace 0.0.2): SelectField, TextAreaField, Collapsible, LineChart; `new-app.sh` pins it.

- 2026-10-30 — pushed tag v0.0.3 (workspace 0.0.3): the widget gaps listed under Consumers.

## Roadmap (owner request 2026-10-28: ready-made designs, optional DB, sync, every design element for web/desktop/tablet/phone; proposed, not started)

Apps import only `bioxus` (the melos_apps `act_root` rule); every block is a feature.
- `bioxus-widgets` — AppShell, Badge, InputField (text/number/date, native types), Table, Dialog (native <dialog>, Esc via oncancel), Stat done (v0.5.0). SelectField, TextAreaField, Collapsible added 2026-10-30 (v0.0.2). Lists skipped (Table covers it; add on demand). Charts are in bioxus-charts (BarChart, LineChart).
- `bioxus-store` — port, memory, sqlite and `indexeddb` adapters done and tested.
- `bioxus-sync` — engine, clock and in-process transport done; HTTP and p2p transports to do once a server contract exists.
Open choice: web storage backend (IndexedDB vs sqlite-wasm) before bioxus-store starts.
- Not ported from melos_apps: coverage floors, lint ceilings, test-impacted, pre-commit gate.

- `bioxus-viewer3d` (owner request 2026-10-28, from Converge's standalone Three.js page): `page(title, &[Part])` builds a standalone HTML page (glb embedded as base64, Three.js 0.170.0 from jsDelivr, title and part names escaped), feature `component` adds `Viewer3d` (the page in an `<iframe srcdoc>`, so no JS glue on any renderer). Facade features `viewer3d` (page only, no Dioxus) and `viewer3d-ui` (default). 8 tests pass (2026-10-30, `make ci-local`). The first run failed on one test that expected named entities, while dioxus-ssr writes numeric ones (`&#34;`); the test now checks that the title is escaped, not how. Needs network at view time (Three.js from the CDN). Converge consumes it by git `rev`, no tag cut (version policy above is the owner's).

## Decisions

- 2026-10-28 — ll added as a consumer (owner). Earlier: consumers are Converge, ss, actrack_deck only; apps stay in their repos and consume by git tag (owner).
- 2026-10-28 — renamed dioxus-kit to bioxus and made public with a clean history, because the first history tracked target/ (owner asked for public + the name).
- Token vocabulary: bg surface fg muted dim border accent warn danger; ss renamed --error->--danger, --pre-bg->--surface; actrack will rename --fg-muted->--muted, --fg-dim->--dim.
- Emvi apps are out of scope (formal prohibition).

## Consumers

- Goal (owner 2026-10-30): ll, ss, actrack, Actif, Converge, acteble_app must all be built with bioxus; screens and domain code stay in each app. Audit 2026-10-30 (component names and grep, not a line-by-line read): all six are already Dioxus. All on bioxus since 2026-10-30, merged PRs: acteble_app#1, converge#1, actrack_deck#43, ll_app#3, Actif#1 on v0.0.2; ss#3 on v0.0.1 (branch `bioxus-ui`, not master). Not compiled by an agent: Converge (none) and acteble_app (`cargo test` not run, CONVENTIONS §16.2). No app checked on screen. Widget gaps closed in v0.0.3: DataTable + TableRow (rich cells, selectable rows), `bx-num`; InputField placeholder, disabled, autofocus, max_length, class, label_hidden; CheckField; NavItem with_section and disabled; Tone::Ok renders in --ok. Not absorbed yet: the iOS icon embedding scripts of ll and actrack.

- ll (owner 2026-10-28: LL's UI is built from bioxus bricks; its screens stay in the private ll_app): `ll-ui` pins `bioxus` by rev with features `design`, `widgets`, `locale-json` (tokens + `data-scheme`, TopBar/Panel/Section/BusyButton/Spinner/Notice/Chip and the bx-btn/bx-option classes, JSON catalogs); `ll-desktop` build.rs uses `icon::emit_png` (LL's own icon).

- ss: PR #2 (`kit-design`, stacked on #1) has design + icon on tag v0.0.0. The active line is `bioxus-ui` (no PR to master; design, widgets, charts). PR #3 (merged into `bioxus-ui` 2026-10-30) moved it to v0.0.1 and added the icon: fmt, clippy and 1361 tests pass (2026-10-30); window not checked on screen. Converge, actrack_deck: not started.
