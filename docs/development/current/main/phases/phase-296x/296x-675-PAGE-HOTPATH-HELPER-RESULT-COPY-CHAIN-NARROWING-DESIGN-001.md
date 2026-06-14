---
Status: Active
Date: 2026-06-15
Task: PAGE-HOTPATH-HELPER-RESULT-COPY-CHAIN-NARROWING-DESIGN-001
Scope: Design the narrow page-hotpath helper result copy-chain narrowing rule
  before implementation.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-674-PAGE-HOTPATH-HELPER-RESULT-MATERIALIZATION-INVENTORY-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# PAGE-HOTPATH-HELPER-RESULT-COPY-CHAIN-NARROWING-DESIGN-001

## Purpose

296x-674 selected the next owner:

```text
selected_owner=page_hotpath_helper_result_copy_chain_narrowing
page_hotpath_helpers_call_count=5
page_hotpath_helpers_attributed_copy_count=22
page_hotpath_helper_result_copy_count=14
result_materialization_copy_count=14
dominant_helper=acquire_usize
dominant_result_chain_shape=call_result_copy_chain_len_1
dominant_result_sink=copy_only
```

This row designs the safe keeper boundary for page-hotpath helper call-result
copy chains. It must not change code until the rule is pinned.

## Candidate Family

```text
source:
  page-hotpath helper call result

helpers:
  acquire_usize
  selectSinglePageFastPath
  reuse

observed result descendants:
  total=14
  acquire_usize=8
  selectSinglePageFastPath=3
  reuse=3

sinks:
  copy_only=10
  compare_lt=2
  compare_ne=1
  compare_eq=1

chain lengths:
  len_1=4
  len_2=4
  len_3=4
  len_4=2
```

## Design Questions

```text
1. Which call-result copies are pure route-carrier materialization?
2. Which copies are required to preserve value identity / debug shape / phi input?
3. Can compare operands consume the call result or nearest existing copy directly?
4. Can call operands consume the call result or nearest existing copy directly?
5. Does any candidate cross a side-effect, block boundary, or mutation boundary?
6. Is the rule LocalSSA-owned, helper-lowering-owned, or a smaller value-use
   rewrite owner?
```

## Required Output

```text
output_contract=hako-mimalloc-page-hotpath-helper-result-copy-chain-narrowing-design-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-674
candidate_result_copy_count=14
safe_candidate_count=<n>
unsafe_candidate_count=<n>
dominant_safe_shape=<shape>
selected_keeper_shape=<shape>
selected_keeper_owner=<owner>
selected_owner_confidence=<low|medium|high>
next_task=<task>
implementation_started=0
optimization_open=0
winner_claim=0
summary=ok
```

## Stop Line

```text
do not change helper lowering in this row
do not broaden LocalSSA copy coalescing
do not forward arbitrary call results
do not assume helper purity from the helper name alone
do not reopen field_get alias forwarding
do not touch allocator provider activation
do not claim a performance win
```

## Acceptance

```text
page_hotpath_helper_result_copy_chain_narrowing_design_active=1
source_evidence=296x-674
design_probe_run=0
selected_keeper_shape=0
implementation_started=0
optimization_open=0
winner_claim=0
summary=pending
```
