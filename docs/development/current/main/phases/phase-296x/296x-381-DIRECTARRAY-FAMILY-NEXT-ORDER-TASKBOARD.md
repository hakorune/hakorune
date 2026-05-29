---
Status: Landed
Date: 2026-05-30
Scope: choose the next DirectArray family work order after the extension gate.
Blocker: DIRECTI64-ARRAYREPR-FACT-INVENTORY-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-380-DIRECTARRAY-FAMILY-EXTENSION-GATE.md
  - docs/development/current/main/phases/phase-296x/296x-382-DIRECTI64-ARRAYREPR-FACT-INVENTORY.md
  - docs/development/current/main/phases/phase-296x/296x-378-ARRAY-REPR-DESIGN-ROW.md
  - docs/development/current/main/design/array-repr-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-374-DIRECTARRAY-FAMILY-STORAGE-SUBSTRATE-ROADMAP.md
---

# 296x-381 DirectArray Family Next Order Taskboard

## Purpose

Pick the next row order after the DirectArray family extension gate.

The important choice is to not add a new `DirectArray` member immediately.
`DirectI64` is already the first member, but its selected-method lowering path
is still too close to a special direct route. Before extending the family, the
existing `DirectI64` path should be carried through the `ArrayRepr` bridge with
small, checkable rows.

## Contract

```text
output_contract=directarray-family-next-order-taskboard-v0
input_contract=directarray-family-extension-gate-v0
selected_boundary=directarray_family_next_order_taskboard
next_diagnostic=direct_i64_arrayrepr_fact_inventory
selected_next=direct_i64_arrayrepr_fact_inventory
new_directarray_member_selected=0
direct_i64_first_member_stays_primary=1
arrayrepr_bridge_must_precede_new_member=1
public_arraybox_facade_preserved=1
nyash_array_birth_h_behavior_change=0
silent_fallback_allowed=0
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Ordered Rows

### DA-SEQ-001: DirectI64 ArrayRepr Fact Inventory

Purpose: inventory the current `DirectI64` fact producers and consumers before
changing lowering.

Input:
- `array-repr-ssot.md`
- current `DirectArrayI64` constructor origin facts
- selected-method ArraySlot NativeDirect lowering code

Output:
- a read-only inventory row

Acceptance:
- list the current producer of direct-array origin facts
- list the current lowerer consumer
- list any helper-name or selected-method special routes that must be retired
- selected next row is the producer contract

Forbidden:
- no code changes
- no new `DirectArray` member
- no public `ArrayBox` behavior change

### DA-SEQ-002: DirectI64 ArrayRepr Producer Contract

Purpose: define the stable fact shape that says an array value has
`ArrayRepr::DirectI64`.

Input:
- DA-SEQ-001 inventory
- `array-repr-ssot.md`

Output:
- contract row for the producer/consumer handoff

Acceptance:
- producer fact has a stable name and owner
- lowerer consumes the fact without re-proving eligibility
- public `nyash.array.birth_h` stays unchanged
- silent fallback is a row failure

Forbidden:
- no implementation
- no helper-name inference in the lowerer
- no public handle reinterpretation

### DA-SEQ-003: DirectI64 ArrayRepr Producer Implementation

Purpose: populate the agreed `ArrayRepr::DirectI64` fact for values produced by
`nyash.array.direct_i64.birth_h`.

Input:
- DA-SEQ-002 contract

Output:
- narrow producer implementation
- focused unit/smoke coverage

Acceptance:
- direct-array birth produces `ArrayRepr::DirectI64`
- public `nyash.array.birth_h` produces no direct repr
- unsupported cases produce no direct plan

Forbidden:
- no lowerer behavior change in this row
- no materialization policy change
- no new `DirectArray` member

### DA-SEQ-004: DirectI64 Lowering Consumer Rebase

Purpose: make selected-method ArraySlot NativeDirect lowering consume
`ArrayRepr::DirectI64` facts instead of ad hoc direct-origin state.

Input:
- DA-SEQ-003 producer implementation

Output:
- lowerer consumer rebase

Acceptance:
- direct load/store lowering requires `ArrayRepr::DirectI64`
- lowerer does not inspect public `ArrayBox` handles as direct pointers
- existing selected-method semantic smoke remains green

Forbidden:
- no generic ArrayBox rewrite
- no helper micro-optimization
- no silent fallback

### DA-SEQ-005: DirectI64 Materialization Smoke Refresh

Purpose: prove the rebased fact path still preserves public materialization and
fallback boundaries.

Input:
- DA-SEQ-004 consumer rebase

Output:
- semantic smoke row

Acceptance:
- public ArrayBox birth smoke stays green
- DirectArray birth smoke stays green
- DirectArray to public ArrayBox materialization smoke stays green
- selected-method direct lowering smoke stays green

Forbidden:
- no perf claim
- no new storage member

### DA-SEQ-006: Post-Rebase Perf Owner Refresh

Purpose: refresh the owner after the `ArrayRepr` bridge is real.

Input:
- DA-SEQ-005 semantic smoke

Output:
- perf owner refresh row

Acceptance:
- decide whether DirectArray remains the dominant owner
- decide whether a new member is justified
- if no new member is justified, select the next non-Array owner

Forbidden:
- no implementation
- no benchmark winner claim

### DA-SEQ-007: Optional Next Member Selection

Purpose: select a new `DirectArray` member only if DA-SEQ-006 shows a real
owner.

Input:
- DA-SEQ-006 perf owner refresh
- row380 extension gate

Output:
- one explicit member-selection row

Acceptance:
- exactly one member is selected
- explicit storage contract required before implementation
- materialization route required before implementation

Forbidden:
- no mixed-storage shortcut
- no implementation in the selection row
- no extension without perf evidence

## Decision

The next implementation should not be a new `DirectArray` member. The next work
order is to route the existing `DirectI64` path through `ArrayRepr` first:

```text
inventory
-> producer contract
-> producer implementation
-> lowerer consumer rebase
-> semantic smoke
-> perf owner refresh
-> optional next member selection
```

This keeps the family extensible without turning `DirectI64` into a permanent
special case.

Alternative considered:

```text
extension gate
-> new member selection
-> selected member storage contract
-> storage-only pilot
-> materialization policy
-> materialization snapshot
-> backend readiness
```

That order is valid after `ArrayRepr` is the active bridge. It is intentionally
not first here, because choosing `DirectBool` / `DirectF64` / `DirectHandle`
before rebasing the existing `DirectI64` path would leave the first member as a
special path and make later members copy that shape.

If DA-SEQ-007 later selects a member, the follow-on order is:

```text
selected member storage contract
-> storage-only pilot
-> materialization policy selection
-> materialization snapshot pilot
-> backend connection / readiness inventory
-> lowering guard surface
```

The selected member implementation still requires the row380 gate:

- explicit storage contract
- explicit materialization route
- preserved public ArrayBox facade
- no mixed-storage shortcut
- no silent fallback

The row is now landed because the next work order is fixed. The current row is
the `DirectI64` fact inventory.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_directarray_family_next_order_taskboard_guard.sh
```
