# Condition-Binding Resolution Rewrite Design

Status: design
Scope: promoted condition identity consumption after proof probe.

## Purpose

Condition-binding identity is now fixture-guarded as a proof candidate:

```text
AllowIdentityCandidate(ConditionBinding.join_value)
```

Existing consumers still call:

```rust
CarrierInfo::resolve_promoted_join_id(original_name)
```

That method depends on parked `CarrierVar.join_id` vocabulary. This design
chooses a narrow additive rewrite path.

## Decision

Do not rewrite or remove `resolve_promoted_join_id` in place.

Add a new adapter in a future implementation row:

```text
resolve_promoted_condition_binding_identity(
  original_name,
  trim_helper,
  promoted_body_locals,
  condition_bindings
) -> Option<ValueId>
```

The adapter consumes `ConditionBinding.join_value` only when the proof
conditions match.

## Why Additive

```text
legacy path:
  CarrierInfo::resolve_promoted_join_id
  uses CarrierVar.join_id
  remains denied / compatibility-only

new path:
  condition-binding identity adapter
  uses ConditionBinding.join_value
  can be tested independently
```

This avoids mutating legacy semantics before the new path is proven.

## Required Adapter Inputs

```text
original_name
trim_helper.original_var
trim_helper.carrier_name
CarrierInfo.promoted_body_locals
JoinInlineBoundary.condition_bindings
```

## Adapter Rules

Allow only when:

```text
promoted_body_locals contains original_name
trim_helper.original_var == original_name
condition_bindings contains name == trim_helper.carrier_name
```

Return:

```text
Some(condition_binding.join_value)
```

Deny/fallback:

```text
missing promoted body-local name -> None
missing trim_helper -> None
missing matching condition binding -> None
name mismatch -> None
```

## Non-Goals

```text
do not remove CarrierVar.join_id
do not implement CarrierVar.join_id producer
do not change scope_manager lookup in this design row
do not emit trim route lowering
do not claim generated program execution
```

## Decision Record

```text
rewrite_design_documented=1
rewrite_shape=additive_adapter
new_adapter=resolve_promoted_condition_binding_identity
legacy_resolve_promoted_join_id_kept=1
implementation_started=0
backend_behavior_changed=0
generated_program_execution_claim=0
```
