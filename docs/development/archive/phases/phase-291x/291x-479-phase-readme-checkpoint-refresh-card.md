---
Status: Landed
Date: 2026-04-27
Scope: Refresh stale phase README checkpoint after GenericMethodRoute cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-478-generic-method-route-visibility-closeout-card.md
---

# 291x-479: Phase README Checkpoint Refresh

## Result

Refreshed the phase README checkpoint after the GenericMethodRoute root export
and visibility prune lanes closed.

This is docs-only. It does not change task ordering, code, `.inc` behavior, or
gate expectations.

## Why

The phase README still named `291x-472` and the already-closed root re-export
prune as the next cleanup. The current SSOT already pointed to `291x-478`, so
the README checkpoint was stale mirror text.
