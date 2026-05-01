---
Status: Active
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P137, string-or-void sentinel return child blocker evidence
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P136-GLOBAL-CALL-STRING-VOID-SENTINEL-BODY.md
  - src/mir/global_call_route_plan.rs
  - src/runner/mir_json_emit/tests/global_call_routes.rs
---

# P137: Global Call Void Sentinel Return Blocker

## Problem

After P136, `--emit-exe` advances past
`Stage1InputContractBox.resolve_emit_program_source_text/0` and stops at:

```text
target_shape_blocker_symbol=Stage1SourceProgramAuthorityBox.emit_program_from_source/2
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

That parent function returns either a void/null sentinel or the result of
another same-module global call. The broad ABI reason hides the next ownership
edge and makes the next acceptance slice harder to choose.

## Decision

Do not add a lowerable shape. Instead, propagate the returned unknown
same-module global call as blocker evidence for void/string-or-null candidates.

The parent remains unsupported, but diagnostics should point at the child call
that produced the non-sentinel return value.

## Rules

Allowed:

- report returned unknown same-module global calls through
  `target_shape_blocker_symbol`
- keep `target_shape=unknown` and `tier=Unsupported`
- mirror the same blocker evidence into MIR JSON and LoweringPlan

Forbidden:

- treating this evidence as backend permission
- accepting the parent body as a new target shape
- externalizing the same-module call
- changing `.hako` source to bypass the blocker

## Expected Evidence

For the active `stage1_cli_env.hako` source-execution route, the unsupported
shape trace should move from the broad parent ABI reason to the returned child
global blocker under `Stage1SourceProgramAuthorityBox.emit_program_from_source/2`.

## Acceptance

- `cargo fmt --check` succeeds.
- `cargo test -q global_call_routes` succeeds.
- `cargo test -q build_mir_json_root_emits_void_sentinel_return_child_blocker`
  succeeds.
- `target/release/hakorune --emit-mir-json ... stage1_cli_env.hako` reports the
  returned child blocker.
- `NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe ... stage1_cli_env.hako`
  reports the returned child blocker in `[llvm-pure/unsupported-shape]`.
