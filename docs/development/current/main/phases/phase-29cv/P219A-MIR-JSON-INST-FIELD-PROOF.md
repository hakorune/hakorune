---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P219a, MirJsonEmitBox instruction schema field proof
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P218A-MIR-JSON-EFFECTS-ARRAY-ITEM-PROOF.md
  - lang/src/shared/mir/json_emit_box.hako
  - src/mir/generic_method_route_plan.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P219a: MIR JSON Instruction Field Proof

## Problem

P218a advances the source-exe probe to:

```text
target_shape_blocker_symbol=MirJsonEmitBox._emit_inst/1
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

`_emit_inst/1` reads static fields from canonical MIR instruction objects and
their nested `mir_call` payload:

```hako
local op = inst.get("op")
local payload = inst.get("mir_call")
local callee_json = me._emit_callee(payload.get("callee"))
```

This is MIR instruction schema projection. It is not permission for arbitrary
`RuntimeDataBox.get/1` inside generic string bodies.

## Decision

Add one exact MIR-owned generic method proof:

```text
proof = mir_json_inst_field
function = MirJsonEmitBox._emit_inst/1
method = RuntimeDataBox.get/1
route_kind = runtime_data_load_any
return_shape = mixed_runtime_i64_or_handle
```

The proof accepts only the static MIR instruction schema keys used by
`_emit_inst/1`. `generic_string_body` consumes this proof by site and maps key
families to narrow value classes:

```text
op / operation / op_kind / cmp -> string-or-void
schema ids and nested payload fields -> scalar-or-void
```

## Non-Goals

- no generic `RuntimeDataBox.get/1` acceptance in string bodies
- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no MIR instruction schema change

## Acceptance

Probe result should clear the `_emit_inst/1` method-call blocker and may expose
the next `_emit_inst/1` void-sentinel cleanup blocker:

```bash
cargo test -q proves_mir_json_inst_field_runtime_data_get --lib
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p219a_inst_field.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
