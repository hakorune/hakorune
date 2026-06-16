# 296x-901 LOCAL-FASTPATH-FACT-METADATA-SURFACE-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-fastpath-fact-metadata-surface-v0
source_evidence=296x-900
row_kind=transport_surface

function_metadata_local_fastpath_facts=1
mir_json_emits_local_fastpath_facts=1
json_site_id_preserved=1
json_block_instruction_index_preserved=1
python_loader_indexes_by_block_instruction=1
backend_reads_positive_fact_only=1

automatic_fact_producer_enabled=0
fallback_fact_enabled=0
fallback_evidence_exported=0
helper_symbol_inference=0
source_variable_name_inference=0
hosthandle_bypass_enabled=0
direct_storage_enabled=0
product_mapbox_storage_changed=0
product_hasher_swap=0
winner_claim=0
next_task=LOCAL-FASTPATH-FACT-PRODUCER-SELECTION-001
summary=ok
```

## Decision

`LocalFastPathFact` now has a durable transport surface:

```text
FunctionMetadata.local_fastpath_facts
  -> MIR JSON metadata.local_fastpath_facts
  -> Python LLVM metadata loader
  -> resolver.local_fastpath_facts_by_site
  -> backend positive-Fact consumer
```

This row intentionally does not create facts automatically. It only makes the
transport path real so the next row can select a narrow producer without
mixing producer policy with backend lowering.

## Invariant

`site_id` is a stable report/proof identifier. Backend lookup uses the explicit
`block` and `instruction_index` fields. Do not derive backend call-site lookup
from `site_id`.

## Stop Lines

- no fallback Fact producer
- no observation / fallback evidence export
- no helper-name or source-variable-name inference
- no HostHandle bypass
- no direct storage enablement
- no product MapBox storage or hasher change
- no Hako-vs-C winner claim

## Validation

```bash
cargo test --lib runner::mir_json_emit::tests::map_repr_plans::build_mir_json_root_emits_local_fastpath_facts
PYTHONPATH=.:src/llvm_py python3 -m unittest \
  src.llvm_py.tests.test_fastmem_metadata_loader.TestFastMemMetadataLoader.test_local_fastpath_fact_loader_indexes_sites
bash tools/checks/k2_wide_phase296x_local_fastpath_fact_metadata_surface_guard.sh
```
