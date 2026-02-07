# Spec: Unsigned Build

Date: 2026-02-07

## Goal

Allow `bun run tauri build` without updater private key signing.

## Scope

- Tauri config updater artifact generation flag.

## Behavior

- Disable updater artifact generation to avoid requiring `TAURI_SIGNING_PRIVATE_KEY`.

## Acceptance

- `bun run tauri build` no longer fails with missing signing private key.
