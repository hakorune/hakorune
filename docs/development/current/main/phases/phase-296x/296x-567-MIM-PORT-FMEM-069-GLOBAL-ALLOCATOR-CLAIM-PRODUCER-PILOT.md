---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-069.
Related:
  - docs/development/current/main/phases/phase-296x/296x-566-MIM-PORT-FMEM-068-GLOBAL-ALLOCATOR-CLAIM-PREFLIGHT.md
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-567 MIM-PORT-FMEM-069 Global Allocator Claim Producer Pilot

## Purpose

Open producer evidence for the global allocator claim after the preflight row
has selected the boundary. This row may set `global_allocator_claim=1`, but it
must keep winner claim closed.

## Required Boundaries

```text
winner claim remains closed
full .hako mimalloc algorithm claim remains closed
Python-template C bridge restoration remains closed
```

## Acceptance Sketch

```text
replacement_front_selected_route=global_allocator_claim_producer_pilot
product_activation=1
hook_install=1
global_allocator_claim_selected=1
global_allocator_claim=1
winner_claim=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
```

## Non-goals

```text
winner claim
Python-template C bridge restoration
```
