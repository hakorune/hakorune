---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: retire `GenericStringOrVoidSentinelBody` from the Stage0 target-shape inventory
Related:
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P381BE-UNIFORM-BODY-EMITTER-CONTRACT-INVENTORY.md
  - src/mir/global_call_route_plan/model.rs
  - src/mir/generic_method_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
---

# P381BQ: Generic String-Or-Void Sentinel Target-Shape Retire

## Problem

`GenericStringOrVoidSentinelBody` was the last temporary
`GlobalCallTargetShape` capsule in the Stage0 inventory. It duplicated facts
already carried by the route proof and return contract:

- `proof=typed_global_call_generic_string_or_void_sentinel`
- `return_shape=string_handle_or_null`

Keeping the extra target-shape string made downstream consumers depend on a
body capsule instead of the explicit MIR-owned contract.

## Decision

Retire `GenericStringOrVoidSentinelBody` as a public target-shape variant.

MIR still classifies accepted sentinel helpers as direct ABI calls, but the
published contract is now:

```text
proof=typed_global_call_generic_string_or_void_sentinel
return_shape=string_handle_or_null
target_shape=null
```

Generic-method string-origin consumers read proof/return facts for both pure
string and nullable string sentinel direct routes. The C lowering-plan direct
generic-string predicate also accepts the sentinel contract through proof/return
facts rather than the retired shape string.

## Boundary

Allowed:

- remove only this target-shape variant
- keep the existing generic string body path for actual emission
- keep source-owner sentinel plumbing as a later owner/body cleanup

Not allowed:

- add a replacement public shape
- teach Stage0 source-owner helper meaning by callee name
- delete generic string body emitters in this card

## Acceptance

```bash
cargo test --release void_sentinel -- --nocapture
cargo test --release global_call_route_plan -- --nocapture
cargo test --release generic_method_route_plan -- --nocapture
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
bash tools/checks/dev_gate.sh quick
```

## Result

Done:

- `GenericStringOrVoidSentinelBody` is removed from
  `GlobalCallTargetShape`
- sentinel direct calls publish proof/return facts with `target_shape=null`
- generic-method route planning reads proof/return facts instead of the retired
  shape string
- the Stage0 inventory now contains only fail-fast/permanent target shapes

Next:

1. start uniform multi-function MIR emitter cleanup
2. consolidate `.inc` capsule body emitters after direct-call contracts stay
   green
