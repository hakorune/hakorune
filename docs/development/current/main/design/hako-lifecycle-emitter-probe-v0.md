# Hako Lifecycle Emitter Probe v0

Status: SSOT
Scope: first bounded lifecycle-aware emission probe.

## Purpose

Prove that the lifecycle-aware emitter can render one verified plan surface
without becoming a general Rust-to-Hako converter rewrite.

## Initial Subject

```text
subject=CarrierInfo::merge_from
plan_kind=OwnedCarrierInfoMerge
source_plan=carrier-info-merge-from-plan-v0.json
verifier_result=carrier-info-merge-from-emitter-verifier-result-v0.json
output=carrier-info-merge-from-emitter-surface-v0.hako
```

## Required Inputs

```text
RustLifecycleFacts-v0:
  carrier-info-merge-from-facts-v0.json

HakoLifecyclePlan-v0:
  carrier-info-merge-from-plan-v0.json

VerifierResult-v0:
  result=VerifiedPlan
  claims.emission_allowed=true
  backend_behavior_changed=false
  resolver_selection_owner=false
  full_variable_context_parity=false
  mirbuilder_wide_lifecycle=false
```

## Output Contract

The probe renders a `.hako` skeleton surface that is explicitly bounded:

```text
emitted_subject:
  CarrierInfo::merge_from only

emitted_plan_kind:
  OwnedCarrierInfoMerge

allowed content:
  plan provenance comments
  function skeleton for verified lifecycle surface
  TODO body comments for not-yet-lowered mutation details

forbidden content:
  join_id producer
  trim_helper lifecycle owner claim
  promoted_body_locals lifecycle owner claim
  general converter behavior
  backend behavior
```

## Stop Lines

```text
do not emit unverified plans
do not rewrite converter_core.hako
do not add Rust code
do not claim generated program execution
do not claim full VariableContext parity
do not claim MirBuilder-wide lifecycle parity
```
