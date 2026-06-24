---
Status: Done
Date: 2026-06-05
Scope: prove product-shaped page-local replacement-front evidence is tied to `.hako` `HakoAllocPageModel` without changing allocator execution.
Blocker: MIM-FMEM-017C
Related:
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - lang/src/hako_alloc/memory/page_box.hako
  - tools/hako_check/replacement_front_report.py
  - tools/hako_check/fastmem_capability_inventory.py
  - tools/hako_check/fastmem_check.py
---

# 296x-432 Page-Local Bridge Evidence

## Purpose

`MIM-FMEM-017B` tied replacement-front size-class evidence to `.hako`
`SizeClassBox`. `MIM-FMEM-017C` ties the next product-shaped surface,
page-local state, to `.hako` `HakoAllocPageModel`.

This row is report/check-only.

## Decision

```text
source_truth=hako_alloc.page_box
source_file=lang/src/hako_alloc/memory/page_box.hako
generated_c_behavior_change=0
source_syntax_change=0
rust_parser_change=0
hako_parser_change=0
remote_free_execution_claim=0
product_activation=0
hook_installed=0
global_allocator_product_claim=0
winner_claim=0
```

The bridge proves only page-local state:

```text
free
local_free
block_used
used
free_top
local_free_top
capacity
reserved
block_size
acquire
releaseLocal
releaseLocalKnownLive
reactivate / lifecycle observer shape
```

It does not prove remote-free completion, segment backing, or product
activation.

## Fields

```text
replacement_front_page_local_bridge_v0=1
replacement_front_page_local_bridge_report_only=1
replacement_front_page_local_bridge_source_truth=hako_alloc.page_box
replacement_front_page_local_bridge_source_file=lang/src/hako_alloc/memory/page_box.hako
replacement_front_page_local_bridge_mirror_source=hako_page_box_report_mirror
replacement_front_page_local_bridge_bound=0|1
replacement_front_page_local_bridge_missing=...

replacement_front_page_local_required_field_count=9
replacement_front_page_local_required_fields_present=0|1
replacement_front_page_local_missing_fields=none|...
replacement_front_page_local_required_method_count=10
replacement_front_page_local_required_methods_present=0|1
replacement_front_page_local_missing_methods=none|...

replacement_front_page_local_directarray_fields_present=0|1
replacement_front_page_local_counter_fields_present=0|1
replacement_front_page_local_acquire_release_methods_present=0|1
replacement_front_page_local_lifecycle_methods_present=0|1
replacement_front_page_local_typed_meta_matches_source=0|1
replacement_front_page_local_same_owner_route_matches_source=0|1
replacement_front_page_local_no_remote_free_claim=1
```

## Acceptance

```text
replacement_front_page_local_bridge_v0=1
replacement_front_page_local_bridge_report_only=1
replacement_front_page_local_bridge_source_truth=hako_alloc.page_box
replacement_front_page_local_bridge_bound=1
replacement_front_page_local_bridge_missing=none
replacement_front_page_local_required_fields_present=1
replacement_front_page_local_required_methods_present=1
replacement_front_page_local_directarray_fields_present=1
replacement_front_page_local_counter_fields_present=1
replacement_front_page_local_acquire_release_methods_present=1
replacement_front_page_local_lifecycle_methods_present=1
replacement_front_page_local_typed_meta_matches_source=1
replacement_front_page_local_same_owner_route_matches_source=1
replacement_front_page_local_no_remote_free_claim=1
product_activation_ready=0
type_abi_hot_path_lookup_count=0
provider_dispatch_hot_path=0
```

Proof:

```bash
python3 -m py_compile tools/hako_check/replacement_front_report.py tools/hako_check/fastmem_capability_inventory.py tools/hako_check/fastmem_check.py
bash tools/hako_check/replacement_front_report_smoke.sh
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/smokes/v2/profiles/integration/parser/fastmem_parser_parity_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

- do not change `HakoAllocPageModel` behavior
- do not add or change source syntax
- do not change generated C malloc/free behavior
- do not claim remote-free completion from this row
- do not open product allocator activation
- do not install hooks, claim global allocator ownership, or make winner claims

## Landed Evidence

```text
replacement_front_page_local_bridge_v0=1
replacement_front_page_local_bridge_source_truth=hako_alloc.page_box
replacement_front_page_local_bridge_mirror_source=hako_page_box_report_mirror
replacement_front_page_local_bridge_bound=1
replacement_front_page_local_bridge_missing=none
replacement_front_page_local_required_field_count=9
replacement_front_page_local_required_fields_present=1
replacement_front_page_local_required_method_count=10
replacement_front_page_local_required_methods_present=1
replacement_front_page_local_typed_meta_matches_source=1
replacement_front_page_local_same_owner_route_matches_source=1
replacement_front_page_local_no_remote_free_claim=1
generated_c_behavior_change=0
source_syntax_change=0
remote_free_execution_claim=0
product_activation=0
```

Next row:

```text
MIM-FMEM-017D Replacement-front producer taxonomy
```
