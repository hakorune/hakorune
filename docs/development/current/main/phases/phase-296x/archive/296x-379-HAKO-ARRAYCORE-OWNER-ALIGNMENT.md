---
Status: Landed
Date: 2026-05-30
Scope: align the lane with the existing .hako ring1 ArrayCore owner docs without moving semantics back into plugin internals.
Blocker: DIRECTARRAY-FAMILY-EXTENSION-GATE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-378-ARRAY-REPR-DESIGN-ROW.md
  - docs/development/current/main/phases/phase-296x/296x-380-DIRECTARRAY-FAMILY-EXTENSION-GATE.md
  - docs/development/current/main/phases/phase-296x/296x-374-DIRECTARRAY-FAMILY-STORAGE-SUBSTRATE-ROADMAP.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/array-map-owner-and-ring-cutover-ssot.md
  - docs/development/current/main/design/collection-raw-substrate-contract-ssot.md
---

# 296x-379 .hako ArrayCore Owner Alignment

## Purpose

Record the owner split that keeps the lane aligned with the existing `.hako`
ring1 collection semantics docs.

This is a cross-reference row only. It does not move collection semantics back
into Rust/private plugin internals and does not change `nyash.array.birth_h`.

## Contract

```text
output_contract=hako-arraycore-owner-alignment-note-v0
input_contract=array-repr-ssot-v0
hako_arraycore_visible_semantics_owner=1
stage0_rust_arrayseed_bootstrap_keep=1
directarray_family_storage_substrate=1
arraybox_public_facade=1
no_collection_semantic_migration=1
no_rust_private_layout_exposure=1
selected_next=directarray_family_extension_gate_row
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The owner split stays:

- `.hako ArrayCore` owns visible collection semantics
- `stage0` keeps the Rust ArraySeed for bootstrap/buildability/recovery
- `DirectArray family` remains the storage substrate
- `ArrayBox` remains the public materialized facade
- the next row is the DirectArray family extension gate

## Guard

```bash
bash tools/checks/k2_wide_phase296x_hako_arraycore_owner_alignment_guard.sh
```
