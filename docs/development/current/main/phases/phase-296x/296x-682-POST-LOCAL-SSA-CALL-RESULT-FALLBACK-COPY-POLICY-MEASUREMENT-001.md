---
Status: Active
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

## Acceptance

```text
post_local_ssa_call_result_fallback_copy_policy_measurement_active=1
source_evidence=296x-681
measurement_run=0
winner_claim=0
selected_next_owner=0
optimization_open=0
summary=pending
```
