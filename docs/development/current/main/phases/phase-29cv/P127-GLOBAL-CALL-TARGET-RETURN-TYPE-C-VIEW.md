---
Status: Active
Decision: accepted
Date: 2026-05-01
Scope: phase-29cv P127, ny-llvmc global-call target return-type view parity
Related:
  - docs/development/current/main/design/ai-handoff-and-debug-contract.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P126-GLOBAL-CALL-TARGET-SIGNATURE-EVIDENCE.md
  - lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
---

# P127: Global Call Target Return-Type C View

## Problem

P126 added `target_return_type` to MIR JSON, but ny-llvmc's typed
`LoweringPlanGlobalCallView` did not read it. That leaves the next return-ABI
slice one step away from raw JSON re-reading in C diagnostics or callsite code.

## Decision

Read `target_return_type` into `LoweringPlanGlobalCallView` and include it on
the existing dev-only unsupported-shape inventory line:

```text
[llvm-pure/unsupported-shape] ... target_return_type=<type|-> ...
```

This is consumer parity only. It does not make any global-call target lowerable.

## Rules

Allowed:

- carry `target_return_type` through the typed C view
- surface it on the existing dev-only unsupported-shape trace
- keep `target_shape`, `tier`, proof, and emitter validators unchanged

Forbidden:

- reading `target_return_type` directly from raw JSON at callsites
- using `target_return_type` as permission to emit a call
- widening `generic_pure_string_body` to accept `void`

## Expected Evidence

For the current full `stage1_cli_env.hako` first stop:

```text
reason=missing_multi_function_emitter
target_return_type=i64
target_shape_reason=generic_string_global_target_shape_unknown
target_shape_blocker_symbol=Stage1InputContractBox.resolve_emit_program_source_text/0
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

The blocker target's own row remains inspectable in MIR JSON with:

```text
callee_name=Stage1InputContractBox.resolve_emit_program_source_text/0
target_return_type=void
target_shape_reason=generic_string_return_abi_not_handle_compatible
```

## Acceptance

- `bash tools/build_hako_llvmc_ffi.sh` succeeds.
- `cargo test -q global_call_routes` succeeds.
- `cargo build --release --bin hakorune` succeeds.
- `git diff --check` succeeds.
- `tools/checks/current_state_pointer_guard.sh` succeeds.
- `NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc ...` still fails fast and
  includes `target_return_type` on `[llvm-pure/unsupported-shape]`.
