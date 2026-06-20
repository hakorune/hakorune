# Scope-Manager Condition-Binding Adapter Wiring Design

Status: design
Scope: lookup consumption of `CarrierInfo::resolve_promoted_condition_binding_identity`.

## Purpose

`CarrierInfo` now has a read-only condition-binding identity adapter. It is not
yet consumed by `LoopBreakScopeManager`.

This design fixes the lookup boundary before any behavior change.

## Current Boundary

```text
LoopBreakScopeManager inputs:
  condition_env
  loop_body_local_env
  captured_env
  carrier_info

current promoted fallback:
  carrier_info.resolve_promoted_join_id(name)
```

`JoinInlineBoundary.condition_bindings` is not available to the scope manager.

## Decision

Add an explicit condition-binding slice to the scope manager in a future
implementation row.

```text
LoopBreakScopeManager:
  condition_bindings: &'a [ConditionBinding]
```

Lookup order for the future implementation:

```text
1. condition_env
2. loop_body_local_env
3. captured_env
4. condition-binding adapter
5. legacy resolve_promoted_join_id
```

The new adapter is a positive identity consumer only. If it cannot prove a
match, it returns `None` and does not produce a fact.

## Why Legacy Remains

```text
legacy_resolve_promoted_join_id_kept=1
reason=existing routes still rely on CarrierVar.join_id when produced
```

This wiring row must not remove legacy behavior. The adapter is a narrow
additional lookup source for condition-binding identity.

## Required Inputs

```text
name
condition_bindings
carrier_info.trim_helper
carrier_info.promoted_body_locals
```

## Deny / No-Claim Cases

```text
missing_condition_bindings_input -> no implementation
adapter_none -> continue existing lookup behavior
missing_trim_helper -> adapter_none
missing_matching_condition_binding -> adapter_none
generated_program_execution_claim -> forbidden
trim_route_lowering_claim -> forbidden
```

## Non-Goals

```text
do not emit trim route lowering
do not remove resolve_promoted_join_id
do not change ConditionEnv semantics
do not infer condition bindings from helper names
do not start rustc adapter work
```

## Decision Record

```text
wiring_design_documented=1
wiring_shape=explicit_scope_manager_condition_bindings_input
lookup_order_documented=1
legacy_resolve_promoted_join_id_kept=1
implementation_started=0
backend_behavior_changed=0
generated_program_execution_claim=0
```
