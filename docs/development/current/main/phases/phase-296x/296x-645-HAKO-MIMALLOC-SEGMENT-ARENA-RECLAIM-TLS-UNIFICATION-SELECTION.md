---
Status: Landed
Date: 2026-06-09
Scope: select the next medium-priority segment arena reclaim/TLS seam for the .hako mimalloc port.
Blocker: HAKO-MIMALLOC-SEGMENT-ARENA-RECLAIM-TLS-UNIFICATION-296X-001
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-66-HAKO-MIMALLOC-PORT-FEATURE-GAP-INVENTORY.md
  - docs/development/current/main/workstreams/mimalloc-current.md
  - lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_box.hako
  - lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_gate_box.hako
---

# 296x-645 Hako Mimalloc Segment Arena Reclaim TLS Unification Selection

## Purpose

Select the next remaining segment-arena seam that composes reclaim, TLS, and
release/recycle facts behind one read-only integration owner.

This row does not activate provider replacement, install hooks, claim a global
allocator, or mutate page state. It only selects the next gap after the unified
production allocator API selection rows and keeps the seam read-only.

## Required Input

```text
output_contract=hako-mimalloc-port-feature-gap-inventory-v0
selected_feature=segment_arena_reclaim_tls_unification
missing_feature_count=7
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Required Output

```text
output_contract=hako-mimalloc-segment-arena-reclaim-tls-unification-selection-v0
selected_feature=segment_arena_reclaim_tls_unification
selector_ready=1
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Evidence

```text
output_contract=hako-mimalloc-port-feature-gap-inventory-v0
implemented_surface_count=12
missing_feature_count=7
missing_3_feature=segment_arena_reclaim_tls_unification
missing_3_priority=medium
next_port_feature=segment_arena_reclaim_tls_unification
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Guard

```text
docs/development/current/main/CURRENT_STATE.toml
docs/development/current/main/workstreams/mimalloc-current.md
docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
docs/development/current/main/phases/phase-296x/296x-66-HAKO-MIMALLOC-PORT-FEATURE-GAP-INVENTORY.md
```

## Stop Line

Do not activate provider replacement, hooks, global allocator claims, or any
LD_PRELOAD shim in this row. This row only opens the next medium-priority
segment-arena selection surface.
