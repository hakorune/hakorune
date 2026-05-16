# 293x-511 TYPED-OBJECT-STORAGE-INFERENCE-SPLIT-001

Status: landed
Date: 2026-05-17

## Decision

`TYPED-OBJECT-STORAGE-INFERENCE-SPLIT-001` is a BoxShape cleanup for typed-object
storage inference. It keeps the existing `build_typed_object_plans(...)` entry
stable while splitting the inference owner into smaller modules.

## Scope

- Keep `build_typed_object_plans(module: &MirModule) -> Vec<TypedObjectPlan>`
  as the public owner entry.
- Keep `src/mir/typed_object_plan/storage_inference.rs` as the facade and
  orchestration owner.
- Move coherent groups behind existing
  `src/mir/typed_object_plan/storage_inference/` modules:
  - field/param origin map types and update helpers
  - collection element storage inference
  - value-analysis helpers that are not the facade entry
  - focused tests where they make production owners thinner

## Stop Lines

- Do not change `typed_object_plans` JSON output, type-id ordering, field order,
  layout kind, observed empty-box behavior, or declared/untyped field storage
  classification.
- Do not change value-origin, same-module method target, collection storage, or
  param-origin inference semantics.
- Do not add new accepted MIR shapes.
- Do not touch backend emission, allocator/provider behavior, hooks, host
  allocator replacement, or `#[global_allocator]`.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `TO.1` | Inventory current storage inference modules and choose the first move. | focused tests pass before edits. | no behavior change |
| `TO.2` | Move map/type aliases and simple inference structs/helpers behind a module if it reduces root coupling. | compile and focused tests pass. | no API drift |
| `TO.3` | Split one value-analysis helper family out of `value_analysis.rs`. | focused tests pass. | no inference behavior change |
| `TO.4` | Keep tests focused and production owners navigable. | focused tests pass. | no test weakening |
| `TO.5` | Verify and close out. | required evidence is green. | no adjacent cleanup |

## Required Evidence

```text
cargo test -q typed_object_plan::storage_inference
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```

## Closeout

This row split typed-object storage inference into a smaller facade plus owner
modules:

```text
src/mir/typed_object_plan/storage_inference.rs
src/mir/typed_object_plan/storage_inference/state.rs
src/mir/typed_object_plan/storage_inference/merge.rs
src/mir/typed_object_plan/storage_inference/field_origin_inference.rs
src/mir/typed_object_plan/storage_inference/param_inference.rs
src/mir/typed_object_plan/storage_inference/collection_storage.rs
src/mir/typed_object_plan/storage_inference/type_facts.rs
src/mir/typed_object_plan/storage_inference/value_analysis.rs
```

The facade keeps `build_typed_object_plans(...)` stable and now owns the
high-level fixed-point orchestration. Field/param origin propagation, merge
policy, collection element storage inference, and shared type-fact helpers are
separate owner modules. Existing typed-object plan behavior, MIR shape
acceptance, backend/runtime behavior, and allocator/provider behavior are
unchanged.

Evidence:

```text
cargo test -q typed_object_plan::storage_inference
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```
