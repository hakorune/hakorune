# 1968 - MIRBUILDER-METADATA-CONTEXT-HAKO-NATIVE-SOURCE-SEED-001

## Token

```text
MIRBUILDER-METADATA-CONTEXT-HAKO-NATIVE-SOURCE-SEED-001
```

## Purpose

Materialize the native `.hako` source seed for
`hakorune_mir_builder::metadata_context` from `DerivedArtifactSeedDraftInput`.

The seed surface set is not hand-selected. It is derived by
`FamilySeedSurfaceCollationV1`: collect all `BridgeEligible` strict-emission
candidates with `owner_edge_id = hakorune_mir_builder::metadata_context` and
`selected_next_card = MIRBUILDER-METADATA-CONTEXT-HAKO-NATIVE-SOURCE-SEED-001`.

This card creates the native source seed and module export only. It does not
run a HakoAdopted decision and does not claim Source Selfhost.

## Output

```text
native seed:
  lang/src/compiler/lib/metadata_context_native_seed.hako

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-metadata-context-hako-native-source-seed-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_metadata_context_hako_native_source_seed_guard.sh
```

## Surface Scope

```text
collation_rule:
  FamilySeedSurfaceCollationV1

selected surfaces:
  metadata_context.scalar_source_file
    role = OwnerFieldSurface

  metadata_context.value_caller
    role = OwnerMapProjectionSurface

  metadata_context.region_parent
    role = OwnerScopedHelperSurface
    general_arraybox_policy = 0
    standalone_current_region_stack = Deny(ReturnedReadBorrow)
    returned_borrow_authority = 0
```

## Acceptance

```text
rerun_004_consumed = 1
selected_owner_edge_id = hakorune_mir_builder::metadata_context
selected_next_card = MIRBUILDER-METADATA-CONTEXT-HAKO-NATIVE-SOURCE-SEED-001

family_seed_surface_collation_rule = FamilySeedSurfaceCollationV1
surface_selection_by_hand = 0
selected_surface_count = 3

required surfaces:
  metadata_context.scalar_source_file
  metadata_context.value_caller
  metadata_context.region_parent

native_source_seed_path =
  lang/src/compiler/lib/metadata_context_native_seed.hako

native_source_seed_outside_generated_tree = 1
module_export = lib.metadata_context_native_seed
generator_overwrite_guard = 1

generated_artifact_as_native_edit_authority = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Result

```text
decision:
  HakoAdoptionDecisionDeferred

reason_token:
  MetadataContextNativeSourceSeedMaterialized

selected_next_card:
  MIRBUILDER-METADATA-CONTEXT-HAKO-ADOPTION-DECISION-001
```

## Non-Claims

```text
no HakoAdopted decision
no Source Selfhost claim
no runtime fallback
no new backend route
no new ABI
no new Python SemanticProjector
no runner semantic ownership
```
