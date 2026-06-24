---
Status: Landed
Date: 2026-06-15
Task: PAGE-HOTPATH-HELPER-RESULT-EMISSION-OWNER-REFRESH-001
Scope: Refresh the actual MIR emission owner for page-hotpath helper result
  copy chains after the LocalSSA trial proved nonkeeper.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-677-PAGE-HOTPATH-HELPER-RESULT-COPY-CHAIN-NARROWING-IMPLEMENTATION-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# PAGE-HOTPATH-HELPER-RESULT-EMISSION-OWNER-REFRESH-001

## Purpose

296x-677 rejected the first LocalSSA implementation owner:

```text
trial_owner=LocalSSA::ensure_call_result_alias_to_consumer
trial_committed=0
post_terminal_consumer_rewrite_candidate_count=4
post_candidate_result_copy_count=14
local_ssa_trace_matched_candidate_chain=0
selected_keeper_owner_rejected=LocalSSA::ensure_call_result_alias_to_consumer
```

This row identifies the actual owner that emits the helper-result copy chain and
terminal compare operands before another implementation attempt.

## Candidate Owners

```text
CoreEffectPlan::Copy emission:
  src/mir/builder/control_flow/plan/lowerer/effect_emission.rs

CoreEffectPlan::Compare emission:
  src/mir/builder/control_flow/plan/lowerer/effect_emission.rs

MIR builder local variable assignment:
  src/mir/builder/stmts/variable_stmt.rs

BlockScheduleBox / schedule rematerialization:
  src/mir/builder/schedule/block.rs

LocalSSA:
  rejected for the first trial unless a stronger trace proves otherwise
```

## Required Output

```text
output_contract=hako-mimalloc-page-hotpath-helper-result-emission-owner-refresh-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-677
candidate_result_copy_count=14
terminal_consumer_rewrite_candidate_count=4
local_ssa_terminal_rewrite_owner_rejected=1
dominant_emission_owner=<owner>
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
do not reapply the LocalSSA trial blindly
do not broaden copy coalescing
do not patch source .hako
do not change helper lowering
do not claim a performance win
```

## Result

```text
output_contract=hako-mimalloc-page-hotpath-helper-result-emission-owner-refresh-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-677
candidate_result_copy_count=14
terminal_consumer_rewrite_candidate_count=4
local_ssa_terminal_rewrite_owner_rejected=1
first_hop_call_result_copy_count=4
chain_internal_copy_count=10
terminal_compare_operand_count=4
terminal_compare_first_hop_root_count=4
dominant_emission_owner=LocalSSA::ensure_fallback_copy
selected_next_owner=local_ssa_call_result_fallback_copy_policy
selected_owner_confidence=medium
next_task=local_ssa_call_result_fallback_copy_policy_design
implementation_started=0
optimization_open=0
winner_claim=0
helper_acquire_usize_candidate_count=8
helper_selectSinglePageFastPath_candidate_count=3
helper_reuse_candidate_count=3
terminal_helper_acquire_usize_count=2
terminal_helper_selectSinglePageFastPath_count=1
terminal_helper_reuse_count=1
summary=ok
```

Interpretation:

```text
296x-677 rejected the proposed terminal-consumer rewrite seam, not the whole
LocalSSA materialization owner. The current MIR shape shows the target family
as LocalSSA fallback Copy materialization:

  first-hop helper call-result copies = 4
  chain-internal copies               = 10
  terminal compare operands           = 4

The next row must design the fallback Copy policy directly. Do not revive the
same terminal-consumer rewrite patch without new evidence, and do not broaden
this into arbitrary copy coalescing.
```

## Acceptance

```text
page_hotpath_helper_result_emission_owner_refresh_landed=1
source_evidence=296x-677
owner_refresh_run=1
selected_next_owner=local_ssa_call_result_fallback_copy_policy
implementation_started=0
optimization_open=0
winner_claim=0
summary=ok
```
