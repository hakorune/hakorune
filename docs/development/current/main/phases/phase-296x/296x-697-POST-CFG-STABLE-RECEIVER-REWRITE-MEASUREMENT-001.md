---
Status: Active
Date: 2026-06-15
Task: POST-CFG-STABLE-RECEIVER-REWRITE-MEASUREMENT-001
Scope: Remeasure product-route object-lifecycle body timing after the
  CFG-stable receiver operand rewrite.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-696-CALL-OPERAND-CFG-STABLE-RECEIVER-REWRITE-IMPLEMENTATION-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# POST-CFG-STABLE-RECEIVER-REWRITE-MEASUREMENT-001

## Purpose

296x-696 removed the selected receiver operand Copy-chain family:

```text
post_selected_keeper_candidate_count=0
post_call_operand_unique_copy_count=13
```

This row measures whether that MIR shape change materially improves the active
product-route body timing surface before any winner or next-owner claim.

## Required Output

```text
output_contract=hako-mimalloc-post-cfg-stable-receiver-rewrite-measurement-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-696
hako_body_elapsed_ns=<n>
c_body_elapsed_ns=<n>
body_elapsed_ratio=<float>
winner_claim=0
next_owner=<owner|unclear>
summary=ok
```

## Stop Line

```text
do not patch source .hako
do not reopen startup optimization
do not claim a winner from MIR shape alone
do not select a new implementation owner without body timing evidence
```

## Acceptance

```text
post_cfg_stable_receiver_rewrite_measurement_active=1
source_evidence=296x-696
winner_claim=0
summary=pending
```
