# 1930 - MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-HAKO-NATIVE-SOURCE-SEED-001

## Token

```text
MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-HAKO-NATIVE-SOURCE-SEED-001
```

## Purpose

Materialize the native Hako source seed for the StatementValueConstruction box
field initialization mutation-frame family after its HakoShadow stage was
promoted to HakoMainline.

This card does not run the HakoAdopted decision and does not claim Source
Selfhost.

## Output

```text
native source seed:
  lang/src/compiler/lib/box_field_initialization_native_seed.hako

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-statement-value-construction-box-field-initialization-hako-native-source-seed-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_statement_value_construction_box_field_initialization_hako_native_source_seed_guard.sh
```

## Result

```text
native_source_owner_seed_present = 1
generator_overwrite_guard = 1
next_card = MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-HAKO-ADOPTION-DECISION-001
```

## Acceptance

```text
BoxFieldInitialization HakoMainline promotion evidence consumed
BoxFieldInitialization mutation-frame contract consumed
BoxFieldInitialization native source seed exists outside lang/generated
module export for native seed exists
native seed preserves contracted validation / delegation order
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
