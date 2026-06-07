---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-066.
Related:
  - docs/development/current/main/phases/phase-296x/296x-563-MIM-PORT-FMEM-065-PRODUCT-ACTIVATION-PRODUCER-PILOT.md
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-564 MIM-PORT-FMEM-066 Hook Install Preflight

## Purpose

Select the hook installation boundary after product activation producer
evidence is open. This row is preflight only: it must keep hook installation,
global allocator claim, and winner claim closed.

## Required Boundaries

```text
hook installation behavior remains closed
global allocator claim remains closed
winner claim remains closed
full .hako mimalloc algorithm claim remains closed
Python-template C bridge restoration remains closed
```

## Acceptance Sketch

```text
replacement_front_selected_route=hook_install_preflight
product_activation=1
hook_install_selected=1
hook_install=0
global_allocator_claim=0
winner_claim=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
```

## Non-goals

```text
actual hook installation
global allocator replacement
winner claim
Python-template C bridge restoration
```
