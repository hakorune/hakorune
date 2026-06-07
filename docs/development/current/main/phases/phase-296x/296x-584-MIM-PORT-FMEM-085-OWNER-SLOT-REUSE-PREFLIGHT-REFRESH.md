---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-085.
Related:
  - docs/development/current/main/phases/phase-296x/296x-583-MIM-PORT-FMEM-084-TLS-BACKING-TRANSFER-PRODUCER-REFRESH.md
  - docs/development/current/main/phases/phase-296x/296x-558-MIM-PORT-FMEM-060-OWNER-SLOT-REUSE-PREFLIGHT.md
---

# 296x-584 MIM-PORT-FMEM-085 Owner Slot Reuse Preflight Refresh

## Purpose

Refresh the owner slot reuse preflight so it consumes the refreshed TLS backing
transfer producer evidence, which now depends on the page-local alloc/free route
body join boundary.

## Required Boundaries

```text
owner slot reuse behavior remains closed
abandoned reclaim behavior remains closed
product activation remains closed
hook installation remains closed
global allocator claim remains closed
winner claim remains closed
no new MemOp kind
no diagnostic Python-template C bridge retirement
```

## Acceptance Sketch

```text
replacement_front_selected_route=owner_slot_reuse_preflight_refresh
replacement_front_selected_memop_family=owner_slot_reuse
replacement_front_selected_memop_kinds=OwnerSlotReuse
replacement_front_next_producer_slice=owner_slot_reuse_producer_refresh

terminal_ladder_refresh_open=1
page_local_route_body_join_open=1
tls_backing_transfer_selected=1
tls_backing_transfer_enabled=1
allocator_owner_slot_reuse_selected=1
allocator_owner_slot_reuse_enabled=0

abandoned_reclaim_enabled=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
```

## Non-goals

```text
opening owner slot reuse behavior
abandoned reclaim
allocator activation
global allocator replacement
winner claim
```

## Landed Evidence

```text
replacement_front_selected_route=owner_slot_reuse_preflight_refresh
replacement_front_selected_memop_family=owner_slot_reuse
replacement_front_selected_memop_kinds=OwnerSlotReuse
replacement_front_next_producer_slice=owner_slot_reuse_producer_refresh
fastmem_allocator_owner_slot_reuse_preflight_refresh=1
terminal_ladder_refresh_open=1
page_local_route_body_join_open=1
tls_backing_transfer_selected=1
tls_backing_transfer_enabled=1
allocator_owner_slot_reuse_selected=1
allocator_owner_slot_reuse_enabled=0
abandoned_reclaim_enabled=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Verification

```bash
python3 -m py_compile tools/hako_check/fastmem_route_profiles.py tools/hako_check/fastmem_check_profile_functions.py tools/hako_check/fastmem_check_terminal_rules.py tools/hako_check/fastmem_mir_to_llvm_producer_report_rows.py tools/hako_check/fastmem_mir_to_llvm_producer_report_route_rows.py tools/hako_check/fastmem_mir_to_llvm_producer_report_body.py tools/hako_check/fastmem_mir_to_llvm_producer_report_tail_rows.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
```

## Next

```text
296x-585 MIM-PORT-FMEM-086 owner slot reuse producer refresh.
```
