---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P299a, MIR JSON block instruction array item get fact consumer
Related:
  - docs/development/current/main/phases/phase-29cv/P298A-MIR-JSON-BLOCK-FIELD-GET-CONSUME.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_method_views.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P299a: MIR JSON Block Inst Array Get Consume

## Problem

After P298a, the source-execution probe advances past
`MirJsonEmitBox._emit_block/1` and stops at:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_block_rec/3
target_shape_blocker_reason=-
```

`_emit_block_rec/3` reads instruction items from the block instruction array:

```hako
local head = me._emit_inst(insts.get(idx))
```

The MIR already carries explicit `generic_method_routes` and `lowering_plan`
rows for these reads:

```text
route_proof=mir_json_block_inst_array_item
source_route_id=generic_method.get
core_op=ArrayGet
route_kind=array_slot_load_any
symbol=nyash.array.slot_load_hi
publication_policy=no_publication
receiver_origin_box=null
```

The module generic string prepass only accepts the existing generic ArrayBox
`get_surface_policy` route when `receiver_origin_box=ArrayBox`, so it rejects
this owner-owned MIR JSON instruction-array item fact.

## Decision

Add a narrow C shim consumer for exact MIR JSON block instruction-array item
`get` facts.

The accepted contract is:

```text
source_route_id=generic_method.get
core_op=ArrayGet
route_kind=array_slot_load_any
symbol=nyash.array.slot_load_hi
route_proof=mir_json_block_inst_array_item
receiver_origin_box=null
return_shape=mixed_runtime_i64_or_handle
value_demand=runtime_i64_or_handle
publication_policy=no_publication
tier=DirectAbi
```

This consumes an existing MIR-owned schema array-item fact. It does not make
`ArrayBox.get` generally acceptable in generic string/i64 bodies.

## Non-Goals

- no generic `ArrayBox.get` widening
- no new `GlobalCallTargetShape`
- no new body-specific C emitter
- no source `.hako` workaround
- no fallback or externalization

## Acceptance

- `_emit_block_rec/3` no longer fails module generic prepass on
  `mir_json_block_inst_array_item` rows.
- The probe advances to the next blocker or emits successfully.
- `bash tools/build_hako_llvmc_ffi.sh`
- `cargo build -q --release --bin hakorune`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

Accepted.

The C shim now consumes exact `mir_json_block_inst_array_item` `ArrayGet`
facts for MIR block instruction-array item reads. The existing generic
ArrayBox `get_surface_policy` predicate remains unchanged, so generic
`ArrayBox.get` acceptance is not widened.

The source-execution probe advances past `MirJsonEmitBox._emit_block_rec/3`
and stops at the next module generic prepass blocker:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_box_value/1
target_shape_blocker_reason=-
```
