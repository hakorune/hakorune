---
Status: Complete
Date: 2026-06-24
Token: REGION-SLOT-CLASSIFIER-LIVE-FACT-OWNER-001
Scope: MirBuilder Rust-to-Hako converter / RegionObserver classifier policy owner
---

# 296x-1660 REGION-SLOT-CLASSIFIER-LIVE-FACT-OWNER-001

## Decision

Select the RegionObserver slot classifier as the next semantic converter
owner. The remaining problem is not emitter-owned policy; `ClassifyEnumVariants`
already renders operation data. The current risk is that the RegionObserver
family spec still hand-writes the classifier table.

```text
source authority:
  src/mir/region/mod.rs::Region::classify_ref_kind
  src/mir/region/observer.rs::classify_slot_name_only

normalized facts:
  ClassifierDecisionFactsV1

lowering:
  facts + target plan
    -> existing ClassifyEnumVariants

non-authority:
  family artifact spec
  shared operation emitter
  backend
```

## Selected Source Shape

```text
Region::classify_ref_kind:
  Box / Array / Future -> StrongRoot
  WeakRef              -> WeakRoot
  other                -> NonRef

classify_slot_name_only:
  literal name set -> StrongRoot
  default          -> NonRef
```

Facts must not contain Hako expressions. Facts describe classifier decisions;
the target plan maps semantic labels to Hako output variants.

## Implementation Tasks

```text
1. Extract ClassifierDecisionFactsV1 from live Rust source.
2. Add classifier facts to the RegionObserver facts fixture/provenance.
3. Remove ref_kind_groups and missing_value_fallback hand-written policy from
   mirbuilder_region_observer_artifacts.py.
4. Add a family-neutral lowerer from classifier facts + target plan to the
   existing ClassifyEnumVariants operation.
5. Keep shared_mirbuilder_operation_emitter.py behavior unchanged.
```

## Acceptance

```text
generated classifier/read-fold method bodies remain byte-identical
generator --check green
RegionObserver MIR green
RegionObserver EXE/AOT green
current_state_pointer_guard green
rust_mirbuilder_converter_matrix_guard green
rust_lifecycle_no_silent_hardcode_guard green
new operation kind = 0
new backend route = 0
runtime fallback = 0
```

Source-authority proof:

```text
temporary source mutation changes extracted facts and operation data:
  WeakRef -> StrongRoot
  add literal fallback name "foo"
```

## Fail-Fast

```text
Deny(UnsupportedDirectShape)
  detail=UnsupportedClassifierPattern

Deny(UnsupportedDirectShape)
  detail=DynamicClassifierPredicate

Deny(UnsupportedDirectShape)
  detail=AmbiguousClassifierDefault

Deny(UnsupportedTypeTransport)
  detail=UnmappedClassifierResult
```

Reject guarded match arms, dynamic predicate calls, non-literal string sets,
duplicate variants, missing defaults, and unmapped semantic result labels.
Never silently map unsupported classifier shapes to `NonRef`.

## Non-Claims

```text
full RegionObserver conversion = 0
classify_slots_from_registry conversion = 0
full MirType declaration extraction = 0
general decision-DAG framework = 0
new Hako syntax = 0
backend behavior changed = 0
```

## Closeout

```text
ClassifierDecisionFactsV1=landed
source_authority=Region::classify_ref_kind + classify_slot_name_only
family_spec_ref_kind_groups=0
family_spec_missing_value_fallback=0
generic_classifier_lowerer=landed
shared_operation_emitter_behavior_changed=0
new_operation_kind=0
new_backend_route=0
runtime_fallback=0
generated_region_observer_slot_metadata_artifact=deterministic
generated_classifier_read_fold_method_bodies=unchanged
region_observer_generated_mir=green
region_observer_generated_exe_aot=green
source_mutation_fact_change_probe=green
rust_mirbuilder_converter_matrix_guard=green
rust_lifecycle_no_silent_hardcode_guard=green
current_state_pointer_guard=green
next=MIRBUILDER-CONVERTER-NEXT-SLICE-DESIGN-STOP-001
```
