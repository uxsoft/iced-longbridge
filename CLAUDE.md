# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Common commands

```sh
cargo run -p demo-app                                  # native desktop showcase
trunk serve crates/demo-web                            # WASM build, served at localhost:8080
cargo build -p demo-web --target wasm32-unknown-unknown  # what CI runs (skip Trunk)

cargo test -p iced-longbridge --lib                    # library tests (CI's only test step)
cargo test -p iced-longbridge --lib <name>             # single test by substring
cargo clippy --workspace --all-targets                 # informational in CI; not blocking
cargo deny check                                       # licence + advisory + duplicate crate scan
cargo audit                                            # CVE scan
```

`mise.toml` exposes `mise run app` and `mise run web` shortcuts and pins the `wasm32-unknown-unknown` target plus Trunk.

## Workspace architecture

Four crates with strict, deliberate separation:

- **`crates/iced-longbridge`** — the published component library. ~65 components under `src/components/` plus `theme.rs` and `styles.rs`. No demo or app code lives here. Depends on `iced` with `tokio` enabled — fine on native, and the WASM target re-features `iced` without `tokio` in `demo-web`.
- **`crates/demo`** — the shared application: `State`, `Message`, `update`, `view`, `subscription`, the sidebar, command palette, and one `demos/<name>_demo.rs` per page (~66 files). Both binaries depend on this crate; neither holds significant logic of its own. Re-exports `iced_longbridge::{components, theme, lucide}` so demo files can write `crate::components::…`.
- **`crates/demo-app`** — native binary (`showcase`). Builds `iced` with the native renderer and `tokio`.
- **`crates/demo-web`** — WASM binary served by Trunk. Uses the `webgl` renderer (no Vulkan / Metal / DX12 in browsers), bundles `Ubuntu-R.ttf` with `include_bytes!` because cosmic-text can't access system or CSS fonts in WASM, and *omits* `tokio` from `iced`'s features. The `demo` crate enables `chrono/wasmbind` so `chrono::Local::now()` works in the browser. Match these per-target feature splits when adding dependencies — getting them wrong typically only fails on the WASM build.

Rust edition 2024, `iced = "0.14"`, workspace-pinned version. `unsafe_code` is **forbidden** workspace-wide (`Cargo.toml` lints). `too_many_arguments` clippy lint is allowed because component constructors are intentionally wide (e.g. `button_ex` takes 7 params); don't refactor away wide signatures just to satisfy clippy.

## Theme & sizing system

Components are stateless functions that take `&AppTheme` and read colors at render time — there is no styling registry or builder pattern. Toggle light/dark by swapping `AppTheme::light()` ↔ `AppTheme::dark()` in `State`; every component re-reads on the next frame. When writing or modifying a component, take `theme: &AppTheme`, dereference it (`let theme = *theme;`) before moving into a `style` closure, and pull colors via field access (`theme.background`, `theme.muted_foreground`, …).

`Size { Xs, Sm, Md, Lg }` is the size variant used across all interactive components. Layout values come from `Size::metrics()` in `theme.rs` — the **single source of truth**. If you add a new component that needs sized variants, call `size.height()`, `size.padding_x()`, `size.font_size()`, `size.radius()` rather than introducing parallel constants.

## Adding a new component

1. New file under `crates/iced-longbridge/src/components/<name>.rs`, registered in `components/mod.rs`.
2. Public constructor takes `theme: &AppTheme` (and `Size` if relevant) and returns `Element<'a, Message>` generic over the caller's `Message`.
3. To showcase it: add a `Page` variant + label to `Page::ALL` in `crates/demo/src/lib.rs`, a `<name>_demo.rs` file under `crates/demo/src/demos/`, register it in `demos/mod.rs`, branch in `build_content` to call its `view`, add a `page_description` entry, and add any backing fields to `State` + `Default` + `Message` + `update`.

## Release flow

Two paths land a release on `master`, both producing a `vX.Y.Z` tag:

- **Tag push** (`v*`) → `release-on-tag.yml` creates a GitHub release from the tag's annotation.
- **PR merge to master** → `release-on-merge.lock.yml` (an [agentic workflow](https://github.com/githubnext/gh-aw) compiled from `release-on-merge.md`) reads the merged PR, picks a semver bump, drafts notes, then a downstream `publish-release` job bumps `[workspace.package].version` in `Cargo.toml`, builds, tags, releases, and `cargo publish -p iced-longbridge`. Only `iced-longbridge` is published; `demo`, `demo-app`, `demo-web` stay local. Re-running release for a no-code change is done by editing `.github/release-trigger.json` (the `release-on-tag` workflow watches that path).

Don't hand-edit `[workspace.package].version` — let the release workflow bump it. The workflow expects a `CARGO_REGISTRY_TOKEN` repo secret.

## Docker / WASM serve

`Dockerfile` is a two-stage build: Rust + a checksum-verified pinned Trunk binary builds the WASM bundle, then `nginx:stable-alpine` serves `crates/demo-web/dist`. `nginx.conf` is the production static-serve config. Builds for both `linux/amd64` and `linux/arm64`.
