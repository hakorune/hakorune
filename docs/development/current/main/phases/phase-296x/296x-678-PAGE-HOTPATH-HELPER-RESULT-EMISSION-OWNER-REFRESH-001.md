---
Status: Active
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
local_ssa_owner_rejected=1
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

## Acceptance

```text
page_hotpath_helper_result_emission_owner_refresh_active=1
source_evidence=296x-677
owner_refresh_run=0
selected_next_owner=0
implementation_started=0
optimization_open=0
winner_claim=0
summary=pending
```
