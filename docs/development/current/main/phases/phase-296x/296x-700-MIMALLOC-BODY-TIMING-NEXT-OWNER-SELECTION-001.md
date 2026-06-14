---
Status: Active
Date: 2026-06-15
Task: MIMALLOC-BODY-TIMING-NEXT-OWNER-SELECTION-001
Scope: Select the next optimization owner from the current product-route
  object-lifecycle body-timing surface after the receiver operand copy-chain
  keeper closed.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-699-MIMALLOC-BODY-TIMING-CFG-STABLE-RECEIVER-REWRITE-CLOSEOUT-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# MIMALLOC-BODY-TIMING-NEXT-OWNER-SELECTION-001

## Purpose

296x-699 closed the receiver operand copy-chain owner:

```text
winner_claim=1
receiver_operand_copy_chain_owner_closed=1
stable_body_elapsed_ratio=1.790
```

This row must select the next owner from fresh evidence, or pause if no strong
owner remains. It must not continue patching receiver operands.

## Required Output

```text
output_contract=hako-mimalloc-body-timing-next-owner-selection-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-699
current_body_elapsed_ratio=<ratio>
receiver_operand_copy_chain_owner_closed=1
selected_next_owner=<owner|pause>
selected_owner_confidence=<low|medium|high>
implementation_started=0
startup_lane_reopened=0
source_hako_changed=0
winner_claim=0
summary=ok
```

## Stop Line

```text
do not change code in this row
do not patch source .hako
do not reopen startup optimization
do not return to receiver operand copy-chain owner
do not select another implementation owner without evidence
```

## Acceptance

```text
mimalloc_body_timing_next_owner_selection_active=1
source_evidence=296x-699
implementation_started=0
winner_claim=0
summary=pending
```
