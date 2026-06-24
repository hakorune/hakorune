---
Status: Landed
Date: 2026-06-15
Task: POST-LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-MEASUREMENT-001
Scope: Remeasure product-route object-lifecycle body timing after the 296x-681
  LocalSSA call-result fallback Copy keeper.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-681-LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-IMPLEMENTATION-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# POST-LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-MEASUREMENT-001

## Purpose

296x-681 removed the selected MIR shape:

```text
pre_candidate_result_copy_count=14
post_candidate_result_copy_count=0
pre_terminal_compare_operand_count=4
post_terminal_compare_operand_count=0
```

This row remeasures the product-route body timing surface before any winner
claim or next-owner selection.

## Required Output

```text
output_contract=hako-mimalloc-post-local-ssa-call-result-fallback-copy-policy-measurement-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-681
post_candidate_result_copy_count=0
post_terminal_compare_operand_count=0
hako_body_elapsed_ns=<n>
c_body_elapsed_ns=<n>
body_elapsed_ratio=<ratio>
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

## Result

```text
output_contract=hako-mimalloc-post-local-ssa-call-result-fallback-copy-policy-measurement-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-681
post_candidate_result_copy_count=0
post_terminal_compare_operand_count=0
hako_body_elapsed_ns=367000000
c_body_elapsed_ns=4132840
body_elapsed_ratio=88.801
body_elapsed_gap_ns=362867160
copy_count=55
expression_materialization_copy_count=3
dominant_copy_owner=local_ssa_copy_materialization
dominant_dynamic_owner=local_ssa_copy_materialization
dominant_position=call_adjacent
dominant_route_carrier_role=call_operand
page_hotpath_helpers_call_count=5
page_hotpath_helpers_attributed_copy_count=8
result_materialization_copy_count=7
winner_claim=0
selected_next_owner=post_keeper_owner_unclear
selected_owner_confidence=low
next_task=post_local_ssa_call_result_fallback_copy_policy_owner_refresh_repeat
optimization_open=0
summary=ok
```

Interpretation:

```text
The keeper removed the selected MIR family, but the product-route body timing
gap remains large:

  previous body_elapsed_ratio=92.800
  current  body_elapsed_ratio=88.801

The current owner refresh is not strong enough for another implementation row:

  selected_next_owner=post_keeper_owner_unclear
  selected_owner_confidence=low

Stop implementation here and repeat owner selection before touching code again.
```

## Acceptance

```text
post_local_ssa_call_result_fallback_copy_policy_measurement_landed=1
source_evidence=296x-681
measurement_run=1
winner_claim=0
selected_next_owner=post_keeper_owner_unclear
optimization_open=0
summary=ok
```
