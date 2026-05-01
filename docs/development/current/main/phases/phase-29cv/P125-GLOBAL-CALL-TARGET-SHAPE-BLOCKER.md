---
Status: Active
Decision: accepted
Date: 2026-05-01
Scope: phase-29cv P125, MIR global-call target shape child blocker evidence
Related:
  - docs/development/current/main/design/ai-handoff-and-debug-contract.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P124-GLOBAL-CALL-TARGET-SHAPE-REASON-SPLIT.md
  - src/mir/global_call_route_plan.rs
---

# P125: Global Call Target Shape Blocker

## Problem

P124 made the first full `stage1_cli_env.hako` stop precise enough to say:

```text
target_shape_reason=generic_string_global_target_shape_unknown
```

That proves the entry target is blocked by a child same-module target, but it
still does not identify which child target owns the next slice. Without that
link, the next card would again inspect raw target bodies or pick a backend
name matcher by hand.

## Decision

When MIR classifies a target as shape-unknown because a child global target is
shape-unknown or missing, carry one child blocker edge:

```text
target_shape_blocker_symbol=<canonical child target symbol>
target_shape_blocker_reason=<child target_shape_reason or ->
```

The fields are emitted in both `global_call_routes` and `lowering_plan`, and
ny-llvmc only reports them on the existing dev-only unsupported-shape line.

## Rules

Allowed:

- carry child blocker evidence from MIR target classification
- report blocker evidence in ny-llvmc diagnostics
- keep `target_shape`, `tier`, and direct-call legality unchanged

Forbidden:

- using blocker evidence as permission to emit a call
- making child unknown targets lowerable
- backend-local body reclassification
- by-name handling for `Main._run_emit_*`

## Expected Evidence

The full `stage1_cli_env.hako` first stop remains fail-fast, but now carries:

```text
reason=missing_multi_function_emitter
target_shape_reason=generic_string_global_target_shape_unknown
target_shape_blocker_symbol=Stage1InputContractBox.resolve_emit_program_source_text/0
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

That means the next BoxCount should choose a return-ABI-compatible callable
shape or a narrower wrapper contract, not a raw `Main._run_emit_program_mode/0`
emitter.

## Acceptance

- `cargo fmt --check` succeeds.
- `cargo test -q global_call_routes` succeeds.
- `bash tools/build_hako_llvmc_ffi.sh` succeeds.
- `cargo build --release --bin hakorune` succeeds.
- `git diff --check` succeeds.
- `tools/checks/current_state_pointer_guard.sh` succeeds.
- `NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc ...` still fails fast and
  reports `target_shape_blocker_symbol` / `target_shape_blocker_reason`.
