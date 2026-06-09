---
Status: Landed
Date: 2026-06-09
Scope: compose the next read-only segment-arena reclaim/TLS integration owner for the .hako mimalloc port.
Blocker: HAKO-MIMALLOC-SEGMENT-ARENA-RECLAIM-TLS-READONLY-INTEGRATION-296X-001
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/development/current/main/phases/phase-296x/296x-66-HAKO-MIMALLOC-PORT-FEATURE-GAP-INVENTORY.md
  - docs/development/current/main/phases/phase-296x/296x-645-HAKO-MIMALLOC-SEGMENT-ARENA-RECLAIM-TLS-UNIFICATION-SELECTION.md
  - docs/development/current/main/investigations/segment-arena-reclaim-tls-unification-ladder-2026-06-09.md
  - lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_box.hako
  - lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_gate_box.hako
  - lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_pointer_derived_lookup_prerequisite_box.hako
  - lang/src/hako_alloc/memory/worker_tls_pilot_box.hako
---

# 296x-646 Hako Mimalloc Segment Arena Reclaim TLS Readonly Integration

## Purpose

Compose the existing segment-arena reclaim/TLS proof surfaces into one
read-only integration owner. This row does not open provider activation,
replacement, hooks, or winner claims. It only makes the seam explicit so the
remaining medium-priority gap can be worked as one narrow owner instead of four
scattered diagnostics.

## Required Input

```text
output_contract=hako-mimalloc-segment-arena-reclaim-tls-unification-selection-v0
selected_feature=segment_arena_reclaim_tls_unification
selector_ready=1
missing_3_feature=segment_arena_reclaim_tls_unification
missing_3_priority=medium
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Required Output

```text
output_contract=hako-mimalloc-segment-arena-reclaim-tls-readonly-integration-v0
selected_feature=segment_arena_reclaim_tls_unification
matrix_present=1
support_gate_present=1
pointer_lookup_prerequisite_present=1
worker_tls_present=1
readonly_integration_ready=1
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
docs/development/current/main/investigations/segment-arena-reclaim-tls-unification-ladder-2026-06-09.md
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_box.hako
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_gate_box.hako
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_pointer_derived_lookup_prerequisite_box.hako
lang/src/hako_alloc/memory/worker_tls_pilot_box.hako
```

## Stop Line

Do not activate provider replacement, install hooks, claim a global allocator,
or add worker scheduling semantics in this row. Keep the owner read-only and
report-only until the next explicit implementation row opens.
