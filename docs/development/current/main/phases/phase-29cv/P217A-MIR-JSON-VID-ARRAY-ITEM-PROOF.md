---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P217a, MirJsonEmitBox value-id array item proof
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P216A-MIR-JSON-ARRAY-ASSERT-SPLIT.md
  - lang/src/shared/mir/json_emit_box.hako
  - src/mir/generic_method_route_plan.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P217a: MIR JSON Value-Id Array Item Proof

## Problem

P216a advances the source-exe probe to:

```text
target_shape_blocker_symbol=MirJsonEmitBox._emit_vid_array_rec/3
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

`_emit_vid_array_rec/3` reads MIR call argument value IDs from a canonical array:

```hako
local head = me._emit_vid_or_null(arr.get(idx))
```

This is not generic array semantics for arbitrary string bodies. It is a MIR JSON
schema projection for value-id arrays.

## Decision

Add an exact MIR-owned generic method proof:

```text
proof = mir_json_vid_array_item
function = MirJsonEmitBox._emit_vid_array_rec/3
method = RuntimeDataBox.get/1
route_kind = array_slot_load_any
return_shape = scalar_i64_or_missing_zero
```

`generic_string_body` consumes only that proof by site and classifies the result
as scalar i64.

## Non-Goals

- no generic array-get acceptance in string bodies
- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no change to MIR call args JSON schema

## Acceptance

Probe result should move past `_emit_vid_array_rec/3`; a later blocker may
remain:

```bash
cargo test -q proves_mir_json_vid_array_item_runtime_data_get --lib
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p217a_vid_array.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
