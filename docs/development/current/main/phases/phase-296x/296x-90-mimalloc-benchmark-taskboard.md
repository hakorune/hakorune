---
Status: Active
Date: 2026-06-09
Scope: compact restart surface for the phase-296x mimalloc source-level lane.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/phase-296x-90-taskboard-history-2026-06-08.md
  - docs/development/current/main/investigations/docs-pointer-inventory-2026-06-09.md
  - docs/development/current/main/workstreams/mimalloc-current.md
---

# 296x-90 Mimalloc Benchmark Taskboard

This card is now a thin pointer surface.
Keep the historical queue in the archive note and keep this card short.

## Rule

- keep the current mirrors thin
- keep provider/DLL activation closed
- no product allocator replacement claim
- no hook installation claim
- no winner claim

## Current State

Current lane and blocker pointers live in `CURRENT_STATE.toml`.
Long queue history lives in the archive note.

## Current Lane

```text
current_lane = read docs/development/current/main/CURRENT_STATE.toml
current_blocker_token = read docs/development/current/main/CURRENT_STATE.toml
latest_card_path = read docs/development/current/main/CURRENT_STATE.toml
```

## Archive Pointers

```text
docs/development/current/main/investigations/docs-pointer-inventory-2026-06-09.md
docs/development/current/main/investigations/mimalloc-current-history-2026-06-02.md
docs/development/current/main/investigations/phase-296x-90-taskboard-history-2026-06-08.md
```

## Restart Notes

- read `CURRENT_STATE.toml` first for the current blocker token
- use the inventory note for pointer hunting
- use the comparison note for the exact-front optimization sweep
- use the MapBox proof-bearing route SSOT before changing map lookup lowering
- keep current mirrors narrow and archive long queue history elsewhere
