---
Status: Landed
Date: 2026-06-15
Task: PAGE-HOTPATH-HELPER-RESULT-COPY-CHAIN-NARROWING-GUARD-SURFACE-001
Scope: Define the guard surface for the narrow page-hotpath helper result
  copy-chain keeper before implementation.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-675-PAGE-HOTPATH-HELPER-RESULT-COPY-CHAIN-NARROWING-DESIGN-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# PAGE-HOTPATH-HELPER-RESULT-COPY-CHAIN-NARROWING-GUARD-SURFACE-001

## Purpose

296x-675 selected:

```text
selected_keeper_shape=same_block_call_result_terminal_consumer_rewrite
selected_keeper_owner=LocalSSA::ensure_call_result_alias_to_consumer
candidate_result_copy_count=14
terminal_consumer_rewrite_candidate_count=4
dependent_dead_copy_candidate_count=10
unsafe_candidate_count=0
```

This row defines the pre-implementation guard surface for that narrow keeper.

## Result

```text
output_contract=hako-mimalloc-page-hotpath-helper-result-copy-chain-guard-surface-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-675
pre_candidate_result_copy_count=14
pre_terminal_consumer_rewrite_candidate_count=4
pre_unsafe_candidate_count=0
post_terminal_consumer_target=0
post_candidate_result_copy_count_upper_bound=10
selected_keeper_owner=LocalSSA::ensure_call_result_alias_to_consumer
next_task=page_hotpath_helper_result_copy_chain_narrowing_implementation
implementation_started=0
optimization_open=0
winner_claim=0
summary=ok
```

The first keeper should be judged by the terminal consumer rewrite target. Full
14-copy removal is not required for the first keeper because 10 candidates are
internal copy-only chain members that depend on later dead-copy cleanup.

## Guard Shape

```text
pre:
  candidate_result_copy_count=14
  terminal_consumer_rewrite_candidate_count=4
  unsafe_candidate_count=0

post target:
  terminal_consumer_rewrite_candidate_count_after=0
  candidate_result_copy_count_after <= 10
  arbitrary_call_result_forwarding_count=0
  broad_local_ssa_coalescing_count=0
```

The post target allows the internal copy-only chain to remain if dead-copy
cleanup does not remove it. The first implementation must prove terminal
consumer rewrite before claiming total chain removal.

## Required Output

```text
output_contract=hako-mimalloc-page-hotpath-helper-result-copy-chain-guard-surface-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-675
pre_candidate_result_copy_count=14
pre_terminal_consumer_rewrite_candidate_count=4
pre_unsafe_candidate_count=0
post_terminal_consumer_target=0
post_candidate_result_copy_count_upper_bound=10
selected_keeper_owner=LocalSSA::ensure_call_result_alias_to_consumer
next_task=page_hotpath_helper_result_copy_chain_narrowing_implementation
implementation_started=0
optimization_open=0
winner_claim=0
summary=ok
```

## Stop Line

```text
do not implement in this row
do not broaden LocalSSA copy coalescing
do not forward arbitrary call results
do not require full 14-copy removal for first keeper
do not claim a performance win
```

## Acceptance

```text
page_hotpath_helper_result_copy_chain_guard_surface_active=1
source_evidence=296x-675
guard_surface_defined=1
implementation_started=0
optimization_open=0
winner_claim=0
summary=ok
```
