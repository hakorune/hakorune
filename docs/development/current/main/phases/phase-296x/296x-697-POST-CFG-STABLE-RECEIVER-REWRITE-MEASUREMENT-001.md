---
Status: Landed
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
post_cfg_stable_receiver_rewrite_measurement_landed=1
source_evidence=296x-696
measurement_run=1
winner_claim=0
summary=ok
```

## Result

```text
output_contract=hako-mimalloc-post-cfg-stable-receiver-rewrite-measurement-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-696
measurement_repeat_count=4
sample_0_hako_body_elapsed_ns=7000000
sample_0_c_body_elapsed_ns=4044673
sample_0_body_elapsed_ratio=1.731
sample_1_hako_body_elapsed_ns=7000000
sample_1_c_body_elapsed_ns=3248430
sample_1_body_elapsed_ratio=2.155
sample_2_hako_body_elapsed_ns=6000000
sample_2_c_body_elapsed_ns=3994552
sample_2_body_elapsed_ratio=1.502
sample_3_hako_body_elapsed_ns=6000000
sample_3_c_body_elapsed_ns=3929710
sample_3_body_elapsed_ratio=1.527
hako_body_elapsed_ns=6500000
c_body_elapsed_ns=3962131
body_elapsed_gap_ns=2537869
body_elapsed_ratio=1.640
winner_claim=0
next_task=post_cfg_stable_receiver_rewrite_stability_measurement
summary=ok
```

Interpretation:

```text
The body timing surface moved from the previous 300ms class to 6-7ms in Hako
body timing. This is a material improvement, but the row still does not claim a
winner: repeat/stability measurement should confirm the new floor before
closing the owner or choosing another implementation family.
```
