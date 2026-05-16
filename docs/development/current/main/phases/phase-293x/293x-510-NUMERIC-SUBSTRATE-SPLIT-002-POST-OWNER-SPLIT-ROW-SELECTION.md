# 293x-510 NUMERIC-SUBSTRATE-SPLIT-002 Post-Owner-Split Row Selection

Status: landed
Date: 2026-05-17

## Decision

`NUMERIC-SUBSTRATE-SPLIT-001` closed the numeric substrate owner-layout split.

Select exactly one next cleanup row:

```text
TYPED-OBJECT-STORAGE-INFERENCE-SPLIT-001:
  split typed-object storage inference into smaller orchestration and value
  analysis owners without changing typed_object_plans behavior
```

## Why This Row

The remaining large-file inventory now points at typed-object storage inference:

```text
src/mir/typed_object_plan/storage_inference.rs
src/mir/typed_object_plan/storage_inference/value_analysis.rs
```

These files own the same typed-object plan contract that the mimalloc and real
app lanes depend on, but they now mix orchestration, field/param origin maps,
collection element storage, value-origin analysis, and tests. The next cleanup
should make that ownership easier to inspect before more allocator rows depend
on typed object planning.

## Selected Row

```text
row:
  TYPED-OBJECT-STORAGE-INFERENCE-SPLIT-001
owner:
  src/mir/typed_object_plan/storage_inference.rs
  src/mir/typed_object_plan/storage_inference/
scope:
  split orchestration, origin maps, collection storage inference, value
  analysis, and tests behind the existing build_typed_object_plans entry
stop_line:
  no typed_object_plans JSON behavior change
  no field storage / param origin inference behavior change
  no backend/runtime/allocator/provider behavior
evidence:
  cargo test -q typed_object_plan::storage_inference
  bash tools/checks/current_state_pointer_guard.sh
  tools/checks/dev_gate.sh quick
  git diff --check
```

## Stop Lines

- Do not change `typed_object_plans` shape, layout kind, type-id ordering,
  field ordering, field storage classification, or observed newbox behavior.
- Do not change value-origin inference semantics or collection element storage
  decisions.
- Do not touch backend emission, allocator behavior, provider activation,
  hooks, host allocator replacement, or `#[global_allocator]`.

## Closeout

This row closes when `TYPED-OBJECT-STORAGE-INFERENCE-SPLIT-001` has a selected
current card with owner, scope, stop lines, and evidence.
