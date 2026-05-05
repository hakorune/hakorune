---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: retire `ProgramJsonEmitBody` as a distinct `GlobalCallTargetShape` capsule
Related:
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - docs/development/current/main/phases/phase-29cv/P380W-BUILDBOX-EMIT-PROGRAM-JSON-CANONICAL-CALL.md
  - src/mir/global_call_route_plan.rs
  - src/mir/global_call_route_plan/model.rs
  - src/mir/global_call_route_plan/program_json_emit_body.rs
  - src/mir/global_call_route_plan/tests/shape_reasons.rs
  - src/runner/mir_json_emit/tests/global_call_routes/parser_program_json.rs
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc
---

# P381AY: Program JSON Emit Capsule Retire

## Problem

`ProgramJsonEmitBody` still existed as a temporary public route shape even though
its accepted contract already matched the existing generic pure string lane:

- same lowering tier and emit kind: direct same-module function call
- same return contract: `string_handle`
- same Stage0 call emission path once lowering-plan facts are read generically

That left an unnecessary split where MIR/LoweringPlan/Stage0 still carried a
dedicated public proof/shape for a source-owner-specific wrapper.

## Decision

Retire `ProgramJsonEmitBody` as a public route shape and collapse accepted
routes into `GenericPureStringBody`.

Implemented:

- removed the public `ProgramJsonEmitBody` shape/proof from the published route
  contract
- kept `program_json_emit_body.rs` only as a narrow MIR-side acceptance shim so
  current source-owner wrappers can still be recognized safely
- updated MIR JSON and route tests to expect
  `typed_global_call_generic_pure_string`
- removed the remaining Stage0 direct-call shell branch for the old
  `program_json_emit_body` contract

## Boundary

Allowed:

- collapsing a temporary public route shape into an existing contract-equivalent
  string shape
- preserving the narrow MIR-side body recognizer while owner cleanup is still
  pending

Not allowed:

- teaching Stage0 new Program(JSON) wrapper semantics
- widening generic string rules beyond the existing string-handle contract
- folding other temporary capsules in the same card

## Acceptance

```bash
cargo fmt --all
cargo test --release shape_reasons -- --nocapture
cargo test --release parser_program_json -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

## Result

Done:

- `ProgramJsonEmitBody` no longer exists as a public MIR/LoweringPlan shape
- accepted emit-wrapper routes now reuse `generic_pure_string_body`
- Stage0 no longer carries a dedicated direct-call shell branch for the old
  public contract

Next:

1. continue capsule retirement with the next thin string-handle candidate
2. current best next candidate is `JsonFragInstructionArrayNormalizerBody`
