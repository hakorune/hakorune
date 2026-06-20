# Hako Lifecycle Verifier Result Vocabulary v0

Status: SSOT
Scope: passive verifier result vocabulary for Rust-to-Hako lifecycle migration.

## Purpose

Name the positive verification result required by the lifecycle-aware
converter/emitter.

This vocabulary does not implement a verifier. It only defines the shape of
evidence that a future verifier must produce before emission.

Pipeline position:

```text
RustLifecycleFacts-v0
  -> HakoLifecycleResolver
  -> HakoLifecyclePlan-v0
  -> HakoLifecycleVerifier
  -> VerifierResult-v0
  -> converter/emitter
```

## Non-Goals

```text
verifier implementation
converter emission
backend behavior
resolver selection ownership
join_id vocabulary decision
trim_helper lifecycle proof
full VariableContext parity
MirBuilder-wide lifecycle parity
```

## Result Kinds

```text
VerifiedPlan:
  plan satisfies all required facts for its bounded subject

DenyUnverified:
  plan is missing required facts or unresolved boundary blocks verification

DenyUnsafeClaim:
  plan claims behavior outside its bounded subject
```

Only `VerifiedPlan` may be consumed by a lifecycle-aware emitter.

## Minimal Record Shape

```text
schema_version:
  0

kind:
  HakoLifecycleVerifierResult

mode:
  passive_fixture | verifier_output

subject:
  stable plan subject

source_facts:
  RustLifecycleFacts-v0 fixture or future fact id

source_plan:
  HakoLifecyclePlan-v0 fixture or future plan id

result:
  VerifiedPlan | DenyUnverified | DenyUnsafeClaim

verified_facts:
  fact predicates the verifier checked

verified_boundaries:
  lifecycle boundaries proven for this subject

denied_boundaries:
  unresolved boundaries explicitly not claimed

claims:
  emission_allowed
  backend_behavior_changed
  resolver_selection_owner
  full_variable_context_parity
  mirbuilder_wide_lifecycle
```

## Positive Rules

```text
emission_allowed:
  true only for VerifiedPlan

unknown fact:
  DenyUnverified

unresolved boundary:
  DenyUnverified unless explicitly outside subject

unsafe broad claim:
  DenyUnsafeClaim
```

## Initial Fixture Scope

The initial fixture may verify one bounded family only:

```text
CarrierInfo::merge_from:
  source_facts=carrier-info-merge-from-facts-v0.json
  source_plan=carrier-info-merge-from-plan-v0.json
  result=VerifiedPlan
  subject=OwnedCarrierInfoMerge only
```

It must continue to deny:

```text
join_id producer
trim_helper lifecycle owner
promoted_body_locals lifecycle owner
general resolver selection ownership
converter emission
backend behavior changes
```

## Stop Lines

```text
do not implement verifier here
do not emit .hako here
do not let VerifiedPlan imply whole-MirBuilder parity
do not use DenyUnverified as fallback plan
do not verify join_id-dependent paths without a producer owner
do not verify trim_helper ownership without a trim_helper owner
```
