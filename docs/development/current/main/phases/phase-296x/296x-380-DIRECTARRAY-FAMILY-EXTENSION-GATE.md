---
Status: Landed
Date: 2026-05-30
Scope: define the gate for adding a new DirectArray family member without reopening helper micro-optimization.
Blocker: DIRECTARRAY-FAMILY-NEXT-ORDER-TASKBOARD-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-379-HAKO-ARRAYCORE-OWNER-ALIGNMENT.md
  - docs/development/current/main/phases/phase-296x/296x-374-DIRECTARRAY-FAMILY-STORAGE-SUBSTRATE-ROADMAP.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/design/representation-direct-lowering-ssot.md
  - docs/development/current/main/design/representation-direct-storage-substrate-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-381-DIRECTARRAY-FAMILY-NEXT-ORDER-TASKBOARD.md
---

# 296x-380 DirectArray Family Extension Gate

## Purpose

Define when a new `DirectArray` family member may be added.

This is a gate row, not a member implementation row. It keeps the current
bridge stable and requires a new member to carry an explicit storage contract
before any implementation can open.

## Contract

```text
output_contract=directarray-family-extension-gate-v0
input_contract=hako-arraycore-owner-alignment-note-v0
selected_boundary=directarray_family_extension_gate_row
next_diagnostic=directarray_family_extension_gate_row
selected_next=directarray_family_next_order_taskboard
new_member_requires_explicit_storage_contract=1
materialization_route_required=1
public_arraybox_facade_preserved=1
silent_fallback_allowed=0
mixed_storage_shortcut_allowed=0
nyash_array_birth_h_behavior_change=0
new_member_implementation_open=0
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The gate is intentionally strict:

- any new `DirectArray` member must have an explicit storage contract
- the materialization route must be explicit
- the public `ArrayBox` facade stays preserved
- silent fallback and mixed-storage shortcuts stay forbidden
- `nyash.array.birth_h` behavior does not change here

This row does not add a new storage member. It only defines the gate that a
future member must pass.

The next row turns the gate into a small ordered taskboard. That taskboard must
avoid opening a new member before the existing `DirectI64` path is routed through
the `ArrayRepr` bridge instead of staying as a special direct-lowering path.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_directarray_family_extension_gate_guard.sh
```
