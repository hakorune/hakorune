---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-062.
Related:
  - docs/development/current/main/phases/phase-296x/296x-559-MIM-PORT-FMEM-061-OWNER-SLOT-REUSE-PRODUCER-PILOT.md
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-560 MIM-PORT-FMEM-062 Abandoned Reclaim Preflight

## Purpose

Select the abandoned reclaim boundary after generation-safe owner slot reuse
producer evidence is open. This row is preflight only: it must expose reclaim as
the next lifecycle slice without enabling reclaim behavior or allocator
activation.

## Required Boundaries

```text
abandoned reclaim behavior remains closed
reclaim with remote candidates remains forbidden
process allocator replacement remains closed
hook installation remains closed
global allocator claim remains closed
winner claim remains closed
full .hako mimalloc algorithm claim remains closed
```

## Acceptance Sketch

```text
replacement_front_selected_route=abandoned_reclaim_preflight
allocator_owner_slot_reuse_enabled=1
allocator_owner_generation_bump_count>0
abandoned_reclaim_selected=1
abandoned_reclaim_enabled=0
page_reclaimed_with_remote_candidates=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
```

## Non-goals

```text
abandoned reclaim producer behavior
allocator activation
global allocator replacement
Python-template C bridge restoration
```
