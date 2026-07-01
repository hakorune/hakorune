# 1971 - MIRBUILDER-TYPE-CONTEXT-HAKO-NATIVE-SOURCE-SEED-001

## Token

```text
MIRBUILDER-TYPE-CONTEXT-HAKO-NATIVE-SOURCE-SEED-001
```

## Purpose

Materialize the native `.hako` source seed for
`hakorune_mir_builder::type_context` from `DerivedArtifactSeedDraftInput`.

The seed surface set is not hand-selected. It is derived by
`FamilySeedSurfaceCollationV1`: collect all `BridgeEligible` strict-emission
candidates with `owner_edge_id = hakorune_mir_builder::type_context` and
`selected_next_card = MIRBUILDER-TYPE-CONTEXT-HAKO-NATIVE-SOURCE-SEED-001`.

This card creates the native source seed and module export only. It does not
run a HakoAdopted decision and does not claim Source Selfhost.

## Surface Scope

```text
collation_rule:
  FamilySeedSurfaceCollationV1

selected surfaces:
  type_context.origin_map
  type_context.snapshot_restore
  type_context.string_literal
  type_context.value_kind
  type_context.value_type
```

## Acceptance

```text
rerun_005_consumed = 1
selected_owner_edge_id = hakorune_mir_builder::type_context
selected_next_card = MIRBUILDER-TYPE-CONTEXT-HAKO-NATIVE-SOURCE-SEED-001

family_seed_surface_collation_rule = FamilySeedSurfaceCollationV1
surface_selection_by_hand = 0
selected_surface_count = 5

native_source_seed_path =
  lang/src/compiler/lib/type_context_native_seed.hako

native_source_seed_outside_generated_tree = 1
module_export = lib.type_context_native_seed
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
  TypeContextNativeSourceSeedMaterialized

selected_next_card:
  MIRBUILDER-TYPE-CONTEXT-HAKO-ADOPTION-DECISION-001
```
