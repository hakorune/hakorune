---
Status: Landed
Date: 2026-04-24
Scope: CoreBox inventory ledger wording closeout after phase-291x landed rows.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
---

# 291x-122 CoreBox Inventory Ledger Closeout Card

## Decision

Close stale inventory wording in `291x-92` so the ledger separates:

- landed facts
- intentional compatibility boundaries
- future-risk rows

This is docs-only BoxShape cleanup. It does not change CoreBox runtime,
catalog, router, or VM behavior.

## Cleanup Scope

- rename stale `Remaining drift` sections that mostly describe landed or
  intentional boundaries
- mark StringBox public sugar / internal helper ownership as intentional, not
  unresolved drift
- mark MapBox first-slice inventory as landed history
- keep actual future-risk rows out of the current implementation path

## Non-Goals

- no StringBox deferred feature rows
- no MapBox non-vtable rows
- no `size` / `len` unification
- no compat ABI deletion
- no router policy edits

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
