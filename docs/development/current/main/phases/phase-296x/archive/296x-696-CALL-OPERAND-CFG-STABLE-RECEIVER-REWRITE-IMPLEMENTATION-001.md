---
Status: Landed
Date: 2026-06-15
Task: CALL-OPERAND-CFG-STABLE-RECEIVER-REWRITE-IMPLEMENTATION-001
Scope: Implement the CFG-stable dominance guarded receiver operand rewrite in
  the callsite canonicalization pass family.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-694-CALL-OPERAND-CFG-STABLE-RECEIVER-REWRITE-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-695-CALL-OPERAND-CFG-STABLE-RECEIVER-REWRITE-GUARD-SURFACE-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# CALL-OPERAND-CFG-STABLE-RECEIVER-REWRITE-IMPLEMENTATION-001

## Purpose

296x-695 fixed the implementation target:

```text
selected_owner=mir_passes_callsite_canonicalize_receiver_operand_rewrite
selected_keeper_shape=cfg_stable_dominance_guarded_receiver_operand_rewrite
post_selected_keeper_candidate_count_target=0
post_call_operand_unique_copy_count_upper_bound=14
```

This row may implement only that narrow receiver operand rewrite.

## Required Output

```text
output_contract=hako-mimalloc-call-operand-cfg-stable-receiver-rewrite-implementation-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-695
selected_owner=mir_passes_callsite_canonicalize_receiver_operand_rewrite
selected_keeper_shape=cfg_stable_dominance_guarded_receiver_operand_rewrite
pre_selected_keeper_candidate_count=13
post_selected_keeper_candidate_count=0
post_call_operand_unique_copy_count<=14
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
optimization_open=0
winner_claim=0
summary=ok
```

## Implementation Constraints

```text
allowed:
  - add a receiver-operand subpass under src/mir/passes/callsite_canonicalize/
  - use final MIR CFG successors for dominance
  - rewrite Method receiver operand from carrier Copy to known dominating root

forbidden:
  - LocalSSA emission-time dominance decisions
  - Arg operand forwarding
  - unknown-root forwarding
  - helper-name special cases
  - variable_map semantic changes
  - PHI lifecycle changes
  - source .hako changes
  - startup / boot optimization changes
  - winner claim before body timing remeasurement
```

## Acceptance

```text
call_operand_cfg_stable_receiver_rewrite_implementation_landed=1
source_evidence=296x-695
post_selected_keeper_candidate_count_target=0
implementation_started=1
optimization_open=0
winner_claim=0
summary=ok
```

## Result

```text
output_contract=hako-mimalloc-call-operand-cfg-stable-receiver-rewrite-implementation-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-695
selected_owner=mir_passes_callsite_canonicalize_receiver_operand_rewrite
selected_keeper_shape=cfg_stable_dominance_guarded_receiver_operand_rewrite
pre_selected_keeper_candidate_count=13
post_selected_keeper_candidate_count=0
post_call_operand_unique_copy_count=13
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
optimization_open=0
winner_claim=0
next_task=post_cfg_stable_receiver_rewrite_measurement
summary=ok
```

Interpretation:

```text
The CFG-stable receiver rewrite removed the selected receiver family:
safe_receiver_candidate_count=0. The remaining dominance-required call operand
candidate is the explicitly rejected Arg surface, so it stays closed.
```
