# 296x-930 CABI-GET-POLICY-SPLIT-001

Status: Landed
Date: 2026-06-16

## Purpose

Split the first responsibility seam out of
`hako_llvmc_ffi_generic_method_get_policy.inc` without changing behavior.

The file had grown into a mixed policy/metadata/emit owner. This row extracts
only the newest LocalI64Map entry-table family:

```text
metadata reader:
  hako_llvmc_ffi_local_i64_map_entry_table_metadata.inc

emitter:
  hako_llvmc_ffi_local_i64_map_entry_table_emit.inc
```

`generic_method_get_policy.inc` remains the compatibility facade and keeps the
existing `emit_generic_method_get_fallback_by_policy()` entry.

## Contract

```text
output_contract=hako-cabi-get-policy-split-v0
source_evidence=chatgpt-pro-cabi-get-policy-split-consultation
row_kind=boxshape_refactor

split_scope=local_i64_map_entry_table_only
metadata_reader_file=lang/c-abi/shims/hako_llvmc_ffi_local_i64_map_entry_table_metadata.inc
emitter_file=lang/c-abi/shims/hako_llvmc_ffi_local_i64_map_entry_table_emit.inc
facade_file=lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc

get_policy_lines_before=715
get_policy_lines_after=609
behavior_changed=0
route_vocab_changed=0
decision_struct_added=0
generic_method_match_changed=0
mir_call_route_policy_changed=0

product_mapbox_storage_changed=0
hasher_policy_changed=0
mirbuilder_object_management_enabled=0
helper_name_inference_added=0
benchmark_name_branch_added=0

next_task=MIMALLOC-ARRAY-TEXT-LOOP-SESSION-PLAN-INVENTORY-001
summary=ok
```

## Validation

```bash
git diff --check -- \
  lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc \
  lang/c-abi/shims/hako_llvmc_ffi_local_i64_map_entry_table_metadata.inc \
  lang/c-abi/shims/hako_llvmc_ffi_local_i64_map_entry_table_emit.inc

bash tools/perf/build_perf_release.sh

target/release/hakorune --emit-mir-json \
  /tmp/kilo_leaf_map_get_dynamic_covered_i64.split.mir.json \
  benchmarks/bench_kilo_leaf_map_get_dynamic_covered_i64.hako

target/release/ny-llvmc \
  --in /tmp/kilo_leaf_map_get_dynamic_covered_i64.split.mir.json \
  --out /tmp/kilo_leaf_map_get_dynamic_covered_i64.split.exe \
  --emit exe \
  --nyrt target/release

/tmp/kilo_leaf_map_get_dynamic_covered_i64.split.exe

nm -C /tmp/kilo_leaf_map_get_dynamic_covered_i64.split.exe | \
  rg 'ny_main|nyash\.map\.(slot_load_hh|scalar_load_hi|local_i64_get_hi)'
```

Observed:

```text
Result: 4000001
rc=1
symbols:
  ny_main
  nyash.map.slot_load_hh
```

The return code remains the program result convention. The semantic output is
unchanged from 296x-927.

## Stop Lines

- do not introduce `GenericMethodGetDecision` in this row
- do not split route vocabulary yet
- do not move metadata policy in this row
- do not touch `generic_method_match.inc`
- do not touch `mir_call_route_policy.inc`
- do not change product `MapBox` storage
- do not add helper-name / benchmark-name inference
