# 296x-848 MIMALLOC-LEAF-ARRAY-STRING-LEN-NEXT-OWNER-SELECTION-001

Status: Landed
Date: 2026-06-16

## Purpose

Select the next owner after the `array_string_len_by_index` ready-path attempt
was rejected as a nonkeeper.

The previous attempt proved that simply replacing the TLS handle-cache route
with the registry ready route is not enough. The next owner is the loop-local
array text session boundary, not another helper-name or source-level patch.

## Current Shape

`ny_main` already emits the right narrow helper body. The visible symbol name
may appear as either alias:

```text
ny_main_hot_loop_call_alias_pair=nyash.array.string_len_hi,hako.array_text.slot_len
ny_main_hot_loop_aliases_share_body=1
backend_emits_array_string_len_helper=1
array_string_len_window_routes_exist=1
```

The helper body is still boundary-heavy:

```text
hako.array_text.slot_len
nyash.array.string_len_hi / hako.array_text.slot_len
  -> array_string_len_by_index
  -> with_array_text_session_cached
  -> with_array_box_at_epoch
  -> HANDLE_CACHE.with(...)
  -> ArrayBox::slot_text_len_raw
  -> items.read()
  -> ArrayTextCell::len()
```

The ready-path experiment:

```text
attempt=with_array_text_session_ready
result=nonkeeper
reason=cycles_and_wall_time_regressed
```

## Selection

The selected next owner is a design-only row for loop-local text-session reuse:

```text
selected_owner=array_text_slot_len_loop_local_session_boundary
selected_owner_confidence=medium
implementation_allowed=0
selected_next=MIMALLOC-LEAF-ARRAY-STRING-LEN-ROUTE-SYMBOL-ATTRIBUTION-PROBE-001
```

The next row must first pin the alias attribution so future rows do not infer
ownership from whichever symbol name `objdump` or `perf` prints.

## Result

```text
output_contract=hako-mimalloc-leaf-array-string-len-next-owner-selection-v0
source_evidence=296x-847,code-inspection-2026-06-16
row_kind=owner_selection
target_front=kilo_leaf_array_string_len

previous_attempt=array_string_len_readonly_ready_path
previous_attempt_kept=0
previous_attempt_reverted=1
previous_nonkeeper_reason=cycles_and_wall_time_regressed

ny_main_hot_loop_call_alias_pair=nyash.array.string_len_hi,hako.array_text.slot_len
ny_main_hot_loop_aliases_share_body=1
backend_array_string_len_direct_helper_enabled=1
array_string_len_window_routes_exist=1

helper_body_owner=crates/nyash_kernel/src/plugin/array_string_slot_indexof.rs
handle_cache_owner=crates/nyash_kernel/src/plugin/array_handle_cache.rs
array_text_storage_owner=src/boxes/array/ops/text.rs

selected_owner=array_text_slot_len_loop_local_session_boundary
selected_owner_confidence=medium
implementation_allowed=0

helper_name_inference_enabled=0
source_hako_changed=0
mirbuilder_changed=0
backend_lowering_changed=0
product_default_changed=0

selected_next=MIMALLOC-LEAF-ARRAY-STRING-LEN-ROUTE-SYMBOL-ATTRIBUTION-PROBE-001
summary=ok
```

## Stop Line

```text
do not reapply the ready-path change
do not infer from hako.array_text.slot_len or nyash.array.string_len_hi by name alone
do not treat alias symbol spelling as owner evidence
do not patch ArrayBox storage
do not change indexOf/store/write paths
do not add a backend loop session without a guard surface
do not touch MIRBuilder object management
do not claim keeper without measuring this exact front
```
