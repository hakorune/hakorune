---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: audit the remaining temporary `GlobalCallTargetShape` capsules after the public-collapse wins
Related:
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - docs/development/current/main/phases/phase-29cv/P381AT-UNIFORM-MULTI-FUNCTION-EMITTER-GAP-PLAN.md
  - src/mir/global_call_route_plan/model.rs
---

# P381BA: Remaining Capsule Retire Inventory

## Problem

After the public-collapse wins for:

- `BuilderRegistryDispatchBody`
- `ProgramJsonEmitBody`
- `JsonFragInstructionArrayNormalizerBody`

the next step was to determine whether the remaining temporary capsules could
also be retired as public shapes/proofs without widening Stage0 semantics.

## Decision

No immediate public-collapse candidates remain in this lane.

Audited results:

- `StaticStringArrayBody`: superseded by P381BL
  - `array_handle` + `ORG_ARRAY_STRING_BIRTH` semantics now live in the
    proof/return contract rather than a target-shape variant
- `MirSchemaMapConstructorBody`: superseded by P381BM
  - `map_handle` + `ORG_MAP_BIRTH` semantics now live in the proof/return
    contract rather than a target-shape variant
- `BoxTypeInspectorDescribeBody`: superseded by P381BO
  - `map_handle` + `ORG_MAP_BIRTH` semantics now live in the proof/return
    contract rather than a target-shape variant; active source-owner callers
    already use scalar predicates
- `ParserProgramJsonBody`: superseded by P381BN
  - `string_handle` + `ORG_STRING` semantics now live in the proof/return
    contract rather than a target-shape variant; the dedicated body emitter
    remains a later cleanup item
- `GenericStringVoidLoggingBody`: superseded by P381BJ
  - P381BJ removed the target-shape variant after storing
    `proof=typed_global_call_generic_string_void_logging` and
    `return_shape=void_sentinel_i64_zero` as target facts
- `PatternUtilLocalValueProbeBody`: superseded by P381BP
  - the mixed scalar/handle contract now lives in proof/return facts rather than
    a target-shape variant; child-probe recognition also reads those facts

## Boundary

Allowed:

- documenting that the remaining public-collapse pass has reached a plateau
- shifting the next action toward owner cleanup or the uniform MIR emitter

Not allowed:

- adding a new public contract just to keep the capsule-retirement count moving
- weakening existing blockers (`ParserBox`, logging, array/map origin handling)
  inside the current lane
- widening Stage0 semantics to rediscover source-owner meaning

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Done:

- the remaining temporary capsules were re-ranked by viability
- the current public-collapse pass is now explicitly recorded as exhausted

Next:

1. move to source-owner cleanup where a temporary capsule has an explicit owner
   removal path
2. otherwise continue with the uniform multi-function MIR emitter instead of
   inventing new public route shapes
