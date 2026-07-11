---
Status: Completed
Date: 2026-05-11
Scope: M86b allocator provider lightweight docs sync policy.
Related:
  - docs/development/current/main/design/allocator-provider-lightweight-doc-sync-policy-ssot.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
  - docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md
---

# 293x-139 M86b Allocator Provider Lightweight Doc Sync Policy

## Summary

M86b fixes the docs workflow friction before M87.

Allocator provider rows M87 and later no longer need to update phase README,
phase taskboard, global mimalloc taskboard, and full task-breakdown progress
tables every time. Normal rows update their row SSOT/card, `CURRENT_STATE.toml`,
and guard wiring when needed. Heavy mirrors are reserved for closeout rows or
durable lane-policy changes.

## Boundary

This is docs/process only. It does not add runtime parsing, CLI routing,
provider selection, proof consumption, rollback preparation, activation gate
opening, hook activation, `#[global_allocator]`, process allocator replacement,
route widening, or `.inc` name matching.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
