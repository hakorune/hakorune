# 296x-851 MIMALLOC-ARRAY-TEXT-LOOP-SESSION-PLAN-SURFACE-001

Status: Landed
Date: 2026-06-16

## Purpose

Add the passive `ArrayTextLoopSessionPlan` proof surface required by 296x-850.

This row adds vocabulary only. It does not discover loop sessions, refresh
metadata, export MIR JSON, or lower backend code.

## Code Surface

```text
plan_file=src/mir/array_text_loop_session_plan.rs
root_facade=src/mir/mod.rs
plan_type=ArrayTextLoopSessionPlan
reject_type=ArrayTextLoopSessionRejectReason
```

Required proof fields:

```text
same_array_handle
read_only_region
no_mutation_region
no_drop_or_publication_boundary
index_domain_guarded
```

Backend session lowering is allowed only when all proof fields hold and the
plan has at least one len call.

## Result

```text
output_contract=hako-mimalloc-array-text-loop-session-plan-surface-v0
source_evidence=296x-850
row_kind=passive_surface
target_front=kilo_leaf_array_string_len

plan_file=src/mir/array_text_loop_session_plan.rs
plan_type=ArrayTextLoopSessionPlan
reject_type=ArrayTextLoopSessionRejectReason

same_array_handle_required=1
read_only_region_required=1
no_mutation_region_required=1
no_drop_or_publication_boundary_required=1
index_domain_guard_required=1

metadata_refresh_enabled=0
mir_json_export_enabled=0
backend_consumer_enabled=0
backend_loop_session_lowering_enabled=0
raw_array_text_session_ffi_enabled=0
raw_arraybox_pointer_ffi_enabled=0
product_default_changed=0

selected_next=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-PLAN-INVENTORY-001
summary=ok
```

## Proof Bundle

```bash
cargo fmt --check
cargo test --lib array_text_loop_session_plan -- --nocapture
bash tools/checks/k2_wide_phase296x_array_text_loop_session_plan_surface_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do not add refresh or backend consumer in this row
do not export ArrayTextLoopSessionPlan to MIR JSON yet
do not pass raw ArrayTextSession or ArrayBox pointers through FFI
do not change ArrayBox storage or product runtime defaults
do not broaden to indexOf/store paths
```
