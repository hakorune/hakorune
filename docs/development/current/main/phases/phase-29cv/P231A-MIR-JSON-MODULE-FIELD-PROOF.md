---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P231a, MirJsonEmitBox module root field proof
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P230A-MIR-JSON-MODULE-FUNCTION-ARRAY-PROOF.md
  - lang/src/shared/mir/json_emit_box.hako
  - src/mir/generic_method_route_plan.rs
  - src/mir/global_call_route_plan/generic_string_body.rs
---

# P231a: MIR JSON Module Field Proof

## Problem

P230a advances the source-exe probe to:

```text
target_shape_blocker_symbol=MirJsonEmitBox.to_json/1
target_shape_blocker_reason=generic_string_unsupported_method_call
```

The unsupported sites are module-root field reads:

```hako
local funcs = module.get("functions")
local f0 = module.get("functions_0")
```

These are exact MIR JSON module schema projections.

## Decision

Add `mir_json_module_field` for `MirJsonEmitBox.to_json/1`:

```text
functions   -> Array
functions_0 -> Map
```

The route remains the existing `generic_method.get` / `runtime_data_load_any`
route with `nyash.runtime_data.get_hh`.

## Non-Goals

- no generic module schema inference
- no general RuntimeDataBox.get acceptance
- no new helper
- no new `GlobalCallTargetShape`

## Acceptance

```bash
cargo test -q mir_json_module_field --lib
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p231a_module_field.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Observed next blocker:

```text
target_shape_blocker_symbol=LowerIfCompareFoldVarIntBox._resolve_side/3
target_shape_blocker_reason=generic_string_return_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```
