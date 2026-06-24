---
Status: Landed
Date: 2026-06-15
Task: CALL-OPERAND-ROUTE-CARRIER-POLICY-SELECTION-001
Scope: Select the safe policy family for the remaining call-operand
  route-carrier copies after route-carrier residual inventory, without
  reopening LocalSSA or PHI freshness implementation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-758-ROUTE-CARRIER-RESIDUAL-INVENTORY-001.md
  - docs/development/current/main/phases/phase-296x/296x-690-CALL-OPERAND-RESIDUAL-POLICY-SELECTION-001.md
  - docs/development/current/main/phases/phase-296x/296x-691-CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-693-CALL-OPERAND-DOMINANCE-REQUIRED-FORWARDING-IMPLEMENTATION-001.md
  - docs/development/current/main/phases/phase-296x/296x-694-CALL-OPERAND-CFG-STABLE-RECEIVER-REWRITE-DESIGN-001.md
---

# CALL-OPERAND-ROUTE-CARRIER-POLICY-SELECTION-001

## Result

```text
output_contract=hako-mimalloc-call-operand-route-carrier-policy-selection-v0
source_evidence=296x-758,296x-690,296x-691,296x-693,296x-694
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
route_carrier_residual_copy_count=13
call_operand_route_carrier_copy_count=13
compare_operand_route_carrier_copy_count=0
prior_receiver_keeper_shape=cfg_stable_dominance_guarded_receiver_operand_rewrite
prior_receiver_keeper_candidate_count=13
prior_localssa_emission_time_rejected=1
prior_cfg_stable_owner=mir_passes_callsite_canonicalize_receiver_operand_rewrite
arg_forwarding_enabled=0
unknown_root_forwarding_enabled=0
helper_name_special_case=0
benchmark_name_special_case=0
selected_policy_family=cfg_stable_call_operand_route_carrier_revalidation
selected_policy_candidate_count=13
selected_next_action=call_operand_route_carrier_revalidation_guard_surface
implementation_allowed=0
guard_surface_required=1
winner_claim=0
startup_lane_reopened=0
source_hako_changed=0
mirbuilder_object_management_enabled=0
product_default_changed=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

The residual route-carrier bucket is call-operand only, but the count matches
the previously selected receiver-only CFG-stable family:

```text
call_operand_route_carrier_copy_count=13
prior_receiver_keeper_candidate_count=13
prior_cfg_stable_owner=mir_passes_callsite_canonicalize_receiver_operand_rewrite
```

This is not evidence for another LocalSSA forwarding attempt. The previous
LocalSSA emission-time owner was rejected because the remaining candidates
needed final-CFG dominance:

```text
prior_localssa_emission_time_rejected=1
```

The next row must therefore revalidate the CFG-stable receiver rewrite against
the current route-carrier classification before any implementation can reopen.

## Boundaries

```text
allowed next:
  call_operand_route_carrier_revalidation_guard_surface

not allowed:
  LocalSSA emission-time forwarding retry
  broad call-argument forwarding
  unknown-root forwarding
  helper-name / benchmark-name special cases
  PHI lifecycle / freshness changes
  product runtime or provider activation changes
```

## Stop Line

```text
do not implement from this policy row
do not patch LocalSSA::ensure_fallback_copy
do not reopen arg forwarding
do not special-case source names, helper names, or benchmark names
do not treat Type ABI / hako_check as execution truth
do not change source .hako, product defaults, provider activation,
replacement, hooks, or global allocator
```

## Next

```text
CALL-OPERAND-ROUTE-CARRIER-REVALIDATION-GUARD-SURFACE-001:
  define the post-target and evidence required to prove whether the prior
  CFG-stable receiver rewrite owner still owns the current route-carrier
  residue
```
