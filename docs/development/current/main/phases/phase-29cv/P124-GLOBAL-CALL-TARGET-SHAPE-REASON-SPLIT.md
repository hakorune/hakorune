---
Status: Active
Decision: accepted
Date: 2026-05-01
Scope: phase-29cv P124, MIR global-call target shape reason split
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P123-GLOBAL-CALL-TARGET-SHAPE-REASON-TRACE.md
  - src/mir/global_call_route_plan.rs
---

# P124: Global Call Target Shape Reason Split

## Problem

P123 surfaces `target_shape_reason` at the ny-llvmc stop line, but the dominant
reason was still too broad:

```text
generic_string_unsupported_instruction_or_call
```

That string mixed local unsupported operations, method calls, unsupported
extern/global surfaces, and child same-module targets whose own shape is still
unknown. The next BoxCount must choose one acceptance shape, so MIR needs to
separate those causes before any emitter widening.

## Decision

Keep `target_shape` unchanged, but split generic string rejection reasons for
call/instruction ownership:

- `generic_string_unsupported_instruction`
- `generic_string_unsupported_call`
- `generic_string_unsupported_method_call`
- `generic_string_unsupported_extern_call`
- `generic_string_unsupported_backend_global_call`
- `generic_string_global_target_missing`
- `generic_string_global_target_shape_unknown`

The old broad reason is retired from new MIR output. This is still classifier
evidence only; no new shape becomes lowerable in this card.

## Rules

Allowed:

- refine MIR-owned target-shape rejection evidence
- add unit tests for method-call and unknown-child-target causes
- keep ny-llvmc consuming the existing field unchanged

Forbidden:

- making method calls lowerable
- making child unknown targets lowerable
- backend-local body reclassification
- raw by-name handling for `Main._run_emit_*`

## Expected Evidence

For full `lang/src/runner/stage1_cli_env.hako`, the first stop should now carry:

```text
reason=missing_multi_function_emitter
target_shape_reason=generic_string_global_target_shape_unknown
```

That means `Main._run_emit_program_mode/0` is blocked by child target shapes,
not by a local backend name matcher.

## Acceptance

- `cargo fmt --check` succeeds.
- `cargo test -q global_call_routes` succeeds.
- `cargo build --release --bin hakorune` succeeds.
- `git diff --check` succeeds.
- `tools/checks/current_state_pointer_guard.sh` succeeds.
- generated `stage1_cli_env.hako` MIR shows the first unsupported target as
  `generic_string_global_target_shape_unknown`.
