# 296x-846 MIMALLOC-LEAF-ARRAY-STRING-LEN-ATTRIBUTION-REPEAT-001

Status: Landed
Date: 2026-06-16

## Purpose

Repeat attribution for `kilo_leaf_array_string_len` and select a narrow owner
before implementation.

The repeated run confirms that the hot loop calls `hako.array_text.slot_len`,
and that the visible hot boundary is the TLS handle cache path inside that
helper. This authorizes a narrow read-only ready-path implementation for
`array_string_len_by_index` only.

## Evidence

Repeated microasm:

```text
target_front=kilo_leaf_array_string_len
runs=3
runner=direct
asm_sample_count=12
asm_top_symbol_0=std::thread::local::LocalKey<T>::with
asm_top_symbol_0_percent=99.79
```

`ny_main` hot loop:

```text
ny_main_calls=hako.array_text.slot_len
ny_main_call_count=1
hako_array_text_slot_len_address=0x414930
```

Inside `hako.array_text.slot_len`:

```text
hako_array_text_slot_len_calls=std::thread::local::LocalKey<T>::with
hako_array_text_slot_len_localkey_call_offset=0x68
```

Code owner:

```text
array_text_slot_len_export_owner=crates/nyash_kernel/src/plugin/array_runtime_aliases.rs
array_string_len_body_owner=crates/nyash_kernel/src/plugin/array_string_slot_indexof.rs
handle_cache_owner=crates/nyash_kernel/src/plugin/array_handle_cache.rs
existing_ready_seam=with_array_box_ready
```

## Result

```text
output_contract=hako-mimalloc-leaf-array-string-len-attribution-repeat-v0
source_evidence=296x-845,microasm-repeat-2026-06-16
row_kind=owner_selection
target_front=kilo_leaf_array_string_len

ny_main_calls_hako_array_text_slot_len=1
hako_array_text_slot_len_calls_localkey=1
runtime_tls_boundary_visible=1
selected_owner=array_text_slot_len_handle_cache_tls_boundary
selected_owner_confidence=medium

implementation_allowed=1
implementation_scope=array_string_len_readonly_ready_path
existing_ready_seam=with_array_box_ready
new_backend_route_enabled=0
mirbuilder_changed=0
backend_lowering_changed=0
product_default_changed=0
helper_name_inference_enabled=0

selected_next=MIMALLOC-LEAF-ARRAY-STRING-LEN-READY-PATH-IMPLEMENTATION-001
summary=ok
```

## Stop Line

```text
do not change indexOf/session cache policy
do not change store/write paths
do not change ArrayBox storage
do not change backend route selection
do not touch MIRBuilder
do not infer from benchmark or helper names
```
