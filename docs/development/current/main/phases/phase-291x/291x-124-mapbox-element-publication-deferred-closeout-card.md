---
Status: Landed
Date: 2026-04-24
Scope: Close stale MapBox keys/values element-publication deferred wording.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-98-mapbox-content-enumeration-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-99-mapbox-write-return-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-101-mapbox-get-missing-key-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-102-mapbox-keys-values-element-publication-card.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md
---

# 291x-124 MapBox Element Publication Deferred Closeout Card

## Decision

Mark older `keys()/values()` element-publication deferral notes as superseded by
`291x-102`.

This is docs-only BoxShape cleanup. It preserves each historical card's original
scope while making the current landed contract discoverable.

## Cleanup Scope

- add closeout notes to the content-enumeration card
- update old `Next Slice` sections that still say element publication is
  deferred
- point readers to `291x-102` for the landed sorted-key order and element
  publication contract

## Non-Goals

- no MapBox behavior changes
- no key/value ordering change
- no new MapBox rows
- no smoke changes

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
