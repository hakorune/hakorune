---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-064.
Related:
  - docs/development/current/main/phases/phase-296x/296x-561-MIM-PORT-FMEM-063-ABANDONED-RECLAIM-PRODUCER-PILOT.md
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-562 MIM-PORT-FMEM-064 Product Activation Preflight

## Purpose

Select the product activation boundary after abandoned reclaim producer evidence
is open. This row is preflight only: it must keep product activation, hook
installation, global allocator claim, and winner claim closed.

## Required Boundaries

```text
process allocator replacement remains closed
hook installation remains closed
global allocator claim remains closed
winner claim remains closed
full .hako mimalloc algorithm claim remains closed
```

## Acceptance Sketch

```text
replacement_front_selected_route=product_activation_preflight
abandoned_reclaim_enabled=1
product_activation_selected=1
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
```

## Landed Evidence

```text
replacement_front_selected_route=product_activation_preflight
replacement_front_selected_memop_family=product_activation
replacement_front_selected_memop_kinds=ProductActivation
replacement_front_next_producer_slice=product_activation_producer_pilot
product_activation_selected=1
abandoned_reclaim_selected=1
abandoned_reclaim_enabled=1
product_activation=0
hook_install=0
global_allocator_claim=0
winner_claim=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
```

## Verification

```bash
python3 -m py_compile tools/hako_check/fastmem_mir_to_llvm_producer_report.py tools/hako_check/fastmem_check.py
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
```

## Next

```text
MIM-PORT-FMEM-065 product activation producer pilot.
```

## Non-goals

```text
product activation producer behavior
hook installation
global allocator replacement
winner claim
Python-template C bridge restoration
```
