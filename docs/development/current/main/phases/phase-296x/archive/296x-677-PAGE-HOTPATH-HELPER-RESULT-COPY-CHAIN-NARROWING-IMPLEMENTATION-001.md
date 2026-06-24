---
Status: Landed
Date: 2026-06-15
Task: PAGE-HOTPATH-HELPER-RESULT-COPY-CHAIN-NARROWING-IMPLEMENTATION-001
Scope: Implement the narrow LocalSSA terminal consumer rewrite for same-block
  page-hotpath helper call-result copy chains.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-676-PAGE-HOTPATH-HELPER-RESULT-COPY-CHAIN-NARROWING-GUARD-SURFACE-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# PAGE-HOTPATH-HELPER-RESULT-COPY-CHAIN-NARROWING-IMPLEMENTATION-001

## Purpose

Implement the narrow keeper selected by 296x-675 and guarded by 296x-676:

```text
selected_keeper_shape=same_block_call_result_terminal_consumer_rewrite
selected_keeper_owner=LocalSSA::ensure_call_result_alias_to_consumer
pre_terminal_consumer_rewrite_candidate_count=4
post_terminal_consumer_target=0
post_candidate_result_copy_count_upper_bound=10
```

## Result

```text
output_contract=hako-mimalloc-page-hotpath-helper-result-copy-chain-implementation-attempt-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-676
trial_owner=LocalSSA::ensure_call_result_alias_to_consumer
trial_committed=0
post_terminal_consumer_rewrite_candidate_count=4
post_candidate_result_copy_count=14
local_ssa_trace_matched_candidate_chain=0
selected_keeper_owner_rejected=LocalSSA::ensure_call_result_alias_to_consumer
next_task=page_hotpath_helper_result_emission_owner_refresh
implementation_started=0
optimization_open=0
winner_claim=0
summary=ok
```

The LocalSSA trial was reverted before commit. It built successfully but did not
change the target MIR shape: terminal consumer candidates remained at 4 and
result-copy candidates remained at 14. A `NYASH_LOCAL_SSA_TRACE=1` run did not
show matching LocalSSA emission for the candidate ValueIds, so the selected
implementation owner is not proven.

Conclusion:

```text
do not keep a no-op LocalSSA patch
do not broaden LocalSSA to force a win
refresh the actual emission owner before implementation
```

## Implementation Boundary

```text
allowed:
  same-block Call-result copy-chain root detection
  terminal consumer LocalSSA rewrite for CompareOperand / Arg if proven safe
  post-keeper MIR shape probe

forbidden:
  arbitrary call-result forwarding
  broad LocalSSA copy coalescing
  helper-name purity assumptions
  helper lowering changes
  allocator provider activation
  winner claim before measurement
```

## Required Output

```text
output_contract=hako-mimalloc-page-hotpath-helper-result-copy-chain-keeper-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
pre_terminal_consumer_rewrite_candidate_count=4
post_terminal_consumer_rewrite_candidate_count=0
post_candidate_result_copy_count<=10
arbitrary_call_result_forwarding_count=0
broad_local_ssa_coalescing_count=0
implementation_started=1
optimization_open=0
winner_claim=0
summary=ok
```

## Stop Line

```text
do not measure as winner in this row
do not broaden beyond same-block helper call-result copy chains
do not forward call results into non-terminal copy-only chain members directly
do not change helper calls or effects
```

## Acceptance

```text
page_hotpath_helper_result_copy_chain_implementation_active=1
source_evidence=296x-676
implementation_started=0
post_probe_run=1
selected_keeper_owner_rejected=LocalSSA::ensure_call_result_alias_to_consumer
winner_claim=0
summary=ok
```
