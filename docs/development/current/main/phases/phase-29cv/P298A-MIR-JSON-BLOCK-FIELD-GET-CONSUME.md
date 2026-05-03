---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P298a, MIR JSON block field get fact consumer
Related:
  - docs/development/current/main/phases/phase-29cv/P297A-JSONFRAG-CONST-VALUE-SIG-TEXT-SENTINEL.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_method_views.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P298a: MIR JSON Block Field Get Consume

## Problem

After P297a, the source-execution probe advances past the JsonFrag
normalizer and stops at:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_block/1
target_shape_blocker_reason=-
```

`MirJsonEmitBox._emit_block/1` already has explicit `generic_method_routes`
and `lowering_plan` rows for its block schema field reads:

```text
proof=mir_json_block_field
route_kind=runtime_data_load_any
symbol=nyash.runtime_data.get_hh
publication_policy=no_publication
key_const_text=instructions|id
```

The module generic string prepass only accepts the existing generic MapBox
`get_surface_policy` route when `receiver_origin_box=MapBox`, so it rejects
these owner-owned MIR JSON schema field facts even though the route is already
explicit.

## Decision

Add a narrow C shim consumer for exact MIR JSON block field `get` facts.

The accepted contract is:

```text
source_route_id=generic_method.get
core_op=MapGet
route_kind=runtime_data_load_any
symbol=nyash.runtime_data.get_hh
route_proof=mir_json_block_field
receiver_origin_box=null
return_shape=mixed_runtime_i64_or_handle
value_demand=runtime_i64_or_handle
publication_policy=no_publication
key_const_text in {id, instructions}
tier=ColdRuntime
```

This consumes an existing MIR-owned schema field-read fact. It does not make
`MapBox.get` generally acceptable in generic string/i64 bodies.

## Non-Goals

- no generic `MapBox.get` widening
- no new `GlobalCallTargetShape`
- no new body-specific C emitter
- no by-name helper capsule
- no source `.hako` workaround
- no fallback or externalization

## Acceptance

- `_emit_block/1` no longer fails module generic prepass on
  `mir_json_block_field` rows.
- The probe advances to the next blocker or emits successfully.
- `bash tools/build_hako_llvmc_ffi.sh`
- `cargo build -q --release --bin hakorune`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

Accepted.

The C shim now consumes exact `mir_json_block_field` `MapGet` facts for the
MIR block schema fields `id` and `instructions`. The existing generic MapBox
`get_surface_policy` predicate remains unchanged, so generic `MapBox.get`
acceptance is not widened.

The source-execution probe advances past `MirJsonEmitBox._emit_block/1` and
stops at the next module generic prepass blocker:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_block_rec/3
target_shape_blocker_reason=-
```
