---
Status: Landed
Date: 2026-06-15
Task: CALL-OPERAND-ROUTE-CARRIER-REVALIDATION-GUARD-SURFACE-001
Scope: Define the guard surface for revalidating residual call-operand
  route-carrier copies against the previously closed CFG-stable receiver
  rewrite family.
Related:
  - docs/development/current/main/phases/phase-296x/296x-759-CALL-OPERAND-ROUTE-CARRIER-POLICY-SELECTION-001.md
  - docs/development/current/main/phases/phase-296x/296x-758-ROUTE-CARRIER-RESIDUAL-INVENTORY-001.md
  - docs/development/current/main/phases/phase-296x/296x-696-CALL-OPERAND-CFG-STABLE-RECEIVER-REWRITE-IMPLEMENTATION-001.md
  - docs/development/current/main/phases/phase-296x/296x-699-MIMALLOC-BODY-TIMING-CFG-STABLE-RECEIVER-REWRITE-CLOSEOUT-001.md
  - tools/allocator/mir_local_ssa_copy_position_probe.py
---

# CALL-OPERAND-ROUTE-CARRIER-REVALIDATION-GUARD-SURFACE-001

## Result

```text
output_contract=hako-mimalloc-call-operand-route-carrier-revalidation-guard-surface-v0
source_evidence=296x-759,296x-758,296x-696,296x-699
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
route_carrier_residual_copy_count=13
call_operand_route_carrier_copy_count=13
compare_operand_route_carrier_copy_count=0
prior_keeper=cfg_stable_dominance_guarded_receiver_operand_rewrite
prior_keeper_owner=mir_passes_callsite_canonicalize_receiver_operand_rewrite
prior_keeper_closed=1
prior_keeper_post_selected_keeper_candidate_count=0
current_probe_gap=call_operand_role_not_split_between_receiver_and_args
required_probe_field=call_operand_receiver_route_carrier_copy_count
required_probe_field=call_operand_arg_route_carrier_copy_count
receiver_post_target=0
arg_forwarding_enabled=0
arg_forwarding_policy=closed_until_explicit_arg_owner_selection
selected_next_action=call_operand_route_carrier_receiver_arg_split_probe
implementation_allowed=0
measurement_required=0
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

The current residual inventory reports only a coarse call-operand role:

```text
call_operand_route_carrier_copy_count=13
```

That is insufficient for implementation. The prior keeper was receiver-only and
already closed:

```text
prior_keeper=cfg_stable_dominance_guarded_receiver_operand_rewrite
prior_keeper_post_selected_keeper_candidate_count=0
```

Before any code change, the current route-carrier probe must split call
operands into receiver and arg surfaces:

```text
call_operand_receiver_route_carrier_copy_count=<n>
call_operand_arg_route_carrier_copy_count=<n>
```

Only receiver residue may re-enter the previous CFG-stable receiver owner. Arg
residue remains closed until a separate arg-owner selection row exists.

## Required Probe Shape

```text
input:
  current MIR JSON for object_lifecycle_body
  target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1

required output:
  call_operand_receiver_route_carrier_copy_count
  call_operand_arg_route_carrier_copy_count
  call_operand_receiver_route_carrier_sample_count
  call_operand_arg_route_carrier_sample_count

acceptance:
  receiver_post_target=0
  arg_forwarding_enabled=0
  implementation_allowed=0
```

## Stop Line

```text
do not implement from this guard-surface row
do not patch LocalSSA::ensure_fallback_copy
do not retry the CFG-stable receiver rewrite without receiver/arg split evidence
do not open arg forwarding
do not special-case source names, helper names, or benchmark names
do not change PHI lifecycle or freshness contracts
do not change source .hako, product defaults, provider activation,
replacement, hooks, or global allocator
```

## Next

```text
CALL-OPERAND-ROUTE-CARRIER-RECEIVER-ARG-SPLIT-PROBE-001:
  extend or wrap the current copy-position probe so call_operand route-carrier
  residue is split into receiver and arg surfaces before any implementation
  owner is selected
```
