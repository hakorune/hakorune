# Promoted Carrier Identity Policy Decision

Status: decision
Scope: promoted carrier identity after join_id / condition-binding inventory.

## Decision

Use condition-binding identity as the next policy to prove.

```text
selected_policy=condition_binding_identity
join_id_producer_selected=0
keep_denied_forever_selected=0
implementation_started=0
```

This is not an implementation row.

## Rationale

`CarrierVar.join_id` is parked in the lifecycle lane:

```text
production Some(ValueId) producer:
  none

production assignment:
  none

test Some(ValueId):
  scope_manager fixtures only
```

Reviving it now would reopen broad PHI carrier value-space design.

The existing JoinIR boundary builder already owns a narrower identity path for
condition-only values:

```text
ParamRole::Condition:
  ConditionBinding { name, host_value, join_value }
```

The clean next step is to prove whether promoted body-local names can map to
the intended `ConditionBinding.join_value`.

## Required Future Proof

A future proof probe must show:

```text
promoted_body_locals contains original_name
trim_helper.original_var == original_name
condition_bindings contains selected promoted carrier name
condition binding is condition-only identity
join_value is the identity returned to trim route decision
```

If any condition is missing:

```text
DenyMissingConditionBindingIdentity
```

## Non-Goals

```text
do not implement condition-binding resolution in this decision
do not implement CarrierVar.join_id producer
do not fabricate ValueId from indices or names
do not emit trim route lowering
do not claim generated program execution
```

## Decision Record

```text
policy_decision_recorded=1
selected_policy=condition_binding_identity
join_id_producer_added=0
condition_binding_rewrite_added=0
trim_route_lowering_still_denied=1
backend_behavior_changed=0
generated_program_execution_claim=0
```
