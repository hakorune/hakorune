# Condition-Binding Promoted Identity Proof Probe

Status: fixture probe
Scope: read-only proof candidate for promoted condition identity.

## Purpose

The selected policy is:

```text
selected_policy=condition_binding_identity
```

This probe defines the proof shape without changing resolution behavior.

## Input Facts

```text
promoted_body_locals contains original_var
trim_helper.original_var == original_var
trim_helper.carrier_name == promoted carrier name
condition_bindings contains promoted carrier name
ConditionBinding.join_value exists
```

## Decision Shape

```text
ConditionBindingPromotedIdentityDecision:
  AllowIdentityCandidate(join_value)
or
  DenyMissingConditionBindingIdentity
or
  DenyPromotedNameMismatch
```

The `join_value` is only a proof candidate in this row.

## Non-Goals

```text
do not rewrite CarrierInfo::resolve_promoted_join_id
do not implement CarrierVar.join_id producer
do not emit trim route lowering
do not change backend behavior
```

## Fixture Meaning

Positive vector:

```text
original_var=ch
carrier_name=is_ch_match
promoted_body_locals=[ch]
condition_bindings=[{ name=is_ch_match, join_value=ValueId(200) }]
decision=AllowIdentityCandidate(ValueId(200))
```

Negative vectors:

```text
missing condition binding:
  DenyMissingConditionBindingIdentity

promoted name mismatch:
  DenyPromotedNameMismatch
```

## Decision

```text
condition_binding_identity_proof_probe=1
allow_identity_candidate=1
resolution_rewrite_added=0
join_id_producer_added=0
trim_route_lowering_added=0
backend_behavior_changed=0
generated_program_execution_claim=0
```

## Stop Lines

```text
do not change resolve_promoted_join_id
do not treat proof candidate as executable route permission
do not emit trim route lowering
do not fabricate join_id
do not claim generated program execution
```
