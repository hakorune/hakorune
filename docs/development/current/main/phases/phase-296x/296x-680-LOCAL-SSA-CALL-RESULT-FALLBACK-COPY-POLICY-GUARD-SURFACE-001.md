---
Status: Active
Date: 2026-06-15
Task: LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-GUARD-SURFACE-001
Scope: Define the post-implementation guard surface for the narrow LocalSSA
  same-block call-result root policy before implementation.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-679-LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-DESIGN-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-GUARD-SURFACE-001

## Purpose

296x-679 selected the next narrow policy:

```text
selected_policy_shape=same_block_call_result_root_for_compare_operand
selected_policy_owner=LocalSSA::ensure_fallback_copy
allowed_use_kind=CompareOperand
arg_forwarding_enabled=0
helper_name_special_case=0
post_candidate_result_copy_count_upper_bound=4
```

This row fixes the acceptance surface before implementation. The implementation
row must be measured by MIR shape first, not by source assumptions or broad copy
coalescing.

## Guard Surface

```text
pre_candidate_result_copy_count=14
pre_terminal_compare_operand_count=4

post_terminal_compare_operand_target=0
post_candidate_result_copy_count_upper_bound=4

allowed_use_kind=CompareOperand
arg_forwarding_enabled=0
receiver_forwarding_changed=0
helper_name_special_case=0
variable_map_semantics_changed=0
phi_lifecycle_changed=0
generic_copy_coalescing_enabled=0
```

## Stop Line

```text
do not implement in this row
do not require full 14-copy removal for first keeper
do not broaden LocalSSA copy coalescing
do not forward arbitrary Call results
do not enable Arg forwarding
do not patch source .hako
do not claim a performance win
```

## Required Output

```text
output_contract=hako-mimalloc-local-ssa-call-result-fallback-copy-policy-guard-surface-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-679
pre_candidate_result_copy_count=14
pre_terminal_compare_operand_count=4
post_terminal_compare_operand_target=0
post_candidate_result_copy_count_upper_bound=4
allowed_use_kind=CompareOperand
arg_forwarding_enabled=0
helper_name_special_case=0
variable_map_semantics_changed=0
phi_lifecycle_changed=0
implementation_started=0
optimization_open=0
winner_claim=0
summary=ok
```

## Acceptance

```text
local_ssa_call_result_fallback_copy_policy_guard_surface_active=1
source_evidence=296x-679
post_target_defined=0
implementation_started=0
optimization_open=0
winner_claim=0
summary=pending
```
