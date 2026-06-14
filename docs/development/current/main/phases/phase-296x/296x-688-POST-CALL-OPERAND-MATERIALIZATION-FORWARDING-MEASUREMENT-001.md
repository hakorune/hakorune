---
Status: Active
Date: 2026-06-15
Task: POST-CALL-OPERAND-MATERIALIZATION-FORWARDING-MEASUREMENT-001
Scope: Remeasure product-route object-lifecycle body timing after the
  call-operand materialization forwarding keeper.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-687-CALL-OPERAND-MATERIALIZATION-FORWARDING-IMPLEMENTATION-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# POST-CALL-OPERAND-MATERIALIZATION-FORWARDING-MEASUREMENT-001

## Purpose

296x-687 removed the selected MIR shape:

```text
pre_selected_keeper_candidate_count=2
post_selected_keeper_candidate_count=0
post_call_operand_unique_copy_count=27
```

This row remeasures the product-route body timing surface before any winner
claim or next-owner selection.

## Required Output

```text
output_contract=hako-mimalloc-post-call-operand-materialization-forwarding-measurement-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-687
post_selected_keeper_candidate_count=0
post_call_operand_unique_copy_count=27
hako_body_elapsed_ns=<n>
c_body_elapsed_ns=<n>
body_elapsed_ratio=<ratio>
body_elapsed_gap_ns=<n>
winner_claim=<0|1>
selected_next_owner=<owner>
selected_owner_confidence=<low|medium|high>
next_task=<task>
optimization_open=0
summary=ok
```

## Stop Line

```text
do not change code in this row
do not patch source .hako
do not reopen startup optimization
do not claim winner from MIR shape alone
do not select a new owner without measurement evidence
```

## Acceptance

```text
post_call_operand_materialization_forwarding_measurement_active=1
source_evidence=296x-687
measurement_run=0
winner_claim=0
selected_next_owner=0
optimization_open=0
summary=pending
```
