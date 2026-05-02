---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P224a, MirJsonEmitBox params array proof
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P223A-MIR-JSON-FUNCTION-BLOCK-ARRAY-PROOF.md
  - lang/src/shared/mir/json_emit_box.hako
  - src/mir/generic_method_route_plan.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P224a: MIR JSON Params Array Proof

## Problem

P223a advances the source-exe probe to:

```text
target_shape_blocker_symbol=MirJsonEmitBox._emit_params_rec/3
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

`_emit_params_rec/3` reads a canonical MIR function `params` array item:

```hako
local head = me._int_str(params.get(idx))
```

This is MIR function-schema params projection, not general
`RuntimeDataBox.get` support.

## Decision

Add an exact MIR-owned generic method proof:

```text
proof = mir_json_params_array_item
function = MirJsonEmitBox._emit_params_rec/3
method = RuntimeDataBox.get/1
route_kind = array_slot_load_any
return_shape = mixed_runtime_i64_or_handle
```

`generic_string_body` consumes only that proof by site as:

```text
params array item -> i64
```

The value is then formatted by `_int_str/1`, keeping Stage0 on explicit MIR
schema facts rather than widening runtime data array semantics.

## Non-Goals

- no generic array-get widening for `RuntimeDataBox`
- no generic map-get acceptance in string bodies
- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no MIR function schema change

## Acceptance

Probe result should move past `_emit_params_rec/3`; a later blocker may
remain:

```bash
cargo test -q proves_mir_json_params_array_item_runtime_data_get --lib
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p224a_params_array.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
