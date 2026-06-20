# Promoted Carrier Identity / join_id Design Inventory

Status: design inventory
Scope: promoted carrier identity after trim route lowering decision probing.

## Purpose

Trim route lowering is blocked by:

```text
DenyMissingPromotedCarrierIdentity
```

The current metadata proves a trim candidate exists, but executable lowering
needs a stable identity for the promoted carrier.

This row inventories the choices. It does not implement any producer.

## Current State

`CarrierInfo::resolve_promoted_join_id(original_name)` currently requires:

```text
promoted_body_locals contains original_name
carrier named is_<name> or is_<name>_match exists
carrier.join_id is Some(ValueId)
```

The current production pipeline has:

```text
promoted_body_locals producers:
  present

CarrierVar.join_id Some(ValueId) producer:
  absent in production

CarrierVar.join_id assignment:
  absent in production

scope_manager tests:
  use Some(ValueId) as fixture-only evidence
```

## Adjacent Value Path

`JoinInlineBoundaryBuilder::add_param_with_role` already creates
JoinIR-local values for condition bindings:

```text
ParamRole::Condition:
  create ConditionBinding { name, host_value, join_value }
```

This is not the same as `CarrierVar.join_id`.

## Design Choices

### A. Keep Denied

```text
decision:
  keep MissingPromotedCarrierIdentity as the executable trim-lowering deny

pros:
  safest
  avoids reviving stale join_id vocabulary

cons:
  trim route lowering remains blocked
```

### B. Implement CarrierVar.join_id Producer

```text
decision:
  assign CarrierVar.join_id from the actual JoinIR carrier value producer

requirements:
  single owner for assignment
  no test fixture truth
  no dummy ValueId
  verifier proves identity source
```

Risk:

```text
can reopen PHI carrier value-space broadly
```

### C. Replace Resolution with ConditionBinding Identity

```text
decision:
  use condition_bindings join_value as the promoted condition identity
  retire or bypass CarrierVar.join_id for promoted condition-only paths
```

Requirements:

```text
prove promoted body-local name maps to a ConditionBinding
prove condition binding is the intended condition-only carrier
keep loop-state carrier behavior separate
```

Risk:

```text
can mix CarrierInfo and JoinInlineBoundary ownership if not separated
```

## Recommended Next Decision

Do not implement anything from this inventory row.

The next row should choose:

```text
keep denied
or
implement CarrierVar.join_id producer
or
replace promoted-name resolution with condition-binding identity
```

## Decision

```text
promoted_carrier_identity_inventory=1
production_join_id_producer=0
condition_binding_identity_path_present=1
selected_implementation=none
trim_route_lowering_still_denied=1
backend_behavior_changed=0
generated_program_execution_claim=0
```

## Stop Lines

```text
do not implement join_id producer in this inventory
do not fabricate join_id from index or name
do not treat scope_manager tests as production truth
do not repurpose condition_bindings without a decision row
do not emit trim route lowering
do not claim generated program execution
```
