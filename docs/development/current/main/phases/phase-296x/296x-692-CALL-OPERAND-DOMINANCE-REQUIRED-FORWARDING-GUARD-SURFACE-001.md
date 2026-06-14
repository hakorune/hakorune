---
Status: Landed
Date: 2026-06-15
Task: CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-GUARD-SURFACE-001
Scope: Fix the post-implementation guard surface for dominance-guarded
  receiver operand forwarding.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-691-CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-DESIGN-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-GUARD-SURFACE-001

## Purpose

296x-691 selected:

```text
selected_keeper_shape=dominance_guarded_receiver_operand_forwarding
selected_keeper_candidate_count=13
rejected_arg_forwarding_count=1
requires_dominance_guard=1
```

This row fixes the post-implementation guard surface before code changes.

## Required Output

```text
output_contract=hako-mimalloc-call-operand-dominance-required-forwarding-guard-surface-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-691
selected_keeper_shape=dominance_guarded_receiver_operand_forwarding
pre_selected_keeper_candidate_count=13
post_selected_keeper_candidate_count_target=0
post_call_operand_unique_copy_count_upper_bound=14
arg_forwarding_enabled=0
requires_dominance_guard=1
helper_name_special_case=0
variable_map_semantics_changed=0
phi_lifecycle_changed=0
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
do not broaden LocalSSA copy coalescing
do not enable Arg forwarding
do not forward unknown-root chains
do not special-case helper names
do not change variable_map semantics
do not change PHI lifecycle
do not claim a winner
```

## Result

```text
output_contract=hako-mimalloc-call-operand-dominance-required-forwarding-guard-surface-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-691
selected_keeper_shape=dominance_guarded_receiver_operand_forwarding
pre_selected_keeper_candidate_count=13
post_selected_keeper_candidate_count_target=0
post_call_operand_unique_copy_count_upper_bound=14
arg_forwarding_enabled=0
requires_dominance_guard=1
helper_name_special_case=0
variable_map_semantics_changed=0
phi_lifecycle_changed=0
implementation_started=0
optimization_open=0
winner_claim=0
next_task=call_operand_dominance_required_forwarding_implementation
summary=ok
```

Interpretation:

```text
The implementation row may remove only the 13 dominance-safe receiver operand
chains. Arg forwarding, unknown-root chains, helper-name seams, variable_map,
and PHI lifecycle stay closed.
```

## Acceptance

```text
call_operand_dominance_required_forwarding_guard_surface_landed=1
source_evidence=296x-691
guard_surface_fixed=1
post_selected_keeper_candidate_count_target=0
implementation_started=0
optimization_open=0
winner_claim=0
summary=ok
```
