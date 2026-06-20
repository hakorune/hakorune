# Trim Route Lowering Readiness Integration Inventory

Status: inventory
Scope: callsites that can consume `decide_trim_route_lowering_readiness`.

## Purpose

The readiness gate exists as a pure decision:

```text
decide_trim_route_lowering_readiness(CarrierInfo, condition_bindings)
```

This inventory identifies where the required inputs are currently available.

## Required Inputs

```text
CarrierInfo.trim_helper
CarrierInfo.promoted_body_locals
JoinInlineBoundary.condition_bindings
```

## Inventory

```text
TrimRouteInfo::to_carrier_info:
  has trim metadata
  produces CarrierInfo with trim_helper/promoted_body_locals
  does not have condition_bindings
  readiness_gate_allowed_here=0

ConditionPromoter::try_promote_condition:
  has trim promotion result
  creates CarrierInfo
  does not own final JoinInlineBoundary.condition_bindings
  readiness_gate_allowed_here=0

InlineBoundaryBuilder:
  owns condition_bindings
  owns boundary construction
  can also receive/attach CarrierInfo
  readiness_gate_candidate=1

LoopBreakScopeManager:
  consumes condition_bindings and CarrierInfo during expression lookup
  validates lookup identity
  does not own route lowering selection
  readiness_gate_candidate=0
```

## Decision

The first executable integration seam should be the boundary construction /
route-lowering seam where both of these are available:

```text
carrier_info
condition_bindings
```

Do not place the readiness gate in `TrimRouteInfo::to_carrier_info`; it lacks
condition bindings and would force inference from names.

## Decision Record

```text
readiness_integration_inventory=1
selected_candidate=InlineBoundaryBuilder_or_route_lowering_boundary
trim_route_info_to_carrier_info_allowed=0
loop_break_scope_manager_allowed=0
condition_bindings_required=1
backend_behavior_changed=0
generated_program_execution_claim=0
```

## Stop Lines

```text
do not emit trim route lowering
do not infer condition bindings from helper names
do not move readiness gate into TrimRouteInfo::to_carrier_info
do not claim generated program execution
```
