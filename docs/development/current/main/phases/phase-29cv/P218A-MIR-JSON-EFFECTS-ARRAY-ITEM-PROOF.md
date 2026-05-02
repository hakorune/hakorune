---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P218a, MirJsonEmitBox effects array item proof
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P217A-MIR-JSON-VID-ARRAY-ITEM-PROOF.md
  - lang/src/shared/mir/json_emit_box.hako
  - src/mir/generic_method_route_plan.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P218a: MIR JSON Effects Array Item Proof

## Problem

P217a advances the source-exe probe to:

```text
target_shape_blocker_symbol=MirJsonEmitBox._emit_effects_rec/3
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

`_emit_effects_rec/3` reads effect tags from a canonical MIR effects array:

```hako
local head = me._quote(effects.get(idx))
```

This is a schema projection for MIR call effects, not general array-get support
inside arbitrary string bodies.

## Decision

Add an exact MIR-owned generic method proof:

```text
proof = mir_json_effects_array_item
function = MirJsonEmitBox._emit_effects_rec/3
method = RuntimeDataBox.get/1
route_kind = array_slot_load_any
return_shape = mixed_runtime_i64_or_handle
```

`generic_string_body` consumes only that proof by site and classifies the result
as string-or-void before `_quote/1`.

## Non-Goals

- no generic array-get acceptance in string bodies
- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no change to MIR call effects JSON schema

## Acceptance

Probe result should move past `_emit_effects_rec/3`; a later blocker may remain:

```bash
cargo test -q proves_mir_json_effects_array_item_runtime_data_get --lib
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p218a_effects_array.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
