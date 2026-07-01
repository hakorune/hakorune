# 1969 - MIRBUILDER-METADATA-CONTEXT-HAKO-ADOPTION-DECISION-001

## Token

```text
MIRBUILDER-METADATA-CONTEXT-HAKO-ADOPTION-DECISION-001
```

## Purpose

Decide whether the `metadata_context` native source seed becomes a
HakoAdopted native source owner.

This card consumes the machine-derived `FamilySeedSurfaceCollationV1` seed for
`scalar_source_file`, `value_caller`, and `region_parent`. It adopts the bounded
metadata_context surface set as native `.hako` edit authority, while keeping
Source Selfhost unclaimed.

## Input Authority

```text
native seed fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-metadata-context-hako-native-source-seed-v0.json

native source:
  lang/src/compiler/lib/metadata_context_native_seed.hako

strict emission candidate selection:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-strict-converter-emission-native-seed-candidate-selection-v0.json
```

## Decision

```text
value:
  Adopt

reason_token:
  MetadataContextNativeSeedPresentAndStrictEmissionBridgeGreen

selected_next_route:
  native_hako_source_owner
```

## Acceptance

```text
native_source_seed_guard = green
surface_collation_rule = FamilySeedSurfaceCollationV1
selected_surface_count = 3

required surfaces:
  metadata_context.scalar_source_file
  metadata_context.value_caller
  metadata_context.region_parent

region_parent:
  role = OwnerScopedHelperSurface
  general_arraybox_policy = 0
  returned_borrow_authority = 0

native source:
  hako-adopted = 1
  generated artifact marker absent
  generated smoke Main absent

generated_artifact_as_edit_authority = 0
source_selfhost_claim = 0
rust_deletion = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Next

```text
MIRBUILDER-CRATE-WIDE-NATIVE-OWNER-SEED-CAPABILITY-SURVEY-RERUN-005
```
