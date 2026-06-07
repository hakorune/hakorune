---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-063.
Related:
  - docs/development/current/main/phases/phase-296x/296x-560-MIM-PORT-FMEM-062-ABANDONED-RECLAIM-PREFLIGHT.md
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-561 MIM-PORT-FMEM-063 Abandoned Reclaim Producer Pilot

## Purpose

Open producer evidence for abandoned reclaim only when remote-candidate safety
remains proven. This row may mark abandoned reclaim enabled, but must still keep
product activation and allocator replacement closed.

## Required Boundaries

```text
reclaim with remote candidates remains forbidden
process allocator replacement remains closed
hook installation remains closed
global allocator claim remains closed
winner claim remains closed
full .hako mimalloc algorithm claim remains closed
```

## Acceptance Sketch

```text
replacement_front_selected_route=abandoned_reclaim_producer_pilot
abandoned_reclaim_selected=1
abandoned_reclaim_enabled=1
page_reclaimed_with_remote_candidates=0
allocator_owner_slot_reuse_enabled=1
allocator_owner_generation_bump_count>0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Non-goals

```text
allocator activation
global allocator replacement
Python-template C bridge restoration
```
