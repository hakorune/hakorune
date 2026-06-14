---
Status: Active
Date: 2026-06-15
Task: MIMALLOC-BODY-TIMING-CFG-STABLE-RECEIVER-REWRITE-CLOSEOUT-001
Scope: Close out the CFG-stable receiver operand rewrite keeper and decide
  the next optimization lane boundary.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-698-POST-CFG-STABLE-RECEIVER-REWRITE-STABILITY-MEASUREMENT-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# MIMALLOC-BODY-TIMING-CFG-STABLE-RECEIVER-REWRITE-CLOSEOUT-001

## Purpose

296x-698 confirmed the post-rewrite body timing floor:

```text
measurement_repeat_count=5
hako_body_elapsed_ns=6000000
c_body_elapsed_ns=3352143
body_elapsed_ratio=1.790
winner_claim=1
selected_next_owner=closeout_current_receiver_operand_copy_chain_owner
```

This row closes the current receiver operand copy-chain owner before selecting
another implementation family.

## Required Output

```text
output_contract=hako-mimalloc-body-timing-cfg-stable-receiver-rewrite-closeout-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-698
keeper=cfg_stable_dominance_guarded_receiver_operand_rewrite
keeper_owner=mir_passes_callsite_canonicalize_receiver_operand_rewrite
pre_selected_keeper_candidate_count=13
post_selected_keeper_candidate_count=0
post_call_operand_unique_copy_count=13
stable_hako_body_elapsed_ns=6000000
stable_c_body_elapsed_ns=3352143
stable_body_elapsed_ratio=1.790
winner_claim=1
receiver_operand_copy_chain_owner_closed=1
startup_lane_reopened=0
source_hako_changed=0
next_task=<next-owner-refresh|pause>
summary=ok
```

## Stop Line

```text
do not change code in this row
do not patch source .hako
do not reopen startup optimization
do not choose another implementation family without explicit next-owner row
```

## Acceptance

```text
mimalloc_body_timing_cfg_stable_receiver_rewrite_closeout_active=1
source_evidence=296x-698
winner_claim=1
summary=pending
```
