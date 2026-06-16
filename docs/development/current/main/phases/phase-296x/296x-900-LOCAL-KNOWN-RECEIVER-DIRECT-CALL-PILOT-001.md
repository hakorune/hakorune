# 296x-900 LOCAL-KNOWN-RECEIVER-DIRECT-CALL-PILOT-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-known-receiver-direct-call-pilot-v0
source_evidence=296x-899
row_kind=implementation

backend_reads_local_fastpath_fact=1
backend_reads_fallback_evidence=0
backend_reads_helper_symbol=0
backend_reads_source_variable_name=0
selected_backend=src/llvm_py/instructions/mir_call/collection_method_call.py
selected_fact_route=local_fastpath.known_receiver_direct_call
selected_backend_kind=known_receiver_direct_call
selected_route_plan=map_repr.generic_hash_runtime
selected_helper=nyash.map.local_i64_get_hi
local_fastpath_metadata_loader_enabled=1
local_fastpath_metadata_field=local_fastpath_facts
function_lower_loads_local_fastpath_facts=1

fallback_reason_blocks_fastpath=1
hosthandle_bypass_enabled=0
direct_storage_enabled=0
product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0
winner_claim=0
next_task=LOCAL-KNOWN-RECEIVER-DIRECT-CALL-MEASUREMENT-001
summary=ok
```

## Implementation

The Python LLVM collection method lowering now checks
`resolver.local_fastpath_facts_by_site` before the older map representation
shadow metadata. A positive fact must have:

```text
route_id=local_fastpath.known_receiver_direct_call
fact_kind=local_fastpath_fact
backend_kind=known_receiver_direct_call
route_plan=map_repr.generic_hash_runtime
fallback_reason=none
```

When those fields match the current receiver/key site, `MapBox.get` emits the
existing local-i64 pilot helper:

```text
nyash.map.local_i64_get_hi(handle, key_i64)
```

This row changes the backend proof source, not product MapBox storage.

The function metadata loader indexes MIR JSON `metadata.local_fastpath_facts`
by `(block, instruction_index)` and installs them into
`resolver.local_fastpath_facts_by_site`. This keeps the backend consumer tied
to positive facts only; it does not make observations or fallback evidence
backend-readable.

## Tests

```bash
PYTHONPATH=.:src/llvm_py python3 -m unittest \
  src.llvm_py.tests.test_collection_method_call.TestCollectionMethodCall.test_mapbox_local_fastpath_fact_get_uses_known_receiver_direct_call_helper \
  src.llvm_py.tests.test_collection_method_call.TestCollectionMethodCall.test_mapbox_local_fastpath_fact_get_ignores_fallback_reason \
  src.llvm_py.tests.test_fastmem_metadata_loader.TestFastMemMetadataLoader.test_local_fastpath_fact_loader_indexes_sites
```

## Stop Lines

- no HostHandle bypass
- no direct storage enablement
- no product MapBox storage change
- no product hasher swap
- no sidecar storage
- no MIRBuilder map storage ownership
- no helper-name / source-variable-name inference
- no Hako-vs-C winner claim
