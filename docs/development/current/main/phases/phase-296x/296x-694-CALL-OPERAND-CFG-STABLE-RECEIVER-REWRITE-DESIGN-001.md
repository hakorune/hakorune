---
Status: Active
Date: 2026-06-15
Task: CALL-OPERAND-CFG-STABLE-RECEIVER-REWRITE-DESIGN-001
Scope: Design the CFG-stable owner for dominance-required receiver operand
  Copy-chain rewriting after LocalSSA emission-time implementation was rejected.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-691-CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-692-CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-GUARD-SURFACE-001.md
  - docs/development/current/main/phases/phase-296x/296x-693-CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-IMPLEMENTATION-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# CALL-OPERAND-CFG-STABLE-RECEIVER-REWRITE-DESIGN-001

## Purpose

296x-693 rejected LocalSSA emission-time implementation:

```text
best_localssa_emission_time_post_selected_keeper_candidate_count=1
observed_cfg_dominance_helper_post_selected_keeper_candidate_count=12
rejected_reason=dominance_required_candidates_need_cfg_stable_rewrite_owner
```

This row designs the correct owner before another implementation attempt.

## Required Output

```text
output_contract=hako-mimalloc-call-operand-cfg-stable-receiver-rewrite-design-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-693
selected_owner=<owner>
selected_owner_reason=<reason>
selected_keeper_shape=cfg_stable_dominance_guarded_receiver_operand_rewrite
pre_selected_keeper_candidate_count=13
post_selected_keeper_candidate_count_target=0
arg_forwarding_enabled=0
requires_cfg_stable_dominance_guard=1
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

## Design Constraints

```text
allowed:
  - receiver operand rewrite after CFG / terminator successors are stable
  - dominance proof from final MIR function structure
  - removal of now-dead receiver carrier Copy values only if a follow-up guard
    proves the shape

forbidden:
  - LocalSSA emission-time dominance decisions
  - Arg operand forwarding
  - unknown-root forwarding
  - helper-name special cases
  - variable_map semantic changes
  - PHI lifecycle changes
  - source .hako changes
  - startup / boot optimization changes
```

## Acceptance

```text
call_operand_cfg_stable_receiver_rewrite_design_active=1
source_evidence=296x-693
selected_owner=0
implementation_started=0
optimization_open=0
winner_claim=0
summary=pending
```
