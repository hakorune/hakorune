# 1995 - MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-HAKO-NATIVE-SOURCE-SEED-001

## Token

```text
MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-HAKO-NATIVE-SOURCE-SEED-001
```

## Purpose

Materialize the native `.hako` source seed for
`hakorune_mir_builder::record_packed_layout_refresh` from
`DerivedArtifactSeedDraftInput`.

This card creates the native source seed and module export only. It does not
run a HakoAdopted decision and does not claim Source Selfhost.

## Output

```text
native seed:
  lang/src/compiler/lib/record_packed_layout_refresh_native_seed.hako

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-record-packed-layout-refresh-hako-native-source-seed-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_record_packed_layout_refresh_hako_native_source_seed.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_record_packed_layout_refresh_hako_native_source_seed_guard.sh
```

## Acceptance

```text
selection_rerun_003_consumed = 1
selected_owner_edge_id = hakorune_mir_builder::record_packed_layout_refresh
native_source_seed_path = lang/src/compiler/lib/record_packed_layout_refresh_native_seed.hako
native_source_seed_outside_generated_tree = 1
module_export = lib.record_packed_layout_refresh_native_seed
generator_overwrite_guard = 1

native seed contains:
  RecordPackedLayoutRefreshPayloadBox
  RecordPackedLayoutRefreshResultBox
  RecordPackedLayoutRefreshFixtureApi
  RecordPackedLayoutRefreshApi.project_shadow_record

generated artifact as edit authority = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_canonical_mir_instruction = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Result

```text
decision:
  HakoAdoptionDecisionDeferred

reason_token:
  RecordPackedLayoutRefreshNativeSourceSeedMaterialized

selected_next_card:
  MIRBUILDER-RECORD-PACKED-LAYOUT-REFRESH-HAKO-ADOPTION-DECISION-001
```

## Non-Claims

```text
no HakoAdopted decision
no Source Selfhost claim
no runtime fallback
no new backend route
no new ABI
no new canonical MIR instruction
no new Python SemanticProjector
no runner semantic ownership
```
