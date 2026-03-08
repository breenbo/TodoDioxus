# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Serve Commands

```bash
# Serve a specific platform (run from workspace root)
cd packages/web && dx serve        # Web (fullstack)
cd packages/desktop && dx serve    # Desktop
cd packages/mobile && dx serve --platform android  # Mobile

# Check compilation
cargo check -p web --features web
cargo check -p web --features server
cargo check -p desktop --features desktop
cargo check -p ui
cargo check -p api --features server

# Lint
cargo clippy --workspace
```

No test suite exists yet. No Dioxus.toml config file is used.

## Architecture

This is a **Dioxus 0.7 fullstack workspace** with a multi-platform todo app. Five workspace members in `packages/`:

- **api** — Server-side business logic and database (SQLite via sqlx). All functions are feature-gated behind `#[cfg(feature = "server")]`. Defines server functions with `#[server]`, `#[get]`, `#[post]` macros.
- **ui** — Shared UI components (Hero, Navbar, Echo) used by all platforms. Must not have platform-specific dependencies. Depends on `api`.
- **web**, **desktop**, **mobile** — Platform entry points. Each has its own `Route` enum, views, and assets. Depends on `ui` + `dioxus`.

**Dependency flow:** platform crates → `ui` → `api`

**Feature flags pattern:** Each platform crate has a client feature (`web`/`desktop`/`mobile`) enabling `dioxus/<platform>`, and a `server` feature enabling `dioxus/server` + `ui/server` + `api/server`. The `server` feature gates sqlx and tokio in the api crate.

## Dioxus 0.7 Key Conventions

Refer to `AGENTS.md` for comprehensive Dioxus 0.7 API reference. Critical points:

- **No `cx`, `Scope`, or `use_state`** — these are removed in 0.7
- Components: `#[component] fn Name() -> Element` with `rsx! {}`
- State: `use_signal()` for local state, `use_memo()` for derived values
- Props must be owned (`String` not `&str`), implement `PartialEq + Clone`
- Server functions: `#[post("/api/path")]` / `#[get("/api/path")]` returning `Result<T, ServerFnError>`
- For fullstack hydration: use `use_server_future` instead of `use_resource`
- Assets: `asset!("/assets/file.ext")` macro (paths relative to crate root)
- Tailwind CSS: see "Tailwind CSS Setup" section below

## Tailwind CSS Setup

Dioxus 0.7 automatically processes Tailwind CSS v4 at build time via `dx serve`. It requires two files per platform crate:

1. **`packages/<platform>/tailwind.css`** — The Tailwind **input** file at the crate root. Contains `@import "tailwindcss"` and `@source` directives pointing to Rust source directories to scan for class names.
2. **`packages/<platform>/assets/tailwind.css`** — An **empty** output placeholder. Referenced by `asset!("/assets/tailwind.css")` in code. Dioxus compiles Tailwind output into this file during build.

The `@source` directives must point to all directories containing Tailwind class usage. In this workspace, each platform crate's input includes `@source "../ui/src"` to pick up classes from shared `ui` components, and `@source "./src"` for platform-specific views.

**Why per-crate and not at the workspace root:** Dioxus looks for `tailwind.css` at the crate root of whichever package is being served, not the workspace root. Since `ui` is never served directly (always consumed through a platform crate), it does not need its own Tailwind files.

**Important:** Do not put `@import "tailwindcss"` inside `assets/tailwind.css` — that file must stay empty. If it contains the raw import, Dioxus serves it as-is and the browser will 404 trying to fetch `tailwindcss` as a URL.

## Clippy Configuration

`clippy.toml` prevents holding Dioxus signal locks (`GenerationalRef`, `GenerationalRefMut`, `WriteLock`) across `.await` points — do not remove this.
