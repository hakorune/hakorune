---
Status: Landed
Date: 2026-06-15
Task: POST-FIELD-GET-ALIAS-KEEPER-OWNER-REFRESH-001
Scope: Refresh the MIR/body owner after 296x-672 removed the selected
  field_get-origin forwarding candidate family.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-672-CROSS-BLOCK-FIELD-GET-ALIAS-FORWARDING-KEEPER-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# POST-FIELD-GET-ALIAS-KEEPER-OWNER-REFRESH-001

## Purpose

296x-672 removed the selected field_get-origin forwarding candidates but did
not materially close the body-time gap. The owner shape changed:

```text
forwarding_candidate_copy_count_after=0
copy_count=69
expression_materialization_copy_count=3
dominant_dynamic_owner=page_hotpath_helper_attribution
dominant_copy_owner=result_materialization
hako_body_elapsed_ns=364000000
c_body_elapsed_ns=3922424
body_elapsed_ratio=92.800
winner_claim=0
```

This row refreshes the next owner before any new code changes.

## Required First Step

Run a post-keeper owner refresh that does not assume
`local_ssa_copy_materialization` is still dominant.

Required output:

```text
output_contract=hako-mimalloc-post-field-get-alias-keeper-owner-refresh-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
dominant_copy_owner=<owner>
dominant_dynamic_owner=<owner>
selected_next_owner=<owner>
selected_owner_confidence=<low|medium|high>
next_task=<task>
optimization_open=0
winner_claim=0
summary=ok
```

## Result

```text
output_contract=hako-mimalloc-post-field-get-alias-keeper-owner-refresh-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
hako_body_elapsed_ns=364000000
c_body_elapsed_ns=3922424
body_elapsed_ratio=92.800
gap_owner=compiler_lowering
copy_count=69
expression_materialization_copy_count=3
dominant_copy_owner=result_materialization
dominant_dynamic_owner=page_hotpath_helper_attribution
dominant_position=call_adjacent
dominant_route_carrier_role=call_operand
page_hotpath_helpers_call_count=5
page_hotpath_helpers_attributed_copy_count=22
result_materialization_copy_count=21
selected_next_owner=page_hotpath_helper_result_materialization_copy_chain
selected_owner_confidence=medium
next_task=page_hotpath_helper_result_materialization_inventory
implementation_started=0
optimization_open=0
winner_claim=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_post_field_get_alias_keeper_owner_refresh_guard.sh
```

## Stop Line

```text
do not extend field_get alias keeper
do not reopen param forwarding
do not broaden LocalSSA coalescing
do not claim body timing winner from 296x-672
do not touch allocator provider activation
```

## Acceptance

```text
post_field_get_alias_keeper_owner_refresh_active=1
source_evidence=296x-672
owner_refresh_run=1
selected_next_owner=page_hotpath_helper_result_materialization_copy_chain
implementation_started=0
optimization_open=0
winner_claim=0
summary=ok
```
