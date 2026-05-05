---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: retire `GenericStringVoidLoggingBody` as a `GlobalCallTargetShape` variant
Related:
  - docs/development/current/main/phases/phase-29cv/P381BI-GLOBAL-CALL-PROOF-CONTRACT-STORAGE.md
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - src/mir/global_call_route_plan/model.rs
---

# P381BJ: Void Logging Target Shape Retire

## Problem

`GenericStringVoidLoggingBody` still existed only to attach the same direct ABI
metadata to void/logging helpers:

- `proof=typed_global_call_generic_string_void_logging`
- `return_shape=void_sentinel_i64_zero`
- `value_demand=scalar_i64`

After P381BH and P381BI, those values are stored target facts. Keeping a
target-shape variant for the same truth would preserve duplicate authority.

## Decision

Remove `GenericStringVoidLoggingBody` from `GlobalCallTargetShape`.

Void/logging helper classification now returns a direct proof/return contract
with no target shape. This intentionally keeps lowering-plan compatibility for
proof, tier, emit kind, return shape, and value demand while making
`target_shape` null for the retired capsule.

Implemented:

- removed the `GlobalCallTargetShape::GenericStringVoidLoggingBody` variant
- added a direct-contract classifier path for void/logging helpers
- taught the classification fixpoint to compare stored proof and return
  contract facts, not only target shape
- updated void logging route tests to assert proof/return-shape compatibility
  with no target shape
- updated the Stage0 shape inventory and LoweringPlan JSON SSOT

## Acceptance

```bash
cargo test --release void_logging -- --nocapture
cargo test --release global_call_route_plan -- --nocapture
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Done:

- Stage0 target-shape inventory now has one fewer temporary capsule
- void/logging direct calls still lower through the same proof and
  `void_sentinel_i64_zero` contract
- no new C shim emitter or body-specific backend predicate was added

Next:

1. continue capsule retirement with an origin-carrying or source-owner cleanup
   candidate
2. only consolidate `.inc` emitter files after the remaining capsule contracts
   are retired or represented as MIR-owned facts
