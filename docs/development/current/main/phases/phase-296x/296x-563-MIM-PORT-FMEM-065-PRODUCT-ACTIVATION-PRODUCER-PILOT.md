---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-065.
Related:
  - docs/development/current/main/phases/phase-296x/296x-562-MIM-PORT-FMEM-064-PRODUCT-ACTIVATION-PREFLIGHT.md
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-563 MIM-PORT-FMEM-065 Product Activation Producer Pilot

## Purpose

Open producer evidence for product activation readiness after the preflight row
selects the boundary. This row may mark product activation evidence enabled, but
must still keep hook installation, global allocator claim, and winner claim
closed.

## Required Boundaries

```text
hook installation remains closed
global allocator claim remains closed
winner claim remains closed
full .hako mimalloc algorithm claim remains closed
Python-template C bridge restoration remains closed
```

## Acceptance Sketch

```text
replacement_front_selected_route=product_activation_producer_pilot
product_activation_selected=1
product_activation=1
hook_install=0
global_allocator_claim=0
winner_claim=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
```

## Landed Evidence

```text
replacement_front_selected_route=product_activation_producer_pilot
replacement_front_selected_memop_family=product_activation
replacement_front_selected_memop_kinds=ProductActivation
replacement_front_next_producer_slice=hook_install_preflight
product_activation_selected=1
product_activation=1
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
MIM-PORT-FMEM-066 hook install preflight.
```

## Non-goals

```text
hook installation
process allocator replacement
global allocator replacement
winner claim
Python-template C bridge restoration
```
