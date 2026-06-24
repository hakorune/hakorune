---
Status: Landed
Date: 2026-06-15
Task: CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-IMPLEMENTATION-001
Scope: Implement only dominance-guarded receiver operand forwarding for the
  residual call-operand Copy chains.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-691-CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-692-CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-GUARD-SURFACE-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-IMPLEMENTATION-001

## Purpose

296x-692 fixed the post target:

```text
selected_keeper_shape=dominance_guarded_receiver_operand_forwarding
pre_selected_keeper_candidate_count=13
post_selected_keeper_candidate_count_target=0
post_call_operand_unique_copy_count_upper_bound=14
arg_forwarding_enabled=0
requires_dominance_guard=1
```

This row attempted to implement only that narrow keeper, but the LocalSSA
emission-time seam was rejected before commit.

## Required Output

```text
output_contract=hako-mimalloc-call-operand-dominance-required-forwarding-implementation-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-692
selected_keeper_shape=dominance_guarded_receiver_operand_forwarding
pre_selected_keeper_candidate_count=13
post_selected_keeper_candidate_count=0
post_call_operand_unique_copy_count<=14
arg_forwarding_enabled=0
requires_dominance_guard=1
helper_name_special_case=0
variable_map_semantics_changed=0
phi_lifecycle_changed=0
source_hako_changed=0
startup_lane_reopened=0
optimization_open=0
winner_claim=0
summary=ok
```

## Implementation Contract

```text
allowed:
  - LocalSSA receiver operand forwarding when the root definition dominates
    the call block
  - same method / same MIR function evidence only
  - explicit dominance guard

forbidden:
  - Arg operand forwarding
  - unknown-root forwarding
  - helper-name special cases
  - variable_map semantic changes
  - PHI lifecycle changes
  - source .hako changes
  - startup / boot optimization changes
```

## Stop Line

```text
stop if the implementation needs Arg forwarding
stop if any candidate requires root visibility without dominance proof
stop if the seam requires helper-name matching
stop if variable_map or PHI lifecycle must change
stop if body timing is used as a winner claim before MIR post-shape proof
```

## Result

```text
output_contract=hako-mimalloc-call-operand-dominance-required-forwarding-implementation-rejected-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-692
selected_keeper_shape=dominance_guarded_receiver_operand_forwarding
localssa_emission_time_attempted=1
localssa_emission_time_committed=0
post_selected_keeper_candidate_count_target=0
best_localssa_emission_time_post_selected_keeper_candidate_count=1
observed_cfg_dominance_helper_post_selected_keeper_candidate_count=12
rejected_reason=dominance_required_candidates_need_cfg_stable_rewrite_owner
arg_forwarding_enabled=0
requires_dominance_guard=1
helper_name_special_case=0
variable_map_semantics_changed=0
phi_lifecycle_changed=0
source_hako_changed=0
startup_lane_reopened=0
optimization_open=0
winner_claim=0
next_task=call_operand_cfg_stable_receiver_rewrite_design
summary=ok
```

Interpretation:

```text
The selected family is real, but LocalSSA emission time is the wrong owner for
the dominance-required part. The final MIR evidence proves dominance after the
CFG is stable; attempting to decide it during receiver materialization either
leaves one candidate or perturbs previously removed candidates. The next row
must design a CFG-stable, receiver-only rewrite owner before any code changes.
```

## Acceptance

```text
call_operand_dominance_required_forwarding_implementation_rejected=1
source_evidence=296x-692
localssa_emission_time_committed=0
post_selected_keeper_candidate_count_target=0
best_localssa_emission_time_post_selected_keeper_candidate_count=1
optimization_open=0
winner_claim=0
summary=ok
```
