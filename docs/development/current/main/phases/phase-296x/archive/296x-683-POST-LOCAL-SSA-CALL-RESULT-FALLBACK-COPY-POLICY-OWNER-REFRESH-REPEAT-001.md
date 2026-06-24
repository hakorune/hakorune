---
Status: Landed
Date: 2026-06-15
Task: POST-LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-OWNER-REFRESH-REPEAT-001
Scope: Repeat owner refresh after 296x-682 because the first post-keeper owner
  selection returned low confidence.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-682-POST-LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-MEASUREMENT-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# POST-LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-OWNER-REFRESH-REPEAT-001

## Purpose

296x-681 removed the selected MIR family, and 296x-682 remeasured:

```text
post_candidate_result_copy_count=0
post_terminal_compare_operand_count=0
copy_count=55
page_hotpath_helpers_attributed_copy_count=8
result_materialization_copy_count=7
body_elapsed_ratio=88.801
selected_next_owner=post_keeper_owner_unclear
selected_owner_confidence=low
```

This row repeats owner selection before any further implementation attempt.

## Required Output

```text
output_contract=hako-mimalloc-post-local-ssa-call-result-fallback-copy-policy-owner-refresh-repeat-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-682
copy_count=55
page_hotpath_helpers_attributed_copy_count=8
result_materialization_copy_count=7
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
do not broaden LocalSSA without a selected owner
do not claim a winner
```

## Result

```text
output_contract=hako-mimalloc-post-local-ssa-call-result-fallback-copy-policy-owner-refresh-repeat-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-682
hako_body_elapsed_ns=367000000
c_body_elapsed_ns=4132840
body_elapsed_ratio=88.801
copy_count=55
local_ssa_copy_materialization_copy_count=20
call_adjacent_copy_count=29
call_operand_route_carrier_copy_count=29
backend_route_carrier_copy_count=33
route_aware_candidate_copy_count=19
page_hotpath_helpers_attributed_copy_count=8
result_materialization_copy_count=7
dominant_copy_owner=local_ssa_copy_materialization
dominant_dynamic_owner=local_ssa_copy_materialization
dominant_position=call_adjacent
dominant_route_carrier_role=call_operand
selected_next_owner=call_operand_materialization_copy_chain_inventory
selected_owner_confidence=medium
selected_reason=same_current_mir_run_shows_call_operand_route_carrier_dominates_remaining_copy_surface
next_task=call_operand_materialization_copy_chain_inventory
implementation_started=0
optimization_open=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Interpretation:

```text
The prior owner refresh was too narrow for the post-681 MIR shape. Current MIR
still has copy_count=55, with dominant copy/dynamic owner
local_ssa_copy_materialization, dominant position call_adjacent, and dominant
route carrier call_operand.

Do not implement from this row. Open an inventory row for call operand
materialization copy chains first.
```

## Acceptance

```text
post_local_ssa_call_result_fallback_copy_policy_owner_refresh_repeat_landed=1
source_evidence=296x-682
owner_refresh_repeat_run=1
selected_next_owner=call_operand_materialization_copy_chain_inventory
selected_owner_confidence=medium
implementation_started=0
optimization_open=0
winner_claim=0
summary=ok
```
