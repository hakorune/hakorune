---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P317a, MIR schema MapBox.set redundant receiver operand normalization
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P196-MIR-SCHEMA-MAP-CONSTRUCTOR-BODY.md
  - docs/development/current/main/phases/phase-29cv/P316A-MIR-JSON-MODULE-FIELD-CONSUME.md
  - src/mir/generic_method_route_plan.rs
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P317a: MIR Schema Map Set Redundant Receiver

## Problem

P316a advances the source-exe probe to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirSchemaBox.block/2
target_shape_blocker_reason=-
```

`MirSchemaBox.block/2` is already classified as a
`mir_schema_map_constructor_body`, but its local `MapBox.set` calls do not have
`generic_method.set` rows.  The MIR surface for these calls carries the
receiver twice:

```text
callee.receiver = <map receiver>
args = [<receiver alias>, <key>, <value>]
```

The generic method planner and C emitter currently consume only the semantic
arity-2 form:

```text
args = [<key>, <value>]
```

## Decision

Normalize only redundant method receiver operands for `set` routes:

```text
[key, value]                  -> semantic set(key, value)
[receiver_alias, key, value]  -> semantic set(key, value)
```

The arity-3 form is accepted only when `receiver_alias` resolves to the same
value origin as `callee.receiver`.  The emitted lowering remains the existing
exact `generic_method.set` / `SetSurfacePolicy` / `MapSet` route.

## Non-Goals

- no new `GlobalCallTargetShape`
- no `MirSchemaBox.block/2` body-specific C emitter
- no generic collection method widening beyond exact `generic_method.set`
- no fallback to VM or backend rediscovery by function name

## Acceptance

```bash
cargo test -q records_mapbox_set_with_redundant_receiver_arg_as_core_method_route --lib
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p317.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected probe result:

```text
MirSchemaBox.block/2 no longer fails module_generic_prepass_failed because its
exact MapBox.set route facts are present and consumed.
```

## Result

Accepted. The generic method route planner now normalizes the redundant
receiver operand for exact `set` calls only when the extra operand resolves to
the same value origin as `callee.receiver`.  The C module generic emitter reads
either semantic arity-2 or redundant-receiver arity-3 `set` operands, but still
requires the exact `generic_method.set` / `SetSurfacePolicy` / `MapSet`
LoweringPlan row.

Validation:

```bash
cargo test -q records_mapbox_set_with_redundant_receiver_arg_as_core_method_route --lib
cargo test -q rejects_mapbox_set_with_non_alias_redundant_receiver_arg --lib
bash tools/build_hako_llvmc_ffi.sh
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p317.exe lang/src/runner/stage1_cli_env.hako
```

The probe advanced past `MirSchemaBox.block/2` to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=Stage1MirPayloadContractBox._mir_text_has_functions/1
target_shape_blocker_reason=-
```
