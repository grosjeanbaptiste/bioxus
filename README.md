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

Ready-made themes: set `data-theme="ocean|forest|sunset"` on the root element. `bioxus::design::form_factor::FormFactor` classifies a width as phone, tablet or desktop.

An app overrides a token by redefining it after the stylesheet. Tokens: `bg surface fg muted dim border accent warn danger`, `rem` units only.

Pick only what you need: `bioxus = { …, default-features = false, features = ["design"] }`. Inject `bioxus::widgets::STYLESHEET` after the design one.

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
