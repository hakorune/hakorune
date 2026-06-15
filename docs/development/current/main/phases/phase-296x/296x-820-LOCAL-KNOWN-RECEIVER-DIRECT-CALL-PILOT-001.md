---
Status: Landed
Date: 2026-06-16
Task: LOCAL-KNOWN-RECEIVER-DIRECT-CALL-PILOT-001
Scope: Classify the local known-receiver direct-call pilot against the existing
  generic RoutePlan backend seam.
Related:
  - docs/development/current/main/phases/phase-296x/296x-819-LOCAL-KNOWN-RECEIVER-DIRECT-CALL-SHADOW-001.md
  - tools/allocator/hako_local_known_receiver_direct_call_pilot.py
---

# LOCAL-KNOWN-RECEIVER-DIRECT-CALL-PILOT-001

## Purpose

The shadow row found three guarded page receiver direct-call candidates.  The
implementation row must not add a page-specific or method-name-specific
lowering path.  This row therefore checks whether the existing backend already
has the generic seam required by the design:

```text
ObjectPlan pre-publication shadow
  + RoutePlan user_box_method_routes backend proof
  -> direct same-module target call
```

## Decision

The generic seam already exists in the measured `ny-llvmc` C ABI route:

```text
emit_user_box_method_lowering_plan_mir_call
  reads source=user_box_method_routes
  validates same-module user-box method target
  emits call i64 @"<target_symbol>"
```

So this row does not add new lowering code.  The pilot is classified as already
satisfied by the existing generic RoutePlan backend route.  Measurement still
has to run because this row only proves reachability, not a body-time win.

## Result

```text
output_contract=hako-local-known-receiver-direct-call-pilot-v0
source_evidence=296x-819,296x-818,296x-817
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
selected_shape=local_known_receiver_direct_call
pilot_status=already_satisfied_existing_generic_route
first_target_receiver=page
first_target_call_count=3
first_target_methods=acquire_usize,reuse
generic_routeplan_backend_seam_ready=1
c_shim_user_box_method_route_consumer=1
c_shim_reads_user_box_method_routes=1
c_shim_emits_target_symbol_call=1
c_shim_trace_consumer_present=1
routeplan_direct_target_predicate_present=1
routeplan_same_module_definition_required=1
objectplan_pre_publication_shadow_used=1
routeplan_backend_consumable_proof_used=1
new_backend_lowering_code_added=0
page_specific_rule_enabled=0
method_name_special_case_enabled=0
helper_symbol_inference_enabled=0
storage_direct_enabled=0
hosthandle_bypass_enabled=0
arc_retirement_enabled=0
product_default_changed=0
measurement_required=1
next_task=LOCAL-KNOWN-RECEIVER-DIRECT-CALL-MEASUREMENT-001
summary=ok
```

## Stop Line

```text
do not add a page receiver-name branch
do not special-case acquire_usize or reuse
do not infer direct calls from helper symbols
do not open storage direct lowering
do not bypass HostHandle
do not retire Arc
do not change product default runtime behavior
do not claim a performance win from reachability alone
```

## Proof

```bash
python3 -m py_compile tools/allocator/hako_local_known_receiver_direct_call_pilot.py
bash tools/checks/k2_wide_phase296x_local_known_receiver_direct_call_pilot_guard.sh
```
