---
Status: Active
Decision: accepted
Date: 2026-05-01
Scope: phase-29cv P128, MIR global-call void sentinel return reason split
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P127-GLOBAL-CALL-TARGET-RETURN-TYPE-C-VIEW.md
  - src/mir/global_call_route_plan.rs
---

# P128: Global Call Void Sentinel Return Reason

## Problem

The current first blocker is a `void`-signature same-module helper that returns
source text on the success path and `void` as an error sentinel:

```text
Stage1InputContractBox.resolve_emit_program_source_text/0
target_return_type=void
target_shape_reason=generic_string_return_abi_not_handle_compatible
```

That broad reason is correct as a stop, but it hides the narrower shape needed
next. Treating `void` as a plain generic string return would be unsafe because
`generic_pure_string_body` advertises `return_shape=string_handle`.

## Decision

Split the return-ABI blocker when MIR can prove that all returns are either
string values or a `void` sentinel:

```text
generic_string_return_void_sentinel_candidate
```

This is classifier evidence only. The target remains `tier=Unsupported` and
`target_shape=null`.

## Rules

Allowed:

- inspect canonical MIR returns to distinguish string-or-void sentinel
  candidates
- keep this as a reject reason until a dedicated shape/proof exists
- use same-module target shapes only as value-flow evidence

Forbidden:

- adding `MirType::Void` to the existing string-handle ABI
- claiming `generic_pure_string_body` for string-or-void functions
- backend-local sentinel detection
- emitting a call based on the new reason

## Expected Evidence

After this card, the `stage1_cli_env.hako` first stop still fails fast, but the
child blocker is no longer the broad return-ABI reason:

```text
target_shape_blocker_symbol=Stage1InputContractBox.resolve_emit_program_source_text/0
target_shape_blocker_reason=generic_string_return_void_sentinel_candidate
```

The next BoxCount can now add a dedicated string-or-void sentinel shape/proof,
or decide to split debug/IO blockers first, without weakening the existing
string-handle contract.

## Acceptance

- `cargo fmt --check` succeeds.
- `cargo test -q global_call_routes` succeeds.
- `cargo build --release --bin hakorune` succeeds.
- `git diff --check` succeeds.
- `tools/checks/current_state_pointer_guard.sh` succeeds.
- `target/release/hakorune --emit-mir-json ... stage1_cli_env.hako` emits
  `generic_string_return_void_sentinel_candidate` for the blocker target.
