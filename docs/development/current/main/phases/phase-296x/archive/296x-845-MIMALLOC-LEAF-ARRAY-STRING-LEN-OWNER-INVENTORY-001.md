# 296x-845 MIMALLOC-LEAF-ARRAY-STRING-LEN-OWNER-INVENTORY-001

Status: Landed
Date: 2026-06-16

## Purpose

Inventory the selected `kilo_leaf_array_string_len` front before any
implementation row.

The first attribution run did not expose a clean Array/String helper owner. The
dominant observed symbol is a runtime TLS boundary:

```text
std::thread::local::LocalKey<T>::with
```

That means the next step is not a source, MIRBuilder, ArrayBox, or helper-name
patch. The next step is a repeated attribution row that separates runtime TLS /
measurement boundary from the actual string-length body.

## Attribution

Command shape:

```bash
bash tools/perf/bench_micro_c_vs_aot_stat.sh kilo_leaf_array_string_len 1 3
bash tools/perf/bench_micro_aot_asm.sh \
  kilo_leaf_array_string_len 'nyash_array|runtime_data|get_hh|string|array' 1
```

Observed stat:

```text
c_instr=14526802
c_cycles=3223732
c_ms=4
ny_aot_instr=92925832
ny_aot_cycles=32183346
ny_aot_ms=10
ratio_instr=0.16
ratio_cycles=0.10
ratio_ms=0.40
aot_status=ok
```

Observed top report:

```text
asm_top_symbol_0=std::thread::local::LocalKey<T>::with
asm_top_symbol_0_percent=97.52
asm_sample_count=25
```

The sample count is small, and the selected symbol is a runtime/TLS boundary.
This is enough to block implementation, not enough to select a keeper.

## Result

```text
output_contract=hako-mimalloc-leaf-array-string-len-owner-inventory-v0
source_evidence=296x-844,microasm-2026-06-16
row_kind=owner_inventory
target_front=kilo_leaf_array_string_len

hako_slower_front=1
ratio_instr=0.16
ratio_cycles=0.10
ratio_ms=0.40
aot_status=ok

asm_top_symbol_0=std::thread::local::LocalKey<T>::with
asm_top_symbol_0_percent=97.52
asm_sample_count=25
selected_owner=runtime_tls_boundary_low_confidence
selected_owner_confidence=low

array_string_helper_owner_selected=0
string_length_body_owner_selected=0
compiler_lowering_owner_selected=0
implementation_allowed=0

runtime_tls_boundary_visible=1
measurement_boundary_confidence=low
repeat_attribution_required=1

source_hako_changed=0
mirbuilder_changed=0
arraybox_changed=0
stringbox_changed=0
runtime_helper_changed=0
product_default_changed=0
helper_name_inference_enabled=0

selected_next=MIMALLOC-LEAF-ARRAY-STRING-LEN-ATTRIBUTION-REPEAT-001
summary=ok
```

## Stop Line

```text
do not patch ArrayBox or StringBox from this low-confidence owner
do not optimize std::thread::local::LocalKey<T>::with without a runtime owner row
do not infer string length ownership from benchmark name
do not touch MIRBuilder
do not change product runtime defaults
do not claim a keeper from a 25-sample top report
```
