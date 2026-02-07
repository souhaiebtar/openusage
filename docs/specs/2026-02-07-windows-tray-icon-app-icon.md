# Spec: Use App Icon for Windows Tray

Date: 2026-02-07

## Goal

Use the app icon for Windows tray icons instead of `icons/tray-icon.png`.

## Scope

- Rust tray bootstrap icon (`src-tauri/src/tray.rs`)
- Frontend tray fallback icon resolution (`src/App.tsx`)
- Tauri bundled resources (`src-tauri/tauri.conf.json`)

## Behavior

- Windows tray icon should use `icons/icon.png` only.
- Non-Windows behavior remains unchanged (`icons/tray-icon.png`).

## Acceptance

- No direct Windows tray dependency on `icons/tray-icon.png` or `icons/icon.ico`.
- Windows runtime can resolve packaged app icon resources.
