---
Status: Landed
Date: 2026-04-24
Scope: phase-291x docs/status closeout after the ArrayBox.slice receiver pin.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md
  - docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - docs/development/current/main/phases/phase-291x/291x-95-mapbox-hako-extended-route-cleanup-card.md
  - docs/development/current/main/phases/phase-291x/291x-96-corebox-router-unified-value-path-card.md
---

# 291x-119 Docs Status Closeout Card

## Decision

Treat the current phase-291x CoreBox surface work as a landed reference lane for
the already-promoted StringBox, ArrayBox, and MapBox rows. Keep the active lane
open only for selecting future cleanup cards.

This is a BoxShape-only docs cleanup. It does not add surface rows, change route
policy, or alter runtime behavior.

## Cleanup Scope

- close stale `Status: Active` / `Active follow-up` wording on landed reference
  docs
- fix the phase README body date to match current docs
- fill the restart `current focus` line
- remove stale `MapBox implementation` from the StringBox deferred list because
  MapBox catalog and current router rows are already landed
- update the MapBox `values()` design brief note from storage order to the
  landed sorted-key order contract
- turn the 291x-95 and 291x-96 docs into landed reference cards

## Non-Goals

- no BoxCount work
- no new StringBox rows such as `split` / `startsWith` / `endsWith` / `charAt`
- no MapBox non-vtable rows such as `getField` / `setField` / `birth` /
  `forEach` / `toJSON`
- no `size` / `len` slot unification
- no `map_compat.rs` deletion

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
```

Optional smoke if a later card touches router wording again:

```bash
cargo test -q router
```

## Landing Snapshot

- phase-291x remains the active restart lane for CoreBox contract cleanup.
- 291x landed reference docs no longer imply unfinished MapBox or current
  router-row work.
- future work must choose a new cleanup card and keep BoxShape closeout separate
  from BoxCount feature rows.
