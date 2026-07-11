---
Status: Landed
Date: 2026-06-14
Task: COLL-VISIBLE-CLOSEOUT-001
Scope: Close the Buffer/String/Map/Array visible semantics lift lane and return the compiler foundation lane to CorePlan / JoinIR expressivity.
Related:
  - docs/development/current/main/workstreams/compiler-foundation-current.md
  - docs/development/current/main/design/selfhost-lift-boundary-and-task-order-ssot.md
  - lang/src/runtime/collections/README.md
  - tools/hako_check/README.md
  - tools/hako_check/collection_visible_contract.py
---

# COLL-VISIBLE-CLOSEOUT-001: Collection Visible Semantics Closeout

## Decision

The collection visible semantics lane is closed out for Buffer, String, Map,
and Array.

```text
collection_visible_semantics_closeout_ready=1
collection_storage_substrate_owner_preserved=1
next_foundation_lane_selected=coreplan_joinir_expressivity
```

This lane moved visible method / alias / policy ownership into modular
`.hako` owner files without moving raw storage, VM dispatch, allocator, or
typed-object identity truth.

## Landed Shape

```text
Buffer:
  modular visible policy / bridge / core owner
  typed little-endian numeric policy owner
  compatibility facade preserved

String:
  modular visible policy / bridge / core owner
  VM-facing wrapper preserved

Map:
  modular visible policy / bridge / core owner
  VM-facing wrapper preserved

Array:
  modular visible policy / bridge / core owner
  VM-facing wrapper preserved
```

The hako_check contract is observation-only. It checks the `.hako` visible
owner files against fixture TSVs and reports the cutover boundary; it does not
become storage truth.

## Acceptance

```text
buffer_visible_policy_owner=runtime.collections.buffer.visible_policy_box
buffer_numeric_le_policy_owner=runtime.collections.buffer.numeric_le_policy_box
string_visible_policy_owner=runtime.collections.string.visible_policy_box
map_visible_policy_owner=runtime.collections.map.visible_policy_box
array_visible_policy_owner=runtime.collections.array.visible_policy_box
collection_storage_substrate_owner_preserved=1
collection_vm_dispatch_changed=0
summary=ok
```

## Proof Commands

```bash
bash tools/hako_check.sh collection-visible-contract
bash tools/hako_check.sh boxcall-contract --include-plugin-catalog-sample
python3 -m py_compile tools/hako_check/collection_visible_contract.py
cargo test -q --lib box_callable
cargo test -q --lib surface_catalog
```

## Stop Line

```text
do not make .hako collection policy raw storage truth
do not change VM handler dispatch in this lane
do not use collection visible owners to replace BoxCallableRegistry
do not start Arc retirement from this card
do not start concurrency MIR lowering from this card
```

## Next

```text
COREPLAN-JOINIR-RESTART-001:
  pick the next one-purpose CorePlan / JoinIR expressivity row.

Default next family:
  CorePlan / FlowBox / JoinIR responsibility cleanup and expressivity,
  not collection visible semantics and not exact-front optimization.
```
