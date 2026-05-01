---
Status: Active
Decision: accepted
Date: 2026-05-01
Scope: phase-29cv P122, MIR global call target shape rejection reason
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P121-GLOBAL-CALL-CANONICAL-TARGET-SYMBOL.md
  - src/mir/global_call_route_plan.rs
---

# P122: Global Call Target Shape Reason

## Problem

After P121, `Main.main` reaches existing same-module targets and the stop moved
from `unknown_global_callee` to `missing_multi_function_emitter`.

The next blocker is no longer name resolution. It is deciding which target body
shape to make lowerable next. Without MIR-owned rejection evidence, ny-llvmc or
humans must inspect raw target body JSON again, which reintroduces a second
classifier outside the MIR route plan.

## Decision

Add `target_shape_reason` to `global_call_routes` / `lowering_plan` when all of
these are true:

- `target_exists=true`
- `target_shape=null`
- the MIR target classifier can explain the non-match

This keeps the ownership split:

```text
MIR route plan owns target shape and target shape rejection evidence.
ny-llvmc consumes target_symbol/target_shape and may only report the reason.
```

## Current Evidence

For the full `stage1_cli_env.hako` path, the first P121 blocker now carries:

```text
callee_name=main._run_emit_program_mode/0
target_symbol=Main._run_emit_program_mode/0
reason=missing_multi_function_emitter
target_shape_reason=generic_string_unsupported_instruction_or_call
```

That means the next BoxCount should not be a raw `Main._run_emit_program_mode`
special case. The target body contains same-module calls whose target shapes
are not lowerable yet, so the next card must choose a narrow callable body
shape and emit definitions before enabling the direct call.

## Rules

Allowed:

- expose stable MIR-owned target shape rejection reason strings
- keep `reason` as the backend-owner stop (`missing_multi_function_emitter`)
- keep `callee_name` diagnostic and `target_symbol` canonical

Forbidden:

- backend-local target body reclassification
- raw by-name handling for `Main._run_emit_*`
- widening the function emitter in the same card
- changing VM/source-execution behavior

## Acceptance

- `cargo fmt --check` succeeds.
- `cargo test -q global_call_routes` succeeds.
- `cargo build --release --bin hakorune` succeeds.
- generated `stage1_cli_env.hako` MIR carries `target_shape_reason` on the
  first existing unsupported same-module target.
