---
Status: Active
Decision: accepted
Date: 2026-05-01
Scope: phase-29cv P123, ny-llvmc global-call target shape reason trace
Related:
  - docs/development/current/main/design/ai-handoff-and-debug-contract.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P122-GLOBAL-CALL-TARGET-SHAPE-REASON.md
  - lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
---

# P123: Global Call Target Shape Reason Trace

## Problem

P122 made MIR emit `target_shape_reason`, but ny-llvmc still only reported the
backend-owner stop reason:

```text
reason=missing_multi_function_emitter
```

That preserves fail-fast ownership, but it hides the MIR-owned target-shape
rejection evidence at the final unsupported-shape line. The next implementer
would still need to inspect raw MIR JSON to choose the next narrow shape.

## Decision

Read `target_shape_reason` through `LoweringPlanGlobalCallView` and surface it
on the dev-only pure-first unsupported-shape inventory line:

```text
[llvm-pure/unsupported-shape] ... reason=missing_multi_function_emitter target_shape_reason=generic_string_unsupported_instruction_or_call
```

This is observability only.

## Rules

Allowed:

- add `target_shape_reason` to the typed C lowering-plan view
- report the field on the existing dev-only unsupported-shape trace
- keep `reason` unchanged as the backend-owner stop

Forbidden:

- using `target_shape_reason` as permission to emit a same-module call
- backend-local target body reclassification
- widening any target shape in this card
- changing VM/source-execution or vm-hako behavior

## Current Evidence

For full `lang/src/runner/stage1_cli_env.hako`, the expected first stop remains:

```text
reason=missing_multi_function_emitter
target_shape_reason=generic_string_unsupported_instruction_or_call
```

The next BoxCount should still be selected from MIR shape ownership, not from a
raw backend name/body matcher.

## Acceptance

- `bash tools/build_hako_llvmc_ffi.sh` succeeds.
- `cargo build --release --bin hakorune` succeeds.
- `git diff --check` succeeds.
- `tools/checks/current_state_pointer_guard.sh` succeeds.
- With `NYASH_LLVM_ROUTE_TRACE=1`, full `stage1_cli_env.hako` still fails
  fast, but the final unsupported-shape line includes `target_shape_reason`.
