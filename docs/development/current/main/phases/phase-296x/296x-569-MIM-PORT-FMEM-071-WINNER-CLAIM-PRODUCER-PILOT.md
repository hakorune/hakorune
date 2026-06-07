---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-071.
Related:
  - docs/development/current/main/phases/phase-296x/296x-568-MIM-PORT-FMEM-070-WINNER-CLAIM-PREFLIGHT.md
  - tools/hako_check/fastmem_mir_to_llvm_producer_report.py
  - tools/hako_check/fastmem_check.py
---

# 296x-569 MIM-PORT-FMEM-071 Winner Claim Producer Pilot

## Purpose

Open the final winner claim producer evidence after the preflight row selects
the boundary. This row is the first slice allowed to set `winner_claim=1`.

## Required Boundaries

```text
Python-template C bridge restoration remains closed
Type ABI hot lookup remains zero
Provider ABI hot dispatch remains zero
```

## Acceptance Sketch

```text
replacement_front_selected_route=winner_claim_producer_pilot
product_activation=1
hook_install=1
global_allocator_claim=1
winner_claim_selected=1
winner_claim=1
type_abi_hot_lookup_count=0
provider_abi_hot_dispatch_count=0
```

## Non-goals

```text
Python-template C bridge restoration
new allocator algorithm claim beyond the winner evidence row
```

## Landed Evidence

```text
fastmem_winner_claim_producer_pilot=1
replacement_front_selected_route=winner_claim_producer_pilot
replacement_front_selected_memop_family=winner_claim
replacement_front_selected_memop_kinds=WinnerClaim
replacement_front_next_producer_slice=complete
replacement_front_deferred_memop_kinds=none
winner_claim_selected=1
winner_claim=1
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
MIM-PORT-FMEM-072 winner claim closeout audit
```
