---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P215a, MirJsonEmitBox callee field proof
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P214A-MIR-JSON-PHI-INCOMING-ARRAY-PROOF.md
  - lang/src/shared/mir/json_emit_box.hako
  - src/mir/generic_method_route_plan.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P215a: MIR JSON Callee Field Proof

## Problem

P214a advances the source-exe probe to:

```text
target_shape_blocker_symbol=MirJsonEmitBox._emit_callee/1
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

`_emit_callee/1` reads fixed MIR callee schema fields:

```text
type, name, box_name, method, receiver, box_type
```

These are not generic map semantics. They are schema projections used only to
emit MIR JSON call payloads.

## Decision

Move direct nullable checks in `_emit_callee/1` behind `_is_present_compat/1`
and add exact MIR-owned generic method proof:

```text
proof = mir_json_callee_field
function = MirJsonEmitBox._emit_callee/1
method = RuntimeDataBox.get/1
key_const_text in {"type", "name", "box_name", "method", "receiver", "box_type"}
```

`generic_string_body` consumes only that proof by site. Text fields are
classified as `StringOrVoid`; `receiver` is classified as scalar-or-void because
it flows into `_emit_vid_or_null/1`.

## Non-Goals

- no generic map-get acceptance
- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no change to MIR callee JSON schema

## Acceptance

Probe result should move past `_emit_callee/1`; a later blocker may remain:

```bash
cargo test -q proves_mir_json_callee_field_runtime_data_get --lib
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p215a_callee_field.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
