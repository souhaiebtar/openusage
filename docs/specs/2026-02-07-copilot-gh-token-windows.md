# Spec: Copilot GH Auth on Windows

Date: 2026-02-07

## Goal

Allow Copilot plugin to use existing `gh auth login` credentials on Windows.

## Scope

- `src-tauri/src/plugin_engine/host_api.rs` keychain bridge.

## Behavior

- On non-macOS, `host.keychain.readGenericPassword("gh:github.com")` falls back to `gh auth token`.
- Other keychain services remain macOS-only.

## Acceptance

- Copilot plugin can load token source `gh:github.com` on Windows when `gh auth status` is logged in.
