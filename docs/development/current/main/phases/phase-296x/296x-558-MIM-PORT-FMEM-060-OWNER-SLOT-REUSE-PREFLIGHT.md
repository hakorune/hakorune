---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-060.
Related:
  - docs/development/current/main/phases/phase-296x/296x-557-MIM-PORT-FMEM-059-TLS-BACKING-TRANSFER-PRODUCER-PILOT.md
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-558 MIM-PORT-FMEM-060 Owner Slot Reuse Preflight

## Purpose

Select the owner slot reuse boundary after TLS backing transfer producer evidence
is open. This row is a preflight only: it must make the next ownership lifecycle
slice visible without enabling slot reuse behavior or allocator activation.

## Required Boundaries

```text
owner slot reuse behavior remains closed
abandoned reclaim behavior remains closed
process allocator replacement remains closed
hook installation remains closed
global allocator claim remains closed
winner claim remains closed
full .hako mimalloc algorithm claim remains closed
```

## Acceptance Sketch

```text
replacement_front_selected_route=owner_slot_reuse_preflight
tls_backing_transfer_selected=1
tls_backing_transfer_enabled=1
allocator_owner_slot_reuse_selected=1
allocator_owner_slot_reuse_enabled=0
allocator_owner_reuse_without_generation_bump_count=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Non-goals

```text
owner slot reuse producer behavior
abandoned reclaim
allocator activation
global allocator replacement
Python-template C bridge restoration
```
