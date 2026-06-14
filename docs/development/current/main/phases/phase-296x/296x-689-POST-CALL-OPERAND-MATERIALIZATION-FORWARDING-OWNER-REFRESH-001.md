---
Status: Active
Date: 2026-06-15
Task: POST-CALL-OPERAND-MATERIALIZATION-FORWARDING-OWNER-REFRESH-001
Scope: Refresh the post-implementation owner after the call-operand forwarding
  keeper failed to improve body timing.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-688-POST-CALL-OPERAND-MATERIALIZATION-FORWARDING-MEASUREMENT-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# POST-CALL-OPERAND-MATERIALIZATION-FORWARDING-OWNER-REFRESH-001

## Purpose

296x-688 measured:

```text
post_selected_keeper_candidate_count=0
post_call_operand_unique_copy_count=27
hako_body_elapsed_ns=375000000
c_body_elapsed_ns=3255360
body_elapsed_ratio=115.195
winner_claim=0
selected_owner_confidence=low
```

The tiny MIR shape keeper is not enough to claim a performance win. This row
refreshes owner selection before any further implementation attempt.

## Required Output

```text
output_contract=hako-mimalloc-post-call-operand-materialization-forwarding-owner-refresh-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-688
hako_body_elapsed_ns=375000000
c_body_elapsed_ns=3255360
body_elapsed_ratio=115.195
copy_count=54
call_operand_route_carrier_copy_count=27
call_adjacent_copy_count=27
dominant_copy_owner=local_ssa_copy_materialization
dominant_position=call_adjacent
dominant_route_carrier_role=call_operand
selected_next_owner=<owner>
selected_owner_confidence=<low|medium|high>
next_task=<task>
implementation_started=0
optimization_open=0
winner_claim=0
summary=ok
```

## Stop Line

```text
do not implement in this row
do not patch source .hako
do not reopen startup optimization
do not broaden LocalSSA copy coalescing
do not claim a winner
```

## Acceptance

```text
post_call_operand_materialization_forwarding_owner_refresh_active=1
source_evidence=296x-688
owner_refresh_run=0
selected_next_owner=0
implementation_started=0
optimization_open=0
winner_claim=0
summary=pending
```
