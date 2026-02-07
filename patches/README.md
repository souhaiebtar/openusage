# Patches

This folder contains portable patch files for re-applying local changes onto an updated codebase.

## Files

- `souhaiebtar-all-changes.patch`
  - Mailbox patch bundle generated with `git format-patch`.
  - Contains 8 commits.

## Commit Order (already embedded in patch)

When applied, commits are replayed in this order:

1. `a96c00f` feat: add Windows support with system tray integration
2. `dc4147a` fix(windows): improve tray icon visibility and panel behavior
3. `7d4e2a6` docs: add Windows build instructions and polish dark mode UI
4. `e9aeaef` key related
5. `90a51d7` fix(windows): use static icon.png tray icon and restore right-click menu
6. `d99e4fe` fix(copilot): support gh auth token fallback on windows/linux
7. `ff2c7d8` fix(cursor): detect auth on windows and remove sqlite3 cli dependency
8. `0b70a8a` fix(ui): hide updater error text in footer

## How To Apply (recommended)

From repo root:

```bash
git am -3 patches/souhaiebtar-all-changes.patch
```

- `-3` enables 3-way merge for better conflict handling on changed upstream code.
- This replays commits in the correct order automatically.

## If Conflicts Happen

1. Resolve conflicts in files.
2. Stage resolved files:

```bash
git add <resolved-file>
```

3. Continue:

```bash
git am --continue
```

Useful abort command:

```bash
git am --abort
```

## Alternative (non-commit apply)

If you only want file changes (no commit history):

```bash
git apply --3way patches/souhaiebtar-all-changes.patch
```

Then commit manually.
