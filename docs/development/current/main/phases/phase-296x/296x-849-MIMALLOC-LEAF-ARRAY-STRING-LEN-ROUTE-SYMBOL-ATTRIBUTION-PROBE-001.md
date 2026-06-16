# 296x-849 MIMALLOC-LEAF-ARRAY-STRING-LEN-ROUTE-SYMBOL-ATTRIBUTION-PROBE-001

Status: Landed
Date: 2026-06-16

## Purpose

Pin the alias attribution for `kilo_leaf_array_string_len` before loop-session
design.

`nyash.array.string_len_hi` and `hako.array_text.slot_len` are aliases for the
same exported Rust body. Future rows must not treat the printed symbol spelling
as owner evidence.

## Evidence

`ny_main` calls the shared helper address:

```text
ny_main_hot_loop_call_address=0x414930
```

`nm` reports both aliases at the same address:

```text
nyash.array.string_len_hi_address=0x414930
hako.array_text.slot_len_address=0x414930
route_aliases_share_address=1
```

Both aliases route to:

```text
array_string_len_by_index
```

## Result

```text
output_contract=hako-mimalloc-leaf-array-string-len-route-symbol-attribution-probe-v0
source_evidence=296x-848,nm-objdump-2026-06-16,worker-inventory-2026-06-16
row_kind=attribution_probe
target_front=kilo_leaf_array_string_len

ny_main_hot_loop_call_address=0x414930
nyash_array_string_len_hi_address=0x414930
hako_array_text_slot_len_address=0x414930
route_aliases_share_address=1
route_aliases_share_body=1

alias_symbol_spelling_is_owner_evidence=0
selected_owner=array_text_slot_len_loop_local_session_boundary
selected_owner_confidence=medium
implementation_allowed=0

source_hako_changed=0
mirbuilder_changed=0
backend_lowering_changed=0
product_default_changed=0
helper_name_inference_enabled=0

selected_next=MIMALLOC-LEAF-ARRAY-STRING-LEN-LOOP-SESSION-DESIGN-001
summary=ok
```

## Stop Line

```text
do not choose owner from alias spelling
do not patch alias exports
do not change C shim route selection from this probe
do not patch ArrayBox storage
do not reapply ready-path change
do not broaden to indexOf/store paths
```
