---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P315a, MIR JSON numeric value field fact consume
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P208A-MIR-SCHEMA-FIELD-READ-FACT-LOCK.md
  - docs/development/current/main/phases/phase-29cv/P208B-MIR-SCHEMA-FIELD-READ-ROUTE-PROOF.md
  - docs/development/current/main/phases/phase-29cv/P208C-GENERIC-I64-SCHEMA-FIELD-PROOF-CONSUME.md
  - docs/development/current/main/phases/phase-29cv/P314A-MIR-JSON-VID-ARRAY-CONSUME.md
  - lang/src/shared/mir/json_emit_box.hako
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_method_views.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P315a: MIR JSON Numeric Value Field Consume

## Problem

P314a advances the source-exe probe to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._expect_i64/2
target_shape_blocker_reason=-
```

`MirJsonEmitBox._expect_i64/2` already has an exact MIR-owned
`generic_method.get` row for its numeric wrapper unwrap:

```text
proof = mir_json_numeric_value_field
route_kind = runtime_data_load_any
key_const_text = value
return_shape = scalar_i64_or_missing_zero
value_demand = scalar_i64
```

The C module generic string emitter does not yet consume this exact MapGet row.

## Decision

Consume only the exact `mir_json_numeric_value_field` MapGet row in the module
generic string emitter.

## Non-Goals

- no generic `RuntimeDataBox.get` / `MapBox.get` widening
- no `_expect_i64/2` body-specific emitter
- no new `GlobalCallTargetShape`
- no MIR JSON numeric wrapper schema change

## Acceptance

```bash
cargo test -q proves_mir_json_numeric_value_field_runtime_data_get --lib
cargo test -q refresh_module_global_call_routes_accepts_mir_json_numeric_value_field_proof --lib
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p315a_numeric_value.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected probe result:

```text
MirJsonEmitBox._expect_i64/2 no longer fails module_generic_prepass_failed
on val.get("value").
```

## Result

Accepted. The C module generic string emitter now consumes exact
`mir_json_numeric_value_field` MapGet rows through the planned
`nyash.runtime_data.get_hh` route.

Validation:

```bash
cargo test -q proves_mir_json_numeric_value_field_runtime_data_get --lib
cargo test -q refresh_module_global_call_routes_accepts_mir_json_numeric_value_field_proof --lib
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p315.exe lang/src/runner/stage1_cli_env.hako
```

The probe advanced past `MirJsonEmitBox._expect_i64/2` to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox.to_json/1
target_shape_blocker_reason=-
```
