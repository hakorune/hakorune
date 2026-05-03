---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P230a, MirJsonEmitBox module function array item proof
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P229A-GENERIC-STRING-COLLECTION-OR-VOID-PHI.md
  - lang/src/shared/mir/json_emit_box.hako
  - src/mir/generic_method_route_plan.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P230a: MIR JSON Module Function Array Proof

## Problem

P229a advances the source-exe probe to:

```text
target_shape_blocker_symbol=MirJsonEmitBox._emit_module_rec/3
target_shape_blocker_reason=generic_string_unsupported_method_call
```

The unsupported sites are function array item reads:

```hako
local first = funcs.get(0)
local head = me._emit_function(funcs.get(idx))
```

These are exact MIR module schema projections from `functions[]`.

## Decision

Add an exact `mir_json_module_function_array_item` proof for
`MirJsonEmitBox._emit_module_rec/3`:

```text
route_id = generic_method.get
route_kind = array_slot_load_any
helper = nyash.array.slot_load_hi
value_class = Map
```

This keeps Stage0 small:

- no generic array object inference
- no new helper
- no new `GlobalCallTargetShape`
- no body-specific C emitter

## Acceptance

```bash
cargo test -q mir_json_module_function_array_item --lib
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p230a_module_function_array.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Observed next blocker:

```text
target_shape_blocker_symbol=MirJsonEmitBox.to_json/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```
