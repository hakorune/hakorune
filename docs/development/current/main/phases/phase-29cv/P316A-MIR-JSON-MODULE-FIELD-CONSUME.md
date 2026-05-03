---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P316a, MIR JSON module field fact consume
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P231A-MIR-JSON-MODULE-FIELD-PROOF.md
  - docs/development/current/main/phases/phase-29cv/P315A-MIR-JSON-NUMERIC-VALUE-FIELD-CONSUME.md
  - lang/src/shared/mir/json_emit_box.hako
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_method_views.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P316a: MIR JSON Module Field Consume

## Problem

P315a advances the source-exe probe to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox.to_json/1
target_shape_blocker_reason=-
```

`MirJsonEmitBox.to_json/1` already has exact MIR-owned `generic_method.get`
rows for module fields:

```text
proof = mir_json_module_field
key_const_text = functions
  -> Array handle

proof = mir_json_module_field
key_const_text = functions_0
  -> Map handle
```

The C module generic string emitter does not yet consume these exact MapGet
rows.

## Decision

Consume only exact `mir_json_module_field` MapGet rows in the module generic
string emitter and publish the result origin according to the static key:

```text
functions   -> ORG_ARRAY_BIRTH
functions_0 -> ORG_MAP_BIRTH
```

## Non-Goals

- no generic `RuntimeDataBox.get` / `MapBox.get` widening
- no `MirJsonEmitBox.to_json/1` body-specific emitter
- no new `GlobalCallTargetShape`
- no MIR module schema change

## Acceptance

```bash
cargo test -q proves_mir_json_module_field_runtime_data_get --lib
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p316a_module_field.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected probe result:

```text
MirJsonEmitBox.to_json/1 no longer fails module_generic_prepass_failed on
module.get("functions") or module.get("functions_0").
```

## Result

Accepted. The C module generic string emitter now consumes exact
`mir_json_module_field` MapGet rows through the planned
`nyash.runtime_data.get_hh` route and assigns static result origins for
`functions` / `functions_0`.

Validation:

```bash
cargo test -q proves_mir_json_module_field_runtime_data_get --lib
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p316.exe lang/src/runner/stage1_cli_env.hako
```

The probe advanced past `MirJsonEmitBox.to_json/1` to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirSchemaBox.block/2
target_shape_blocker_reason=-
```
