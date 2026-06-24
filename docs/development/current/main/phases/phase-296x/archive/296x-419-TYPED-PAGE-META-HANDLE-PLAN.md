---
Status: Done
Date: 2026-06-05
Scope: promote TypedPageMetaHandle report evidence without opening product allocator activation.
Blocker: MIM-FMEM-010
Related:
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - tools/hako_check/fastmem_capability_inventory.py
  - tools/hako_check/fastmem_check.py
---

# 296x-419 TypedPageMetaHandle Plan

## Purpose

`MIM-FMEM-009` made PageMapBridge evidence visible and rejected hot `range_scan`
free-path ownership. This row fixes the next boundary: the bridge must return a
layout-verified `TypedPageMetaHandle`, not an unstructured metadata pointer or
unverified offset-load route.

This is still report metadata only. It does not change generated C behavior,
open product allocator activation, or introduce broad raw pointer semantics.

## Decision

```text
typed_page_meta_handle=1
typed_page_meta_layout_verified=1
typed_page_meta_layout_id=PageMetaLayoutV0
typed_page_meta_field_count=7
typed_page_meta_required_field_missing_count=0

fastmem_layout_verified=1
fastmem_unverified_offset_load_count=0

product_activation=0
hook_installed=0
global_allocator_claim=0
winner_claim=0
hako_mimalloc_algorithm_claim=0
replacement_front_is_full_hako_algorithm=0
```

## Required Fields

```text
owner_worker_id
block_size
free_head
local_free_head
remote_head
capacity
used
```

## Scope

Accepted in this row:

```text
fastmem capability inventory reads typed_page_meta_* fields from benchmark reports
fastmem-check rejects missing required PageMeta fields
dedicated smoke proves complete and incomplete PageMeta fixtures
current docs point to MIM-FMEM-011 after completion
```

Left for later:

```text
WorkerId / TLS arena owner-state implementation
AtomicRemoteHead plan/pilot
generated C algorithm changes
source-level safe capability wrappers
product-shaped replacement front
activation / hooks / global allocator claim
```

## Acceptance

Positive fixture:

```text
typed_page_meta_handle=1
typed_page_meta_layout_verified=1
typed_page_meta_layout_id=PageMetaLayoutV0
typed_page_meta_field_count=7
typed_page_meta_required_field_missing_count=0
fastmem_unverified_offset_load_count=0
summary=ok
```

Negative fixture:

```text
typed_page_meta_handle=1
typed_page_meta_field_remote_head=0
typed_page_meta_required_field_missing_count=1
fastmem-check summary=failed
```

Proof:

```bash
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_page_map_bridge_smoke.sh
bash tools/hako_check/fastmem_typed_page_meta_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Landed Evidence

```text
typed_page_meta_report_surface=1
typed_page_meta_required_fields=7
typed_page_meta_missing_field_rejected=1
product_activation=0
hook_installed=0
global_allocator_claim=0
winner_claim=0
```

Next row:

```text
MIM-FMEM-011 WorkerId / TLS arena owner state
```

## Stop Line

- no allocator activation
- no hook install
- no global allocator claim
- no winner claim
- no Type ABI hot lookup
- no Provider ABI hot dispatch
- no full `.hako` mimalloc algorithm claim
- no arbitrary offset-load route
