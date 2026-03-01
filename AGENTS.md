# AGENTS.md

## Project Overview

A trip-planning & hype app for a 21-day China itinerary (Oct 2026), 7 friends 


## Stack
- Backend: Rust, Axum, SQLite, Oauth2, ts-rs
- Frontend: Typescript + Svelte 5 + Vite

**Run these before each commit:**

```bash
# From /backend
cargo check          # fast compile check + warnings
cargo test           # runs all unit and integration tests
```

```bash
# From /frontend
npx vite build       # full build — surfaces all Svelte a11y/compiler warnings
```
