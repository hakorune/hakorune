---
Status: Selected
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
generated region_observer_slot_metadata.hako body remains byte-identical
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
