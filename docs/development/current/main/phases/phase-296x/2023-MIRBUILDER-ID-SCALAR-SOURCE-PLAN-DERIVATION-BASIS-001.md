# 2023 - MIRBUILDER-ID-SCALAR-SOURCE-PLAN-DERIVATION-BASIS-001

## Token

```text
MIRBUILDER-ID-SCALAR-SOURCE-PLAN-DERIVATION-BASIS-001
```

## Purpose

Define the basis for deriving `SourcePlanAndRecipe` from ID scalar descriptor
and directability evidence.

`DirectabilityOnly` and descriptor-only evidence remain insufficient. This card
only defines the machine-checkable conditions required before a later inventory
or rerun may call a source plan derivable.

## Result

```text
directability_only_is_source_plan = 0
descriptor_only_is_source_plan = 0
source_plan_derivation_basis_defined = 1

decision:
  PolicyDefined

reason_token:
  IdScalarSourcePlanDerivationBasisDefined

selected_next_card:
  MIRBUILDER-ID-SCALAR-SOURCE-SURFACE-INVENTORY-001
```

## Basis

```text
requires:
  owner_edge_confidence_exact_or_fixture
  owner_scope_bounded
  required_source_surfaces_complete
  operation_vocabulary_complete
  behavior_recipe_effect_coverage_complete
  nominal_id_domain_isolation_preserved
  id_domain_boundary_declared
  state_mutation_frame_declared
  error_semantics_declared
  deterministic_order_declared
  verifier_input_contract_declared
  no_borrow_policy_gap
  no_carrier_type_transport_gap
  no_runtime_fallback
  no_new_backend_route
  no_new_abi
  no_new_python_semantic_projector
```

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-id-scalar-source-plan-derivation-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_id_scalar_source_plan_derivation_basis.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_id_scalar_source_plan_derivation_basis_guard.sh
```

## Non-Claims

```text
source_plan_implied_by_descriptor = 0
source_plan_implied_by_directability = 0
behavior_recipe_implied_by_descriptor = 0
behavior_recipe_implied_by_directability = 0
verifier_result_implied_by_source_plan = 0
derived_artifact_seed_draft_implied_by_verifier = 0
raw_i64_interchangeability = 0
nominal_id_erasure = 0
id_sentinel_semantics_inferred = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
```
