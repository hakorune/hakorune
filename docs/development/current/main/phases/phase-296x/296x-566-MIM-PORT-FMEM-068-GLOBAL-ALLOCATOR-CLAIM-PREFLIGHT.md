---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-068.
Related:
  - docs/development/current/main/phases/phase-296x/296x-565-MIM-PORT-FMEM-067-HOOK-INSTALL-PRODUCER-PILOT.md
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-566 MIM-PORT-FMEM-068 Global Allocator Claim Preflight

## Purpose

Select the global allocator claim boundary after hook installation producer
evidence is open. This row is preflight only: it must keep global allocator
claim and winner claim closed.

## Required Boundaries

```text
global allocator claim behavior remains closed
winner claim remains closed
full .hako mimalloc algorithm claim remains closed
Python-template C bridge restoration remains closed
```

## Acceptance Sketch

```text
replacement_front_selected_route=global_allocator_claim_preflight
product_activation=1
hook_install=1
global_allocator_claim_selected=1
global_allocator_claim=0
winner_claim=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
```

## Non-goals

```text
global allocator replacement
winner claim
Python-template C bridge restoration
```
