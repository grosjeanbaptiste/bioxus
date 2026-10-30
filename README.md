# bioxus

Shared crates for Dioxus 0.7 apps: design tokens and stylesheet today, widgets and i18n next.
Layout and gates follow the `melos_apps` monorepo model: role-named packages under `packages/`,
`make ci-local` runs format, clippy (`-D warnings`) and tests.

## Use

```toml
bioxus = { git = "https://github.com/grosjeanbaptiste/bioxus", tag = "v0.0.0" }
```

```rust
rsx! { style { {bioxus::design::STYLESHEET} } }
```

Ready-made themes: set `data-theme="ocean|forest|sunset"` on the root element. The scheme follows the system; `data-scheme="light|dark"` on any element forces it for the page. `bioxus::design::form_factor::FormFactor` classifies a width as phone, tablet or desktop.

An app overrides a token by redefining it after the stylesheet. Tokens: `bg surface fg muted dim border accent on-accent ok warn danger`, `rem` units only.

Pick only what you need: `bioxus = { …, default-features = false, features = ["design"] }`. Inject `bioxus::widgets::STYLESHEET` after the design one.

## App icon

Every app has an icon: its own if the repo holds `assets/icon.svg|png` or `icon.svg|png`, else the BIOXUS icon. In the app's `build.rs`:

```rust
let out = std::path::PathBuf::from(std::env::var("OUT_DIR")?);
bioxus::design::icon::emit_png(std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR")?), &out)?;
```
For iOS, `bioxus::design::icon::ios::emit_set` writes the 13-file `AppIcon.appiconset` (`icon_<n>x<n>.png`, same names `ll` and `actrack` use). `dx bundle` writes no AppIcon entry, so without a set iOS shows a white square; the apps' scripts (`build_appicon.py`, `embed_ios_icons.py`) copy the set into the `.app` and patch `Info.plist`. An app with a master icon but no set gets an error telling it to run `scripts/make-ios-set.sh <master> <dir>`: its own icon is never replaced by the default.

then `dioxus::desktop::icon_from_memory(include_bytes!(concat!(env!("OUT_DIR"), "/app-icon.png")))` for the window, and the same file for bundles. `icon::emit` also keeps an SVG for the web favicon. A provided icon that is corrupt or unreadable fails the build; it never silently becomes the default.

## New app

`scripts/new-app.sh <name> <dir>` copies `template/`: one crate for web, desktop and mobile (features `web`, `desktop`, `mobile`), `AppShell`, icon `build.rs`, Makefile gates, `AGENTS.md`, `CONTEXT.md`, pinned to the current bioxus tag (`BIOXUS_TAG` overrides). Run it with `dx serve --platform web|desktop|ios|android`.

## Build

`make ci-local`

## Structure

| Package | Role |
|---|---|
| `bioxus` | The crate apps import; one module per block, each behind a feature |
| `bioxus-widgets` | `AppShell` (bottom bar on phones, rail on tablets, sidebar on desktops, forced with `layout`), `Badge`, `InputField`, `Table`, `Dialog`, `Stat` → `bioxus::widgets` |
| `bioxus-charts` | `BarChart` with pure geometry → `bioxus::charts` |
| `bioxus-locale` | Locales, catalogs, fallback chain, interpolation, no UI dependency → `bioxus::locale` |
| `bioxus-store` | Optional storage behind a `Store` port: memory, SQLite (feature `store-sqlite`), contract tests → `bioxus::store` |
| `bioxus-sync` | Offline-first sync: hybrid logical clock, last-write-wins, `Transport` port (feature `sync`) → `bioxus::sync` |
| `bioxus-design` | Tokens, reset, shared CSS (light and dark) → `bioxus::design` |

```text
bioxus/
├── Cargo.toml            workspace, members = packages/*, Dioxus pinned once
├── Makefile              format-check, lint, test, wasm-check, wasm-test, ci-local
├── scripts/              render-icon.sh, make-ios-set.sh, new-app.sh
├── template/             multiplatform app copied by new-app.sh (not a workspace member)
└── packages/
    ├── bioxus/           facade, features: design widgets charts locale locale-json store store-sqlite sync
    ├── bioxus-design/    src/{lib,form_factor,icon}.rs, theme.css, themes.css, assets/ (icons)
    ├── bioxus-widgets/   nav, shell, badge, field, table, dialog, stat, widgets.css
    ├── bioxus-charts/    scale, bar, charts.css
    ├── bioxus-locale/    locale, catalog, interpolate, translator
    ├── bioxus-viewer3d/  component, page.html, viewer3d.css (glTF viewer)
    ├── bioxus-store/     record, error, memory, sqlite, indexeddb, contract
    └── bioxus-sync/      clock, transport, hub, engine
```

Dependencies point one way: `widgets` → `design`, `charts` → `widgets`, `sync` → `store`; `locale` and `store` depend on no other package.

## Database

The DB is optional (`store`). One schema for every adapter: a single table of versioned documents, no per-app tables. The app owns the meaning of `collection` and `value`.

| Field | Type | Meaning |
|---|---|---|
| `collection` | text | Name of a group of documents (`"notes"`) |
| `id` | text | Document id, unique inside its collection |
| `value` | text, nullable | Opaque payload, JSON usually. `NULL` is a tombstone (deleted, kept so the deletion syncs) |
| `stamp` | u64 | Version chosen by the caller. A `put` with a lower stamp than the stored one is ignored; equal or higher wins |

Key: `(collection, id)`. Index: `stamp`, which serves `changes_since(stamp)`. `list` hides tombstones, `changes_since` returns them.

| Adapter | Feature | Storage |
|---|---|---|
| memory | `store` | Process memory, tests |
| SQLite | `store-sqlite` | Table `records(collection, id, value, stamp)`, `PRIMARY KEY (collection, id) WITHOUT ROWID`, index `records_stamp`. `stamp` is `INTEGER`; above `i64::MAX` is an error |
| IndexedDB | `indexeddb` on `bioxus-store` (wasm32) | Object store `records`, key path `[collection, id]`, index `stamp`. `stamp` is a 20-digit decimal string (JS numbers hold 53 bits) |

With `sync`, the stamp is a hybrid logical clock: 44 bits of milliseconds, 12 bits of counter, 8 bits of node id (256 nodes at most). Conflicts resolve last-write-wins; a clock more than 24 h ahead is refused.
