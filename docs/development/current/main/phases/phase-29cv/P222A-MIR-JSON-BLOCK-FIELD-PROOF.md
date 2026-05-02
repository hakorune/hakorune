---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P222a, MirJsonEmitBox block field proof
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P221A-MIR-JSON-BLOCK-INST-ARRAY-PROOF.md
  - lang/src/shared/mir/json_emit_box.hako
  - src/mir/generic_method_route_plan.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P222a: MIR JSON Block Field Proof

## Problem

P221a advances the source-exe probe to:

```text
target_shape_blocker_symbol=MirJsonEmitBox._emit_block/1
target_shape_blocker_reason=generic_string_unsupported_method_call
backend_reason=missing_multi_function_emitter
```

`_emit_block/1` reads canonical MIR block fields:

```hako
local insts = block.get("instructions")
block.get("id")
```

This is MIR block schema projection, not general map-get support.

## Decision

Add an exact MIR-owned generic method proof:

```text
proof = mir_json_block_field
function = MirJsonEmitBox._emit_block/1
method = RuntimeDataBox.get/1
route_kind = runtime_data_load_any
return_shape = mixed_runtime_i64_or_handle
```

`generic_string_body` consumes only that proof by site:

```text
instructions -> Array
id -> scalar-or-void
```

Also normalize `_emit_block/1` direct null guards to `_is_present_compat` so
the source owner, not Stage0, owns object-or-null guard shape.

## Non-Goals

- no generic map-get acceptance in string bodies
- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no MIR block schema change

## Acceptance

Probe result should move past `_emit_block/1`; a later blocker may remain:

```bash
cargo test -q proves_mir_json_block_field_runtime_data_get --lib
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p222a_block_field.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
