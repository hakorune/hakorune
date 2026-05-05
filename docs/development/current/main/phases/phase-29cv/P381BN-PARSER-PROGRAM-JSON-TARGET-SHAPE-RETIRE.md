---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: retire `ParserProgramJsonBody` as a `GlobalCallTargetShape` while keeping its proof/return/origin contract.
Related:
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P381BC-STAGE0-CAPSULE-EXIT-TASK-MAP.md
  - src/mir/global_call_route_plan/model.rs
---

# P381BN: Parser Program(JSON) Target-Shape Retire

## Problem

`ParserProgramJsonBody` still existed as a target-shape variant even though the
route already had a narrow direct contract:

```text
proof=typed_global_call_parser_program_json
return_shape=string_handle
value_demand=runtime_i64_or_handle
```

That left a temporary Stage0 shape for a contract that can be expressed by
LoweringPlan proof/return facts.

## Decision

Retire `ParserProgramJsonBody` as a `GlobalCallTargetShape`.

Parser Program(JSON) targets now publish:

```text
target_shape=null
proof=typed_global_call_parser_program_json
return_shape=string_handle
value_demand=runtime_i64_or_handle
```

The C-side direct-call predicate reads proof/return facts instead of a shape
string, so string-origin propagation remains tied to the LoweringPlan contract.

## Boundary

Allowed:

- keep the existing parser Program(JSON) recognizer as the source of this
  narrow MIR-owned contract
- keep the dedicated `emit_parser_program_json_function_definition` path for
  now
- update route JSON tests to assert `target_shape=null`

Not allowed:

- add a replacement target-shape variant
- infer parser Program(JSON) semantics from a raw source owner name at direct
  call sites
- delete the dedicated body emitter in the same target-shape retirement card

## Acceptance

```bash
cargo test --release parser_program_json -- --nocapture
cargo test --release build_mir_json_root_emits_direct_plan_for_parser_program_json_contract -- --nocapture
cargo test --release global_call_route_plan -- --nocapture
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Done:

- `ParserProgramJsonBody` removed from `GlobalCallTargetShape`
- parser Program(JSON) classification now uses
  `direct_contract(ParserProgramJson, StringHandle)`
- C parser Program(JSON) global-call predicate no longer requires
  `target_shape`
- MIR route JSON and LoweringPlan JSON tests now expect `target_shape=null`
