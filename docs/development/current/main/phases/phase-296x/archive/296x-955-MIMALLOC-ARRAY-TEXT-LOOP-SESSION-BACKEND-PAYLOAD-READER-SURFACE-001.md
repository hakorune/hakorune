# 296x-955 MIMALLOC-ARRAY-TEXT-LOOP-SESSION-BACKEND-PAYLOAD-READER-SURFACE-001

Status: Landed
Date: 2026-06-16

## Purpose

Extend the C ABI `array_text_loop_session_plans` metadata reader so the backend
can read the MIR-owned region payload added in 296x-954.

This row is still reader-only. It does not emit loop-session runtime calls,
add helper declarations, enable backend lowering, or infer payload values from
raw MIR windows.

## Implementation

```text
lang/c-abi/shims/hako_llvmc_ffi_array_text_loop_session_metadata.inc
```

`ArrayTextLoopSessionPlanMetadata` now includes:

```text
region_payload_present
region_array_root_value
region_loop_index_phi_value
region_loop_index_initial_value
region_loop_index_initial_const
region_loop_index_next_value
region_loop_bound_value
region_loop_bound_const
region_accumulator_phi_value
region_accumulator_initial_value
region_accumulator_initial_const
region_accumulator_next_value
region_exit_accumulator_value
region_row_index_value
region_row_modulus_value
region_row_modulus_const
```

The reader keeps `array_text_loop_session_plan_valid()` focused on the existing
legality surface. Region payload reading is passive: `region_payload_present`
is set only when all 15 payload fields are present and numeric.

## Result

```text
output_contract=hako-mimalloc-array-text-loop-session-backend-payload-reader-surface-v0
array_text_loop_session_region_payload_reader_enabled=1
region_payload_present_field_enabled=1
selected_region_payload_field_count=15
plan_valid_requires_region_payload=0
backend_consumer_enabled=0
backend_lowering_enabled=0
runtime_helper_enabled=0
raw_mir_window_rescan_allowed=0
product_default_changed=0
winner_claim=0

selected_next=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-HELPER-CONTRACT-DESIGN-001
summary=ok
```

## Stop Line

```text
do not emit loop-session runtime calls in this row
do not add extern helper declarations in this row
do not enable backend_consumer_enabled
do not require C to infer missing payload from raw MIR
do not change product ArrayBox/StringBox behavior
```

## Proof Bundle

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/mir_metadata_catalog_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
