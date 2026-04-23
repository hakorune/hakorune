---
Status: Landed
Date: 2026-04-24
Scope: MapBox taskboard stale follow-up closeout after phase-291x landed rows.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-93-mapbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-97-mapbox-length-alias-card.md
  - docs/development/current/main/phases/phase-291x/291x-109-map-compat-source-cleanup-card.md
---

# 291x-120 MapBox Taskboard Closeout Card

## Decision

Close stale MapBox taskboard wording after the MapBox catalog, alias,
source-route, router, and cleanup cards have landed.

This is docs-only BoxShape cleanup. It does not change MapBox surface rows,
aliases, routing, or runtime behavior.

## Cleanup Scope

- remove `length` from the deferred list because `291x-97` landed it as a
  read-only alias of the existing size surface
- record landed MapBox follow-up cards in one completed list
- update the stale `291x-109` wording from remaining boundary cleanup to landed
  boundary cleanup
- keep actual future rows explicitly deferred: `getField` / `setField`,
  `birth`, `forEach`, `toJSON`, slot unification, compat deletion, and wider
  bad-key unification

## Non-Goals

- no `size` / `len` slot unification
- no non-vtable MapBox rows
- no compat ABI deletion
- no router policy edits

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
```

## Landing Snapshot

The MapBox taskboard now reads as a landed reference for current phase-291x rows
and leaves only future-risk rows in the deferred list.
