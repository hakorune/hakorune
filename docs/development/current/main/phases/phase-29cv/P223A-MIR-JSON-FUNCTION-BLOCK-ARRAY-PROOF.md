---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P223a, MirJsonEmitBox function block array proof
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P222A-MIR-JSON-BLOCK-FIELD-PROOF.md
  - lang/src/shared/mir/json_emit_box.hako
  - src/mir/generic_method_route_plan.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P223a: MIR JSON Function Block Array Proof

## Problem

P222a advances the source-exe probe to:

```text
target_shape_blocker_symbol=MirJsonEmitBox._emit_function_rec/3
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

`_emit_function_rec/3` reads a canonical MIR function `blocks` array item:

```hako
local head = me._emit_block(blocks.get(idx))
```

This is MIR function-schema array projection, not general `RuntimeDataBox.get`
support.

## Decision

Add an exact MIR-owned generic method proof:

```text
proof = mir_json_function_block_array_item
function = MirJsonEmitBox._emit_function_rec/3
method = RuntimeDataBox.get/1
route_kind = array_slot_load_any
return_shape = mixed_runtime_i64_or_handle
```

`generic_string_body` consumes only that proof by site as:

```text
block array item -> scalar-or-void
```

The element is then passed to `_emit_block/1`, which owns the block-level
presence guard and block field facts.

## Non-Goals

- no generic array-get widening for `RuntimeDataBox`
- no generic map-get acceptance in string bodies
- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no MIR function schema change

## Acceptance

Probe result should move past `_emit_function_rec/3`; a later blocker may
remain:

```bash
cargo test -q proves_mir_json_function_block_array_item_runtime_data_get --lib
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p223a_function_block_array.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
