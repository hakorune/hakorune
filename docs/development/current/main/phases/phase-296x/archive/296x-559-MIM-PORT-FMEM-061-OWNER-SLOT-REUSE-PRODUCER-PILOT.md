---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-061.
Related:
  - docs/development/current/main/phases/phase-296x/296x-558-MIM-PORT-FMEM-060-OWNER-SLOT-REUSE-PREFLIGHT.md
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-559 MIM-PORT-FMEM-061 Owner Slot Reuse Producer Pilot

## Purpose

Open producer evidence for generation-safe owner slot reuse after the preflight
row selected the boundary. This row may mark slot reuse as enabled only when the
report continues to prove that reuse without generation bump is zero.

## Required Boundaries

```text
generation bump proof remains required
abandoned reclaim behavior remains closed
process allocator replacement remains closed
hook installation remains closed
global allocator claim remains closed
winner claim remains closed
full .hako mimalloc algorithm claim remains closed
```

## Acceptance Sketch

```text
replacement_front_selected_route=owner_slot_reuse_producer_pilot
tls_backing_transfer_selected=1
tls_backing_transfer_enabled=1
allocator_owner_slot_reuse_selected=1
allocator_owner_slot_reuse_enabled=1
allocator_owner_generation_bump_count>0
allocator_owner_reuse_without_generation_bump_count=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Landed Evidence

```text
replacement_front_selected_route=owner_slot_reuse_producer_pilot
replacement_front_selected_memop_family=owner_slot_reuse
replacement_front_selected_memop_kinds=OwnerSlotReuse
replacement_front_next_producer_slice=abandoned_reclaim_preflight
allocator_owner_slot_reuse_selected=1
allocator_owner_slot_reuse_enabled=1
allocator_owner_generation_bump_count=1
allocator_owner_reuse_without_generation_bump_count=0
tls_backing_transfer_selected=1
tls_backing_transfer_enabled=1
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Verification

```bash
python3 -m py_compile tools/hako_check/fastmem_mir_to_llvm_producer_report.py tools/hako_check/fastmem_check.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
```

## Next

```text
MIM-PORT-FMEM-062 abandoned reclaim preflight.
```

## Non-goals

```text
abandoned reclaim
allocator activation
global allocator replacement
Python-template C bridge restoration
```
