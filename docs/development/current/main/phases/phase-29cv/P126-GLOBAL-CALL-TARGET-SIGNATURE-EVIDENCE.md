---
Status: Active
Decision: accepted
Date: 2026-05-01
Scope: phase-29cv P126, MIR global-call target signature evidence
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P125-GLOBAL-CALL-TARGET-SHAPE-BLOCKER.md
  - src/mir/global_call_route_plan.rs
---

# P126: Global Call Target Signature Evidence

## Problem

P125 identified the first `stage1_cli_env.hako` stop as a child target blocker:

```text
target_shape_blocker_symbol=Stage1InputContractBox.resolve_emit_program_source_text/0
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

The blocker reason says the next slice is return-ABI related, but the JSON did
not expose the target signature fact that caused it. Without that evidence, the
next BoxCount could drift into raw target-body inspection or by-name backend
handling.

## Decision

Carry the MIR target function return type into same-module global-call route
metadata:

```text
target_return_type=<compact MIR return type label>
```

The field is emitted in both `metadata.global_call_routes[]` and
`metadata.lowering_plan[]` for `source=global_call_routes`.

## Rules

Allowed:

- expose target return type from the MIR function signature
- use the field to choose the next acceptance slice
- keep the existing target-shape classifier as the legality owner

Forbidden:

- treating `target_return_type` as permission to emit a call
- making `void` return ABI-compatible in this card
- scanning raw target bodies in ny-llvmc to recover this fact
- backend by-name handling for the Stage1 helpers

## Expected Evidence

The first full `stage1_cli_env.hako` blocker remains fail-fast. The top-level
row still points at the child blocker:

```text
target_shape_blocker_symbol=Stage1InputContractBox.resolve_emit_program_source_text/0
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
```

The target function's own lowering-plan row then exposes the signature fact
without raw body scanning:

```text
callee_name=Stage1InputContractBox.resolve_emit_program_source_text/0
target_return_type=void
target_shape_reason=generic_string_return_abi_not_handle_compatible
```

This points the next BoxCount at a narrow void/null/string-handle return
contract instead of a backend-specific call escape hatch.

## Acceptance

- `cargo fmt --check` succeeds.
- `cargo test -q global_call_routes` succeeds.
- `cargo build --release --bin hakorune` succeeds.
- `git diff --check` succeeds.
- `tools/checks/current_state_pointer_guard.sh` succeeds.
- `target/release/hakorune --emit-mir-json ... stage1_cli_env.hako` emits
  `target_return_type` for global-call route and lowering-plan entries.
