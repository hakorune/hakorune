# 1871 - MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-HAKO-NATIVE-SOURCE-SEED-001

## Token

```text
MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-HAKO-NATIVE-SOURCE-SEED-001
```

## Purpose

Materialize the native Hako source seed for the carrier-merge assignment
mutation-frame family after its HakoShadow stage was promoted to HakoMainline.

This card does not run the HakoAdopted decision and does not claim Source
Selfhost.

## Output

```text
native source seed:
  lang/src/compiler/lib/carrier_merge_assignment_native_seed.hako

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-carrier-merge-assignment-hako-native-source-seed-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_carrier_merge_assignment_hako_native_source_seed_guard.sh
```

## Result

```text
native_source_owner_seed_present = 1
generator_overwrite_guard = 1
next_card = MIRBUILDER-CARRIER-MERGE-ASSIGNMENT-STATEMENT-HAKO-ADOPTION-DECISION-001
```

## Acceptance

```text
CarrierMergeAssignment HakoMainline promotion evidence consumed
CarrierMergeAssignment mutation-frame contract consumed
CarrierMergeAssignment native source seed exists outside lang/generated
module export for native seed exists
native seed preserves contracted mutation-frame state writes
generator overwrite guard is explicit
manual_family_selection = 0
family_adoption_decision = 0
source_selfhost_claim = 0
generated_artifact_as_edit_authority = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
```

## Non-Claims

```text
no HakoAdopted decision
no Source Selfhost claim
no Rust deletion
no runner semantic ownership
```
