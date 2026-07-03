# 2098 - SOURCE-SELFHOST-RUST-TO-HAKO-CONVERTER-ROLE-PIVOT-001

## Token

```text
SOURCE-SELFHOST-RUST-TO-HAKO-CONVERTER-ROLE-PIVOT-001
```

## Purpose

Record the role pivot after the Source Selfhost route-selection closeout.

The full Rust-to-Hako MirBuilder converter is no longer the Source Selfhost
main route. Rust remains the bootstrap oracle and parity reference. New
Source Selfhost progress must come from small hand-authored `.hako` native
owners with machine-checked Rust-oracle parity.

The converter is retained only as a helper for inventories, test-vector
extraction, parity fixture generation, and narrow library-subset draft output.

## Consumed State

```text
closeout_card:
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-CLOSEOUT-001

closeout_reason:
  SourceSelfhostRouteSelectionExhaustedNoMachineDerivedNextLane

parked_or_exhausted_lanes:
  DomainObjectIdLane
  CarrierTypeRemainingAxisLane
  CarrierTypeParentPolicyLane
  MissingProjectionPolicyPostTypeTransportLane
```

## Decision

```text
decision:
  SelectRustOracleParityMigrationPath

reason_token:
  FullRustToHakoConverterMainPathExhaustedByMachineDerivedRouteSelection

full_rust_to_hako_converter_as_source_selfhost_main_path:
  stopped

rust_role:
  bootstrap_oracle_and_parity_reference

hako_native_role:
  hand_authored_owner_implementation

converter_role:
  migration_helper_library_subset_draft_only

selected_next_card:
  MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-001
```

## RustOracleParityMigrationPolicyV1

Allowed:

```text
human_or_ai_target_selection = 1
small_hako_native_owner_written_by_hand = 1
rust_oracle_fixture_required = 1
same_input_same_output_or_mir_parity_required = 1
parity_mismatch_must_emit_fixture_diff = 1
converter_may_generate_draft_or_test_vectors = 1
```

Required before any adoption claim:

```text
selected_owner_scope_is_small = 1
rust_oracle_fixture_exists = 1
hako_native_source_exists = 1
parity_gate_exists = 1
parity_gate_passes = 1
generated_artifact_as_native_edit_authority = 0
```

Forbidden:

```text
source_selfhost_claim = 0
hako_adopted_decision_before_parity = 0
native_seed_materialization_from_converter = 0
full_rust_semantic_projection_as_authority = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
manual_correctness_claim_without_parity = 0
```

## Converter Scope

Keep:

```text
read_only_inventory_tools
source_surface_extraction
parity_fixture_generation
test_vector_extraction
library_subset_draft_converter
```

Freeze / stop expanding:

```text
full_mirbuilder_auto_conversion
projection_policy_mega_router
result_option_borrow_carrier_inference_as_adoption_authority
generated_hako_as_native_edit_authority
route_selection_cards_that_only_park_another_lane
```

## Next Card Contract

`MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-001` must select a
small owner for hand-authored `.hako` implementation. It may use human/AI
target choice, but its correctness proof must be machine-checked parity
against Rust oracle fixtures.

It must not select Source Selfhost, Hako adoption, native seed materialization,
a backend route, or an ABI.

## Non-Claims

```text
source_selfhost_claim = 0
source_selfhost_complete = 0
hako_adopted_decision = 0
native_seed_materialization = 0
hako_generation = 0
projection_policy_selected = 0
generated_artifact_as_native_edit_authority = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
runner_semantic_owner = 0
```
