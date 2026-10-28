# bioxus

Shared crates for Dioxus 0.7 apps: design tokens and stylesheet today, widgets and i18n next.
Layout and gates follow the `melos_apps` monorepo model: role-named packages under `packages/`,
`make ci-local` runs format, clippy (`-D warnings`) and tests.

## Use

```toml
bioxus-design = { git = "https://github.com/grosjeanbaptiste/bioxus", tag = "v0.1.0" }
```

```rust
rsx! { style { {bioxus_design::STYLESHEET} } }
```

An app overrides a token by redefining it after the stylesheet. Tokens: `bg surface fg muted dim border accent warn danger`, `rem` units only.

## Build

`make ci-local`

## Structure

| Package | Role |
|---|---|
| `bioxus-design` | Tokens, reset, shared CSS (light and dark) |
