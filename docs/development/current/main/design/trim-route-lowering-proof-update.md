# Trim Route Lowering Proof Update

Status: fixture proof
Scope: trim route lowering decision after condition-binding identity lookup.

## Purpose

The original trim route lowering decision denied executable lowering with:

```text
MissingPromotedCarrierIdentity
```

Since then, the lifecycle lane added:

```text
ConditionBinding.join_value proof
CarrierInfo::resolve_promoted_condition_binding_identity
LoopBreakScopeManager.condition_bindings
```

This proof update reclassifies the identity dependency while keeping backend
trim route lowering denied.

## Updated Decision

```text
metadata_decision=AllowMetadataCandidate
identity_decision=AllowConditionBindingIdentity
executable_decision=Deny
deny_reason=MissingExecutableTrimRouteLoweringImplementation
```

## Why Executable Lowering Still Denies

The identity path is now available as lookup proof, but this row does not add:

```text
trim route backend lowering
route emitter changes
generated program execution support
```

Therefore executable lowering remains denied by implementation readiness, not
by missing promoted carrier identity.

## Required Proof Chain

```text
trim_helper.has_valid_structure=1
condition_binding_identity_available=1
carrier_info_adapter_available=1
scope_manager_condition_bindings_input_available=1
scope_manager_lookup_consumes_adapter=1
backend_trim_lowering_implementation=0
```

## Decision Record

```text
trim_route_proof_updated=1
missing_promoted_carrier_identity_retired_or_reclassified=1
scope_manager_condition_binding_input_consumed_as_proof=1
executable_lowering_allow=0
deny_reason=MissingExecutableTrimRouteLoweringImplementation
backend_behavior_changed=0
generated_program_execution_claim=0
```

## Stop Lines

```text
do not emit trim route lowering
do not add backend lowering
do not claim generated program execution
do not start rustc adapter work
```
