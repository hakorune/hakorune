# 1965 - MIRBUILDER-CORE-CONTEXT-HAKO-NATIVE-SOURCE-SEED-001

## Token

```text
MIRBUILDER-CORE-CONTEXT-HAKO-NATIVE-SOURCE-SEED-001
```

## Purpose

Materialize the native `.hako` source seed for
`hakorune_mir_builder::core_context` from `DerivedArtifactSeedDraftInput`.

This card creates the native source seed and module export only. It does not
run a HakoAdopted decision and does not claim Source Selfhost.

## Output

```text
native seed:
  lang/src/compiler/lib/core_context_native_seed.hako

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-core-context-hako-native-source-seed-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_core_context_hako_native_source_seed_guard.sh
```

## Acceptance

```text
candidate_selection_consumed = 1
selected_owner_edge_id = hakorune_mir_builder::core_context
native_source_seed_path = lang/src/compiler/lib/core_context_native_seed.hako
native_source_seed_outside_generated_tree = 1
module_export = lib.core_context_native_seed
generator_overwrite_guard = 1

native seed contains:
  CoreContext
  CoreContextApi.next_value
  CoreContextApi.peek_next_value
  CoreContextApi.next_block
  CoreContextApi.peek_next_block
  CoreContextApi.next_binding
  CoreContextApi.next_temp_slot
  CoreContextApi.next_debug_join

generated artifact as edit authority = 0
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
  NativeSourceSeedMaterialized

selected_next_card:
  MIRBUILDER-CORE-CONTEXT-HAKO-ADOPTION-DECISION-001
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
