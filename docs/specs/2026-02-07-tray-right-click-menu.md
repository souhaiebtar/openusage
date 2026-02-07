# Spec: Tray Right-Click Menu

Date: 2026-02-07

## Goal

Ensure right-click on tray icon shows tray menu.

## Scope

- `src-tauri/src/tray.rs` tray click event handling.

## Behavior

- Left click (button up) toggles panel.
- Right click does not toggle panel and is left for tray menu behavior.

## Acceptance

- Right-click tray menu is not blocked by panel toggle logic.
