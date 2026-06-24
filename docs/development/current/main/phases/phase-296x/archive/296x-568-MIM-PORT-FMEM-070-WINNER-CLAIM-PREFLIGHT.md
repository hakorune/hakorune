---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-070.
Related:
  - docs/development/current/main/phases/phase-296x/296x-567-MIM-PORT-FMEM-069-GLOBAL-ALLOCATOR-CLAIM-PRODUCER-PILOT.md
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-568 MIM-PORT-FMEM-070 Winner Claim Preflight

## Purpose

Select the final winner-claim boundary after global allocator claim producer
evidence is open. This row is preflight only: it must keep `winner_claim=0`.

## Required Boundaries

```text
winner claim behavior remains closed
full .hako mimalloc algorithm claim remains closed
Python-template C bridge restoration remains closed
```

## Acceptance Sketch

```text
replacement_front_selected_route=winner_claim_preflight
product_activation=1
hook_install=1
global_allocator_claim=1
winner_claim_selected=1
winner_claim=0
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
```

## Non-goals

```text
winner claim producer
Python-template C bridge restoration
```

## Landed Evidence

```text
fastmem_winner_claim_preflight=1
replacement_front_selected_route=winner_claim_preflight
replacement_front_selected_memop_family=winner_claim
replacement_front_selected_memop_kinds=WinnerClaim
replacement_front_next_producer_slice=winner_claim_producer_pilot
replacement_front_deferred_memop_kinds=WinnerClaimProducer
winner_claim_selected=1
global_allocator_claim=1
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
MIM-PORT-FMEM-071 winner claim producer pilot
```
