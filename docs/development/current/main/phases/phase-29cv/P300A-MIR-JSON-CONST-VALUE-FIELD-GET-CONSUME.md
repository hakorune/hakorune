---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P300a, MIR JSON const value field get fact consumer
Related:
  - docs/development/current/main/phases/phase-29cv/P299A-MIR-JSON-BLOCK-INST-ARRAY-GET-CONSUME.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_method_views.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P300a: MIR JSON Const Value Field Get Consume

## Problem

After P299a, the source-execution probe advances past
`MirJsonEmitBox._emit_block_rec/3` and stops at:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_box_value/1
target_shape_blocker_reason=-
```

`_emit_box_value/1` unwraps MIR const-value wrapper maps:

```hako
local ty_box = val.get("type")
local inner_box = val.get("value")
```

The MIR already carries explicit `generic_method_routes` and `lowering_plan`
rows for these reads:

```text
route_proof=mir_json_const_value_field
source_route_id=generic_method.get
core_op=MapGet
route_kind=runtime_data_load_any
symbol=nyash.runtime_data.get_hh
publication_policy=no_publication
key_const_text=type|value
receiver_origin_box=null
```

The module generic string prepass only accepts the generic MapBox
`get_surface_policy` route when `receiver_origin_box=MapBox`, so it rejects
this owner-owned MIR JSON const-value field fact.

## Decision

Add a narrow C shim consumer for exact MIR JSON const-value field `get` facts.

The accepted contract is:

```text
source_route_id=generic_method.get
core_op=MapGet
route_kind=runtime_data_load_any
symbol=nyash.runtime_data.get_hh
route_proof=mir_json_const_value_field
receiver_origin_box=null
return_shape=mixed_runtime_i64_or_handle
value_demand=runtime_i64_or_handle
publication_policy=no_publication
key_const_text in {type, value}
tier=ColdRuntime
```

This consumes an existing MIR-owned schema field-read fact. It does not make
`MapBox.get` generally acceptable in generic string/i64 bodies.

## Non-Goals

- no generic `MapBox.get` widening
- no new `GlobalCallTargetShape`
- no new body-specific C emitter
- no source `.hako` workaround
- no fallback or externalization

## Acceptance

- `_emit_box_value/1` no longer fails module generic prepass on
  `mir_json_const_value_field` rows.
- The probe advances to the next blocker or emits successfully.
- `bash tools/build_hako_llvmc_ffi.sh`
- `cargo build -q --release --bin hakorune`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

Accepted.

The C shim now consumes exact `mir_json_const_value_field` `MapGet` facts for
MIR const-value wrapper `type` and `value` reads. The existing generic MapBox
`get_surface_policy` predicate remains unchanged, so generic `MapBox.get`
acceptance is not widened.

The source-execution probe advances past `MirJsonEmitBox._emit_box_value/1`
and stops at the next module generic prepass blocker:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_callee/1
target_shape_blocker_reason=-
```
