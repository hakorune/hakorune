---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-038A.
Related:
  - docs/development/current/main/phases/phase-296x/296x-536-MIM-PORT-FMEM-038-ATOMIC-REMOTE-HEAD-DRAIN-EXCHANGE-SELECTION.md
  - src/mir/fastmem_access_plan.rs
---

# 296x-536A MIM-PORT-FMEM-038A FastMem FactStore Cleanup

## Purpose

Clean up the fact lookup seam before opening AtomicRemoteHead drain exchange
selection/lowering work.

The immediate issue was that `LocalFree`, `FreeHead`, and `AtomicRemoteHead`
plan construction each carried separate fact-slice plumbing and local lookup
helpers. That made the free-list family look simple while keeping the same
ownership/block-next/non-empty lookup decisions in multiple places.

## Decision

Introduce a narrow `FastMemFactStore` inside `fastmem_access_plan.rs`.

```text
FastMemFactStore:
  table_length_facts
  same_owner_facts
  remote_owner_facts
  block_next_facts
  local_free_non_empty_facts
  free_head_non_empty_facts
  range_index_facts
```

Plan builders now receive the store instead of independent fact slices.

## Non-Goals

```text
do not merge LocalFree / FreeHead / AtomicRemoteHead payload structs yet
do not introduce ResolvedHeadAccess yet
do not introduce FastMemLinkedListPlanCore yet
do not change lowerability or failure reasons
do not open AtomicRemoteHeadDrain lowering
do not change current blocker away from MIM-PORT-FMEM-038
```

## Follow-Up

The next cleanup layer can extract:

```text
ResolvedHeadAccess:
  layout_id / field_id / field_class / byte_offset / size / type / alignment

ResolvedBlockNextAccess:
  layout_id / field_id / field_class / byte_offset / size / type / alignment

FastMemLinkedListPlanCore:
  page / block / result / head access / block-next access / proof flags
```

That should be a separate BoxShape slice after MIM-038 if the drain exchange
work exposes more duplication.

## Verification

```bash
cargo test -q --lib atomic_remote_head
```
