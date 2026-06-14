---
Status: Landed
Date: 2026-06-15
Task: CALL-OPERAND-CFG-STABLE-RECEIVER-REWRITE-GUARD-SURFACE-001
Scope: Fix the post-implementation guard surface for CFG-stable dominance
  guarded receiver operand rewrite before code changes.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-694-CALL-OPERAND-CFG-STABLE-RECEIVER-REWRITE-DESIGN-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# CALL-OPERAND-CFG-STABLE-RECEIVER-REWRITE-GUARD-SURFACE-001

## Purpose

296x-694 selected:

```text
selected_owner=mir_passes_callsite_canonicalize_receiver_operand_rewrite
selected_keeper_shape=cfg_stable_dominance_guarded_receiver_operand_rewrite
pre_selected_keeper_candidate_count=13
requires_cfg_stable_dominance_guard=1
```

This row fixes the post-implementation guard surface before source changes.

## Required Output

```text
output_contract=hako-mimalloc-call-operand-cfg-stable-receiver-rewrite-guard-surface-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-694
selected_owner=mir_passes_callsite_canonicalize_receiver_operand_rewrite
selected_keeper_shape=cfg_stable_dominance_guarded_receiver_operand_rewrite
pre_selected_keeper_candidate_count=13
post_selected_keeper_candidate_count_target=0
post_call_operand_unique_copy_count_upper_bound=14
arg_forwarding_enabled=0
requires_cfg_stable_dominance_guard=1
dominance_source=final_mir_cfg_successors
receiver_only_rewrite=1
unknown_root_forwarding_enabled=0
helper_name_special_case=0
variable_map_semantics_changed=0
phi_lifecycle_changed=0
source_hako_changed=0
startup_lane_reopened=0
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
do not put cross-block dominance in LocalSSA emission
do not enable Arg forwarding
do not forward unknown-root chains
do not special-case helper names
do not change variable_map semantics
do not change PHI lifecycle
do not claim a winner
```

## Acceptance

```text
call_operand_cfg_stable_receiver_rewrite_guard_surface_landed=1
source_evidence=296x-694
guard_surface_fixed=1
implementation_started=0
optimization_open=0
winner_claim=0
summary=ok
```

## Result

```text
output_contract=hako-mimalloc-call-operand-cfg-stable-receiver-rewrite-guard-surface-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-694
selected_owner=mir_passes_callsite_canonicalize_receiver_operand_rewrite
selected_keeper_shape=cfg_stable_dominance_guarded_receiver_operand_rewrite
pre_selected_keeper_candidate_count=13
post_selected_keeper_candidate_count_target=0
post_call_operand_unique_copy_count_upper_bound=14
arg_forwarding_enabled=0
requires_cfg_stable_dominance_guard=1
dominance_source=final_mir_cfg_successors
receiver_only_rewrite=1
unknown_root_forwarding_enabled=0
helper_name_special_case=0
variable_map_semantics_changed=0
phi_lifecycle_changed=0
source_hako_changed=0
startup_lane_reopened=0
implementation_started=0
optimization_open=0
winner_claim=0
next_task=call_operand_cfg_stable_receiver_rewrite_implementation
summary=ok
```
