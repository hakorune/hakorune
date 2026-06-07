---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-059.
Related:
  - docs/development/current/main/phases/phase-296x/296x-556-MIM-PORT-FMEM-058-TLS-BACKING-TRANSFER-PREFLIGHT.md
  - lang/src/hako_alloc/memory/page_meta_fastmem_branch_cfg_lowering_box.hako
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-557 MIM-PORT-FMEM-059 TLS Backing Transfer Producer Pilot

## Purpose

Open the producer evidence for TLS backing transfer after the preflight row
selected the boundary. This row is still non-activating and does not open owner
slot reuse, abandoned reclaim, hook installation, global allocator replacement,
or winner claims.

## Required Boundaries

```text
owner slot reuse remains closed
abandoned reclaim behavior remains closed
process allocator replacement remains closed
hook installation remains closed
global allocator claim remains closed
winner claim remains closed
full .hako mimalloc algorithm claim remains closed
```

## Acceptance Sketch

```text
replacement_front_selected_route=tls_backing_transfer_producer_pilot
page_local_free_route_cfg_selected=1
page_local_free_route_cfg_lowering_enabled=1
tls_backing_transfer_selected=1
tls_backing_transfer_enabled=1
allocator_owner_slot_reuse_enabled=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Landed Evidence

```text
replacement_front_selected_route=tls_backing_transfer_producer_pilot
replacement_front_selected_memop_family=tls_backing_transfer
replacement_front_selected_memop_kinds=TlsBackingTransfer
replacement_front_next_producer_slice=owner_slot_reuse_preflight
tls_backing_transfer_selected=1
tls_backing_transfer_enabled=1
page_local_free_route_cfg_selected=1
page_local_free_route_cfg_lowering_enabled=1
allocator_owner_slot_reuse_enabled=0
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
MIM-PORT-FMEM-060 owner slot reuse preflight.
```

## Non-goals

```text
owner slot reuse
abandoned reclaim
allocator activation
global allocator replacement
Python-template C bridge restoration
```
