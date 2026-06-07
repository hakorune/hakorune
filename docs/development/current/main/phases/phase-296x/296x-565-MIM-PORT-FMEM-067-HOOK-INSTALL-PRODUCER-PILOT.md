---
Status: Active
Date: 2026-06-07
Scope: MIM-PORT-FMEM-067.
Related:
  - docs/development/current/main/phases/phase-296x/296x-564-MIM-PORT-FMEM-066-HOOK-INSTALL-PREFLIGHT.md
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-565 MIM-PORT-FMEM-067 Hook Install Producer Pilot

## Purpose

Open producer evidence for hook installation after the preflight row selects the
boundary. This row may mark hook installation evidence enabled, but must still
keep global allocator claim and winner claim closed.

## Required Boundaries

```text
global allocator claim remains closed
winner claim remains closed
full .hako mimalloc algorithm claim remains closed
Python-template C bridge restoration remains closed
```

## Acceptance Sketch

```text
replacement_front_selected_route=hook_install_producer_pilot
product_activation=1
hook_install_selected=1
hook_install=1
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
