# Spec: Hide Update Failed Label

Date: 2026-02-07

## Goal

Remove red `Update failed` text from footer.

## Scope

- `src/components/panel-footer.tsx`
- `src/components/panel-footer.test.tsx`

## Behavior

- Update error state renders no left-side status text.

## Acceptance

- Footer never displays `Update failed` text.
