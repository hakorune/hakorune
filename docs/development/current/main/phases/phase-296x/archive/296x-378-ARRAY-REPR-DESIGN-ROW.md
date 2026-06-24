---
Status: Landed
Date: 2026-05-30
Scope: write the ArrayRepr bridge SSOT after the post-retirement perf owner refresh.
Blocker: HAKO-ARRAYCORE-OWNER-ALIGNMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-377-ARRAY-SLOT-NATIVEDIRECT-POST-RETIREMENT-PERF-OWNER-REFRESH.md
  - docs/development/current/main/phases/phase-296x/296x-379-HAKO-ARRAYCORE-OWNER-ALIGNMENT.md
  - docs/development/current/main/phases/phase-296x/296x-374-DIRECTARRAY-FAMILY-STORAGE-SUBSTRATE-ROADMAP.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/representation-direct-lowering-ssot.md
  - docs/development/current/main/design/representation-direct-storage-substrate-ssot.md
---

# 296x-378 ArrayRepr Design Row

## Purpose

Write the `ArrayRepr` SSOT for the bridge from the public `ArrayBox` facade to
the `DirectArray` storage family.

This row is docs-first only. It does not implement array lowering and does not
change `nyash.array.birth_h`.

## Contract

```text
output_contract=array-repr-ssot-v0
input_contract=array-slot-nativedirect-post-retirement-perf-owner-refresh-v0
array_repr_ssot_path=docs/development/current/main/design/array-repr-ssot.md
array_repr_variants=DirectI64|PublicArrayBoxFallback
public_arraybox_facade=1
directarray_family_storage_substrate=1
materialization_route=explicit
plugin_internals_as_abi=0
nyash_array_birth_h_behavior_change=0
selected_next=array_hako_arraycore_owner_alignment_row
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The lane now has an explicit `ArrayRepr` bridge:

- `DirectI64` for exact direct storage
- `PublicArrayBoxFallback` for the public facade and mixed-storage fallback

This row keeps the public `ArrayBox` facade intact and freezes the direct
storage bridge as a design SSOT rather than a helper micro-lane.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_array_repr_design_ssot_guard.sh
```
