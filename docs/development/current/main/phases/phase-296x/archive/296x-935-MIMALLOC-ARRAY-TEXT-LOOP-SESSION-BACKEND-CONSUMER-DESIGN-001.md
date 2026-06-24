# 296x-935 MIMALLOC-ARRAY-TEXT-LOOP-SESSION-BACKEND-CONSUMER-DESIGN-001

Status: Landed
Date: 2026-06-16

## Purpose

Fix the backend consumer boundary for `array_text_loop_session_plans` before
adding C ABI lowering.

The plan is exported to MIR JSON, but export alone must not trigger lowering.
This row selects the future reader/consumer seam and records risky seams to
avoid.

## Decision

```text
reader_owner=lang/c-abi/shims/hako_llvmc_ffi_array_text_loop_session_metadata.inc
emit_owner=lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering.inc
declaration_owner=lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc

backend_consumer_enabled=0
backend_loop_session_lowering_enabled=0
product_default_changed=0
```

Rationale:

```text
array_text_loop_session_plans is region/session metadata.
It has loop_header / loop_exit, so the consumer is region-level lowering,
not per-GET fallback policy.
```

## Reader Contract

The future reader may validate only MIR-owned fields:

```text
route_id == "array_text.loop_session.plan"
loop_header
loop_exit
array_value
index_value
len_call_count
same_array_handle
read_only_region
no_mutation_region
no_drop_or_publication_boundary
index_domain_guarded
backend_session_lowering_allowed
first_reject_reason
mir_json_export_only
backend_consumer_enabled
```

The future implementation row must not re-scan raw MIR JSON windows or infer
legality from helper symbols.

## Risky Seams

Do not place loop-session consumer logic in:

```text
hako_llvmc_ffi_generic_method_get_policy.inc
hako_llvmc_ffi_generic_method_lowering.inc
hako_llvmc_ffi_pure_compile.inc
hako_llvmc_ffi_indexof_text_state_residence.inc
```

These are either fallback policy, broad generic dispatch, include
orchestration, or unrelated exact-seed surfaces.

## Result

```text
output_contract=hako-mimalloc-array-text-loop-session-backend-consumer-design-v0
array_text_loop_session_plan_json_export_enabled=1
array_text_loop_session_backend_reader_owner_selected=1
array_text_loop_session_backend_consumer_enabled=0
backend_loop_session_lowering_enabled=0
raw_mir_window_rescan_allowed=0
helper_name_inference_allowed=0

selected_next=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-BACKEND-READER-SURFACE-001
summary=ok
```

## Proof Bundle

```bash
bash tools/checks/mir_metadata_catalog_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do not implement backend reader code in this row
do not emit loop-session runtime calls in this row
do not add raw ArrayTextSession / ArrayBox FFI routes
do not mix this with generic_method_get_policy.inc cleanup
```
