# Spec: Windows Static Tray Icon

Date: 2026-02-07

## Goal

Keep Windows tray icon fixed to `icons/icon.png`.

## Scope

- Frontend tray icon update pipeline in `src/App.tsx`.

## Behavior

- On Windows, skip dynamic tray icon rendering/updates.
- Initial tray icon from Rust (`icons/icon.png`) remains visible.

## Acceptance

- Windows tray icon does not switch to generated bar/text/provider icons.
