---
Status: Landed
Date: 2026-06-15
Task: LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-IMPLEMENTATION-001
Scope: Implement the narrow LocalSSA same-block Call-root CompareOperand policy
  selected by 296x-679 and guarded by 296x-680.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-296x/296x-680-LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-GUARD-SURFACE-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# LOCAL-SSA-CALL-RESULT-FALLBACK-COPY-POLICY-IMPLEMENTATION-001

## Purpose

Implement only the selected 296x-679 policy:

```text
selected_policy_shape=same_block_call_result_root_for_compare_operand
selected_policy_owner=LocalSSA::ensure_fallback_copy
allowed_use_kind=CompareOperand
arg_forwarding_enabled=0
helper_name_special_case=0
```

## Implementation Boundary

```text
allowed:
  LocalSSA::ensure fallback policy in src/mir/builder/ssa/local.rs
  same-block Copy chain root detection
  CompareOperand-only forwarding to same-block Call root
  MIR shape post probe

forbidden:
  Arg forwarding
  receiver forwarding changes
  arbitrary Call-result forwarding
  helper-name special casing
  variable_map changes
  PHI / phi_lifecycle changes
  source .hako changes
  generic DCE / copy coalescing
  winner claim before body timing remeasurement
```

## Required Output

```text
output_contract=hako-mimalloc-local-ssa-call-result-fallback-copy-policy-implementation-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-680
pre_terminal_compare_operand_count=4
post_terminal_compare_operand_count=0
post_candidate_result_copy_count<=4
allowed_use_kind=CompareOperand
arg_forwarding_enabled=0
helper_name_special_case=0
variable_map_semantics_changed=0
phi_lifecycle_changed=0
implementation_started=1
optimization_open=0
winner_claim=0
summary=ok
```

## Result

```text
output_contract=hako-mimalloc-local-ssa-call-result-fallback-copy-policy-implementation-v0
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
source_evidence=296x-680
pre_candidate_result_copy_count=14
pre_terminal_compare_operand_count=4
post_terminal_compare_operand_count=0
post_candidate_result_copy_count=0
post_candidate_result_copy_count_upper_bound=4
allowed_use_kind=CompareOperand
arg_forwarding_enabled=0
helper_name_special_case=0
variable_map_semantics_changed=0
phi_lifecycle_changed=0
implementation_started=1
optimization_open=0
winner_claim=0
post_helper_selectSinglePageFastPath_candidate_count=0
post_helper_selectPage_candidate_count=0
post_helper_acquire_usize_candidate_count=0
post_helper_reuse_candidate_count=0
summary=ok
```

Implementation note:

```text
The keeper is generic over MIR shape, not helper names:
  CompareOperand only
  same-block Copy chain rooted at same-block Call
  return the Call root ValueId

It does not change Arg forwarding, receiver forwarding, variable_map, PHI
lifecycle, helper lowering, source .hako, or provider activation.
```

## Stop Line

```text
do not measure or claim winner in this row
do not broaden LocalSSA copy coalescing
do not forward arbitrary Call results
do not enable Arg forwarding
do not change helper lowering or source
```

## Acceptance

```text
local_ssa_call_result_fallback_copy_policy_implementation_landed=1
source_evidence=296x-680
implementation_started=1
post_probe_run=1
winner_claim=0
summary=ok
```
