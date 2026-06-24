---
Status: Done
Date: 2026-06-05
Scope: promote PageMapBridge evidence in benchmark-only replacement-front reports without opening product allocator activation.
Blocker: MIM-FMEM-009
Related:
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/design/mimalloc-replacement-front-fidelity-ssot.md
  - tools/hako_check/replacement_front_report.py
  - tools/hako_check/fastmem_capability_inventory.py
  - tools/hako_check/fastmem_check.py
---

# 296x-418 PageMapBridge Benchmark-Front Pilot

## Purpose

`MIM-FMEM-008` connected parsed `fastmem ContractName { ... }` regions to
source-facing FastMemory inventory/check metadata. This row promotes the
benchmark-only replacement-front PageMapBridge evidence into the stable report
surface so old `range_scan` free-path ownership cannot pass as a keeper.

This is still not product allocator replacement. The bridge is benchmark-front
evidence for the generated C replacement front.

## Decision

```text
page_map_bridge_benchmark_front_pilot=1
free_path_page_lookup_route=page_map_bridge
free_path_page_lookup_range_scan_count=0
page_map_bridge_kind=flat_side_table|page_base_mask|header_backptr
page_map_bridge_type_abi_hot_lookup_count=0
page_map_bridge_provider_abi_hot_dispatch_count=0

product_activation=0
hook_installed=0
global_allocator_claim=0
winner_claim=0
hako_mimalloc_algorithm_claim=0
replacement_front_is_full_hako_algorithm=0
```

## Scope

Accepted in this row:

```text
replacement-front report exposes page lookup route and bridge kind
fastmem-check rejects benchmark inventories that still use hot range_scan
dedicated smoke proves good PageMapBridge fixture and bad range_scan fixture
current docs point to MIM-FMEM-010 after completion
```

Left for later:

```text
TypedPageMetaHandle source/API surface
WorkerId / TLS arena owner state beyond existing report counters
AtomicRemoteHead plan/pilot
generated C algorithm changes not already present
product-shaped replacement front
activation / hooks / global allocator claim
```

## Acceptance

Positive fixture:

```text
replacement_front_page_bins_lookup_route=page_from_ptr_bridge
replacement_front_page_from_ptr_route=side_table_direct
free_path_page_lookup_route=page_map_bridge
page_map_bridge_kind=flat_side_table
free_path_page_lookup_range_scan_count=0
page_map_bridge_type_abi_hot_lookup_count=0
page_map_bridge_provider_abi_hot_dispatch_count=0
summary=ok
```

Negative fixture:

```text
replacement_front_page_bins_lookup_route=range_scan
free_path_page_lookup_route=range_scan
free_path_page_lookup_range_scan_count>0
fastmem-check summary=failed
```

Proof:

```bash
bash tools/hako_check/replacement_front_report_smoke.sh
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_page_map_bridge_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Landed Evidence

```text
replacement_front_report_page_map_bridge_fields=1
fastmem_check_rejects_range_scan=1
fastmem_page_map_bridge_smoke=pass
product_activation=0
hook_installed=0
global_allocator_claim=0
winner_claim=0
```

Proof:

```bash
bash tools/hako_check/replacement_front_report_smoke.sh
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_page_map_bridge_smoke.sh
python3 -m py_compile tools/hako_check/replacement_front_report.py tools/hako_check/fastmem_check.py tools/hako_check/fastmem_capability_inventory.py
bash tools/checks/current_state_pointer_guard.sh
```

Next row:

```text
MIM-FMEM-010 TypedPageMetaHandle plan
```

## Stop Line

- no allocator activation
- no hook install
- no global allocator claim
- no winner claim
- no Type ABI hot lookup
- no Provider ABI hot dispatch
- no full `.hako` mimalloc algorithm claim
