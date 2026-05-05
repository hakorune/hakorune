---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: retire `BuilderRegistryDispatchBody` as a distinct `GlobalCallTargetShape` capsule
Related:
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - docs/development/current/main/phases/phase-29cv/P381AT-UNIFORM-MULTI-FUNCTION-EMITTER-GAP-PLAN.md
  - src/mir/global_call_route_plan.rs
  - src/mir/global_call_route_plan/model.rs
  - src/mir/global_call_route_plan/tests/builder_registry_dispatch.rs
  - lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
---

# P381AX: Builder Dispatch Capsule Retire

## Problem

`BuilderRegistryDispatchBody` was still listed as a temporary capsule even though
its downstream contract already matched the existing string-or-void sentinel
lane:

- same return contract: `string_handle_or_null`
- same lowering tier/emit kind: direct same-module function call
- same Stage0 emission path once selected declarations/bodies were aligned

That left one unnecessary distinction:

- MIR/LoweringPlan/Stage0 carried a dedicated builder-dispatch shape/proof
- but consumers only needed the existing string-or-void sentinel contract

## Decision

Retire `BuilderRegistryDispatchBody` as a distinct route shape and collapse its
accepted routes into `GenericStringOrVoidSentinelBody`.

Implemented:

- removed the `BuilderRegistryDispatchBody` enum variant and dedicated proof
  mapping from `GlobalCallTargetShape`
- kept the narrow MIR-side builder-dispatch classifier only as a temporary
  acceptance shim so current source-owner bodies still classify safely
- changed accepted builder-dispatch routes to emit the existing
  `typed_global_call_generic_string_or_void_sentinel` proof / target shape
- removed builder-specific Stage0 lowering-plan readers and direct-call branches

## Boundary

Allowed:

- collapsing a temporary route shape into an existing contract-equivalent shape
- preserving the narrow MIR-side acceptance helper while the source-owner body
  still needs it

Not allowed:

- widening Stage0 body semantics
- claiming the source-owner builder-dispatch cleanup is finished
- changing unrelated temporary capsules in the same card

## Acceptance

```bash
cargo fmt --all
cargo test --release builder_registry_dispatch -- --nocapture
cargo test --release global_call_route_plan -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

## Result

Done:

- `BuilderRegistryDispatchBody` no longer exists as a public MIR/LoweringPlan
  shape
- builder-dispatch routes now reuse the existing string-or-void sentinel
  contract through MIR JSON and Stage0
- the Stage0 shape inventory shrank by one variant

Next:

1. continue capsule retirement with the next thin candidate
2. current best next candidate is `ProgramJsonEmitBody`
