# Spec: Embedded SQLite Host API

Date: 2026-02-07

## Goal

Make plugin SQLite APIs work without requiring external `sqlite3` CLI.

## Scope

- `src-tauri/src/plugin_engine/host_api.rs`
- `src-tauri/Cargo.toml`

## Behavior

- `host.sqlite.query` uses embedded SQLite (`rusqlite`) and returns JSON rows.
- `host.sqlite.exec` uses embedded SQLite for writes.
- No runtime dependency on `sqlite3` executable in `PATH`.

## Acceptance

- Cursor/Windsurf plugins can read SQLite state on Windows even when `sqlite3` is not installed.
