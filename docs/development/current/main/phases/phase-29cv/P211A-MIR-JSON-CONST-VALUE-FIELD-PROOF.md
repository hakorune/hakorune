---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P211a, exact MirJsonEmitBox const-value field read proof
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P208B-MIR-SCHEMA-FIELD-READ-ROUTE-PROOF.md
  - lang/src/shared/mir/json_emit_box.hako
  - src/mir/generic_method_route_plan.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P211a: MIR JSON Const-Value Field Proof

## Problem

P210a advances the source-exe probe to:

```text
target_shape_blocker_symbol=MirJsonEmitBox._emit_box_value/1
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

The blocking method calls are exact schema field reads in
`MirJsonEmitBox._emit_box_value/1`:

```hako
local ty_box = val.get("type")
local inner_box = val.get("value")
```

They are not generic map semantics. They read MIR const-value schema fields and
are immediately folded into string JSON output.

## Decision

Add a MIR-owned generic-method route proof:

```text
proof = mir_json_const_value_field
function = MirJsonEmitBox._emit_box_value/1
method = RuntimeDataBox.get/1
key_const_text in {"type", "value"}
```

`generic_string_body` consumes only that exact proof by site:

```text
key "type"  -> StringOrVoid
key "value" -> runtime i64/handle value class for string concatenation
```

This keeps the route fact exact and avoids accepting arbitrary
`RuntimeDataBox.get`.

## Non-Goals

- no generic map-get acceptance
- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no change to `MirJsonEmitBox._emit_box_value/1` source semantics

## Acceptance

Probe result should move past `_emit_box_value/1`; a later blocker may remain:

```bash
cargo test -q proves_mir_json_const_value_field_runtime_data_get --lib
cargo test -q refresh_module_global_call_routes_accepts_mir_json_emit_box_value_field_reads --lib
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p211a_const_value.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

