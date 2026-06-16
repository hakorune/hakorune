# 296x-936 MIMALLOC-ARRAY-TEXT-LOOP-SESSION-BACKEND-READER-SURFACE-001

Status: Landed
Date: 2026-06-16

## Purpose

Add the C ABI reader surface for `array_text_loop_session_plans` without
lowering runtime loop-session calls.

This keeps the backend boundary route-first: C may validate MIR-owned proof
fields, but it must not rediscover loop-session legality from raw MIR windows.

## Implementation

```text
lang/c-abi/shims/hako_llvmc_ffi_array_text_loop_session_metadata.inc:
  ArrayTextLoopSessionPlanMetadata
  array_text_loop_session_plan_valid()
  match_array_text_loop_session_plan_by_header_metadata()

lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc:
  include metadata reader only

lang/c-abi/shims/README.md:
  document reader responsibility
```

The reader validates:

```text
route_id == "array_text.loop_session.plan"
same_array_handle=1
read_only_region=1
no_mutation_region=1
no_drop_or_publication_boundary=1
index_domain_guarded=1
backend_session_lowering_allowed=1
first_reject_reason=null
mir_json_export_only=1
backend_consumer_enabled=<bool present>
```

`backend_consumer_enabled` is read but not used to emit code in this row.

## Result

```text
output_contract=hako-mimalloc-array-text-loop-session-backend-reader-surface-v0
array_text_loop_session_backend_reader_surface_enabled=1
array_text_loop_session_backend_consumer_enabled=0
backend_loop_session_lowering_enabled=0
raw_mir_window_rescan_allowed=0
helper_name_inference_allowed=0
product_default_changed=0

selected_next=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-BACKEND-LOWERING-GUARD-SURFACE-001
summary=ok
```

## Proof Bundle

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/mir_metadata_catalog_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do not emit loop-session runtime calls in this row
do not add extern declarations in this row
do not read raw MIR instruction windows for legality
do not put loop-session reader logic in generic_method_get_policy.inc
```
