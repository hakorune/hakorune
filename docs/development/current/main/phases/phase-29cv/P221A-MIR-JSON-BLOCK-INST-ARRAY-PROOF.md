---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P221a, MirJsonEmitBox block instruction array item proof
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P220A-FOLD-BIN-INTS-RESOLVE-SIDE-TEXT-ABI.md
  - lang/src/shared/mir/json_emit_box.hako
  - src/mir/generic_method_route_plan.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P221a: MIR JSON Block Instruction Array Proof

## Problem

P220a advances the source-exe probe to:

```text
target_shape_blocker_symbol=MirJsonEmitBox._emit_block_rec/3
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

`_emit_block_rec/3` reads canonical MIR instructions from a block instruction
array:

```hako
local head = me._emit_inst(insts.get(idx))
```

This is a MIR JSON schema array projection, not general array-get support.

## Decision

Add an exact MIR-owned generic method proof:

```text
proof = mir_json_block_inst_array_item
function = MirJsonEmitBox._emit_block_rec/3
method = RuntimeDataBox.get/1
route_kind = array_slot_load_any
return_shape = mixed_runtime_i64_or_handle
```

`generic_string_body` consumes only that proof by site and classifies the result
as scalar-or-void, matching `_emit_inst/1`'s existing present guard.

## Non-Goals

- no generic array-get acceptance in string bodies
- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no MIR block schema change

## Acceptance

Probe result should move past `_emit_block_rec/3`; a later blocker may remain:

```bash
cargo test -q proves_mir_json_block_inst_array_item_runtime_data_get --lib
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p221a_block_inst.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
