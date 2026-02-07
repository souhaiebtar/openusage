# Spec: Cursor Auth Path by OS

Date: 2026-02-07

## Goal

Detect Cursor login on Windows/Linux/macOS by reading token DB from the correct OS path.

## Scope

- `plugins/cursor/plugin.js`
- `docs/providers/cursor.md`

## Behavior

- Resolve `state.vscdb` path from `ctx.app.platform`.
- Windows: `~/AppData/Roaming/Cursor/User/globalStorage/state.vscdb`
- Linux: `~/.config/Cursor/User/globalStorage/state.vscdb`
- macOS/default: `~/Library/Application Support/Cursor/User/globalStorage/state.vscdb`

## Acceptance

- Windows users with valid Cursor auth no longer see "Not logged in. Sign in via Cursor app." due to wrong DB path.
