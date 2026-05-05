---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: decouple the static-array Rust consumer from `target_shape`
Related:
  - docs/development/current/main/phases/phase-29cv/P381BJ-VOID-LOGGING-TARGET-SHAPE-RETIRE.md
  - src/mir/generic_method_route_plan.rs
  - src/mir/generic_method_route_plan/tests/string_routes.rs
  - src/mir/global_call_route_plan/model.rs
---

# P381BK: Static Array Consumer Contract Read

## Problem

After proof and return contracts became stored target facts, most Rust
consumers no longer need to derive behavior from `GlobalCallTargetShape`.

One remaining Rust consumer still used
`target_shape=static_string_array_body` to recognize a static-array global call
result as an `ArrayBox` origin while building generic method routes.

That would block a later `StaticStringArrayBody` shape retirement even though
the route already carries the narrower contract facts:

- `proof=typed_global_call_static_string_array`
- `return_shape=array_handle`

## Decision

Make the generic-method route consumer read the stored proof/return contract
instead of the static-array target shape.

This is behavior-preserving. `StaticStringArrayBody` remains in the target-shape
inventory in this card; only the downstream Rust reader stops depending on it.

## Acceptance

```bash
cargo test --release static_string_array -- --nocapture
cargo test --release records_runtime_data_array_len_from_static_array_global_contract -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Done:

- static-array `ArrayBox` origin seeding now reads proof/return contract facts
- a targeted generic-method route test covers the shape-less static-array
  global-call contract
- no target-shape variant was removed
- no C shim predicate was changed

Next:

Superseded by
`docs/development/current/main/phases/phase-29cv/P381BL-STATIC-ARRAY-TARGET-SHAPE-RETIRE.md`,
which moved the C direct predicate to proof/return checks and retired
`StaticStringArrayBody` as a target-shape variant.
