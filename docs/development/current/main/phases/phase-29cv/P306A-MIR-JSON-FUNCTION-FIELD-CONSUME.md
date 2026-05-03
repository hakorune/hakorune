---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P306a, MIR JSON function field fact consume
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P228A-MIR-JSON-FUNCTION-FIELD-PROOF.md
  - docs/development/current/main/phases/phase-29cv/P305A-MIR-JSON-FLAGS-REC-ACCESS-CONSUME.md
  - lang/src/shared/mir/json_emit_box.hako
  - src/mir/generic_method_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_method_views.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P306a: MIR JSON Function Field Consume

## Problem

P305a advances the source-exe probe to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_function/1
target_shape_blocker_reason=-
```

`_emit_function/1` already has exact MIR-owned function-field routes:

```text
func.get("name")   -> proof=mir_json_function_field
func.get("params") -> proof=mir_json_function_field
func.get("flags")  -> proof=mir_json_function_field
func.get("blocks") -> proof=mir_json_function_field
```

The module generic string emitter does not yet consume those exact rows.
Also, the `blocks` projection must remain an Array origin so the following
`blocks.length()` lowers as ArrayLen, not StringLen.

## Decision

Consume only exact `mir_json_function_field` rows in the module generic string
emitter:

```text
generic_method.get + MapGet + runtime_data_load_any + mir_json_function_field
keys: name | params | flags | blocks
```

The MIR generic-method route fact treats:

```text
params -> ArrayBox origin
blocks -> ArrayBox origin
```

This keeps the route fact owner in MIR metadata and prevents C from deriving
schema semantics from raw method names.

## Non-Goals

- no generic `RuntimeDataBox.get` widening
- no new function-body shape
- no new `GlobalCallTargetShape`
- no ny-llvmc body-specific emitter
- no MIR JSON function schema change

## Acceptance

```bash
cargo test -q mir_json_function_field --lib
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p306a_function_field.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected probe result:

```text
_emit_function/1 no longer fails module_generic_prepass_failed at function-field get sites
```

Observed next blocker:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_function_rec/3
target_shape_blocker_reason=-
```
