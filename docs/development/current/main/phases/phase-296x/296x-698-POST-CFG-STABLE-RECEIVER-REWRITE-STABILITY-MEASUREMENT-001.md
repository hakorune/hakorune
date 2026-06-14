---
Status: Landed
Date: 2026-06-15
Task: POST-CFG-STABLE-RECEIVER-REWRITE-STABILITY-MEASUREMENT-001
Scope: Confirm the post-CFG-stable receiver rewrite body-timing floor before
  closing the owner or selecting the next implementation family.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-697-POST-CFG-STABLE-RECEIVER-REWRITE-MEASUREMENT-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# POST-CFG-STABLE-RECEIVER-REWRITE-STABILITY-MEASUREMENT-001

## Purpose

296x-697 measured a large body-time improvement after the CFG-stable receiver
operand rewrite:

```text
measurement_repeat_count=4
hako_body_elapsed_ns=6500000
c_body_elapsed_ns=3962131
body_elapsed_ratio=1.640
winner_claim=0
```

This row repeats the measurement enough to decide whether to close the current
owner or continue with a new owner refresh.

## Required Output

```text
output_contract=hako-mimalloc-post-cfg-stable-receiver-rewrite-stability-measurement-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-697
measurement_repeat_count=<n>
hako_body_elapsed_ns=<median_ns>
c_body_elapsed_ns=<median_ns>
body_elapsed_ratio=<ratio>
winner_claim=<0|1>
selected_next_owner=<owner|closeout>
selected_owner_confidence=<low|medium|high>
summary=ok
```

## Stop Line

```text
do not change code in this row
do not patch source .hako
do not reopen startup optimization
do not choose another implementation owner before stability evidence
```

## Acceptance

```text
post_cfg_stable_receiver_rewrite_stability_measurement_landed=1
source_evidence=296x-697
winner_claim=1
summary=ok
```

## Result

```text
output_contract=hako-mimalloc-post-cfg-stable-receiver-rewrite-stability-measurement-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-697
measurement_repeat_count=5
sample_0_hako_body_elapsed_ns=6000000
sample_0_c_body_elapsed_ns=3716279
sample_0_body_elapsed_ratio=1.615
sample_1_hako_body_elapsed_ns=6000000
sample_1_c_body_elapsed_ns=3191214
sample_1_body_elapsed_ratio=1.880
sample_2_hako_body_elapsed_ns=6000000
sample_2_c_body_elapsed_ns=3207986
sample_2_body_elapsed_ratio=1.870
sample_3_hako_body_elapsed_ns=7000000
sample_3_c_body_elapsed_ns=3700302
sample_3_body_elapsed_ratio=1.892
sample_4_hako_body_elapsed_ns=6000000
sample_4_c_body_elapsed_ns=3352143
sample_4_body_elapsed_ratio=1.790
hako_body_elapsed_ns=6000000
c_body_elapsed_ns=3352143
body_elapsed_ratio=1.790
winner_claim=1
selected_next_owner=closeout_current_receiver_operand_copy_chain_owner
selected_owner_confidence=high
next_task=mimalloc_body_timing_cfg_stable_receiver_rewrite_closeout
summary=ok
```

Interpretation:

```text
The new body timing floor is stable in the 6-7ms range. The CFG-stable receiver
operand rewrite is a real keeper for this product-route body timing front.
Close this receiver operand copy-chain owner before selecting any further
optimization family.
```
