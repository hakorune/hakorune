# 296x-968 MIMALLOC-ARRAY-TEXT-INDEXOF-CONST-BACKEND-READER-SURFACE-001

Status: Landed
Date: 2026-06-16

## Purpose

Add the C ABI metadata reader surface for
`array_text_indexof_const_region_plans`.

This row does not add backend lowering, runtime helpers, or product storage
changes.

## Implementation

Reader surface:

```text
lang/c-abi/shims/hako_llvmc_ffi_array_text_loop_session_metadata.inc
  struct ArrayTextIndexOfConstRegionPlanMetadata
  array_text_indexof_const_region_plan_valid()
  array_text_indexof_const_region_payload_fill()
  array_text_indexof_const_region_plan_fill()
  match_array_text_indexof_const_region_plan_by_header_metadata()
  array_text_indexof_const_region_has_payload_plan()
```

The existing loop-session metadata reader file remains the array-text region
metadata owner. This avoids creating a second include surface before there is a
backend lowering consumer.

## Reader Contract

The reader accepts only the MIR JSON surface from 296x-967:

```text
route_id=array_text.indexof_const_region.plan
consumer_shape=found_predicate
needle_const_text is string
needle_byte_len matches the UTF-8 byte length
selected_helper_symbol namespace=hako.array_text.*
mir_json_export_only=1
backend_consumer_enabled field present
region_payload present with all required scalar fields
```

`selected_helper_symbol` is copied as metadata but is not a legality proof. The
backend still must not infer legality from helper symbol spelling.

## Result

```text
output_contract=hako-mimalloc-array-text-indexof-const-backend-reader-surface-v0

c_abi_reader_surface_enabled=1
reader_owner=lang/c-abi/shims/hako_llvmc_ffi_array_text_loop_session_metadata.inc
json_field=array_text_indexof_const_region_plans

backend_lowering_enabled=0
runtime_helper_added=0
product_default_changed=0
helper_symbol_inference_allowed=0
raw_backend_window_scan_allowed=0

selected_next=MIMALLOC-ARRAY-TEXT-INDEXOF-CONST-REGION-HELPER-CONTRACT-001
summary=ok
```

## Proof Bundle

```bash
cargo check --bin hakorune
cargo test --lib build_mir_json_root_emits_array_text_indexof_const_region_plans -- --nocapture
bash tools/perf/build_perf_release.sh
```

`build_perf_release.sh` compiles `libhako_llvmc_ffi.so`, so the new `.inc`
reader surface is syntax-checked through the C ABI build.

## Stop Line

```text
do not add backend lowering in this row
do not add runtime helper in this row
do not let backend consume observer routes directly
do not infer legality from selected_helper_symbol
do not change product ArrayBox/StringBox storage
```
