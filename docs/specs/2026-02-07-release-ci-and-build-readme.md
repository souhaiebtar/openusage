# Spec: Release CI + Build Docs

Date: 2026-02-07

## Goal

- Document local build flow clearly in `README.md`.
- Ensure GitHub Actions publishes release assets to GitHub Releases for Windows only.

## Scope

- `README.md`
- `.github/workflows/publish.yml`

## Behavior

- Release workflow runs on `v*` tags (and manual dispatch).
- Builds Windows assets.
- Publishes assets to GitHub Release.

## Acceptance

- README contains actionable build + release instructions.
- Workflow publishes release artifacts for Windows.
