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
- Tailwind CSS: automatic via `asset!("/assets/tailwind.css")` containing `@import "tailwindcss";`

## Clippy Configuration

`clippy.toml` prevents holding Dioxus signal locks (`GenerationalRef`, `GenerationalRefMut`, `WriteLock`) across `.await` points — do not remove this.
