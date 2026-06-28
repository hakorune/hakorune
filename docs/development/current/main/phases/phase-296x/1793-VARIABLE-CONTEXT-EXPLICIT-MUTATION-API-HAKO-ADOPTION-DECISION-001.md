---
Status: Active
Decision: accepted
Date: 2026-06-29
Scope: Adopt the machine-derived VariableContext native surface that includes
  explicit mutation APIs for the formerly returned mutable borrow route.
Related:
  - docs/development/current/main/phases/phase-296x/1792-MIRBUILDER-VARIABLE-CONTEXT-ROUTE-MATRIX-RERUN-002.md
  - docs/development/current/main/phases/phase-296x/1791-MIRBUILDER-VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-PROJECTION-001.md
  - docs/development/current/main/design/fixtures/rust-lifecycle/variable-context-explicit-mutation-api-hako-adoption-decision-v0.json
  - tools/checks/rust_lifecycle_variable_context_explicit_mutation_api_hako_adoption_decision_guard.sh
---

# VARIABLE-CONTEXT-EXPLICIT-MUTATION-API-HAKO-ADOPTION-DECISION-001

## Summary

Adopt the selected `VariableContextNativeSurfaceExplicitMutationApiOnlyV1`
surface as native Hako authority. The full `VariableContext` family remains
parked because returned borrow routes are still denied.

```text
docs_only_closeout = forbidden
code_or_guard_delta_required = 1
```

## Authority

Selection evidence:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  variable-context-route-matrix-rerun-002-v0.json
docs/development/current/main/design/fixtures/rust-lifecycle/
  variable-context-explicit-mutation-api-projection-v0.json
```

Native source owner evidence:

```text
apps/lib/hakorune_mir_builder/variable_context.hako
apps/lib/hakorune_mir_builder/carrier_info.hako
tools/checks/rust_mirbuilder_variable_context_native_simple_map_guard.sh
tools/checks/rust_mirbuilder_variable_context_native_snapshot_restore_guard.sh
tools/checks/rust_mirbuilder_carrier_info_native_snapshot_guard.sh
tools/checks/rust_lifecycle_variable_context_explicit_mutation_api_projection_guard.sh
```

## Acceptance

```text
decision = Adopt
surface_id = VariableContextNativeSurfaceExplicitMutationApiOnlyV1
included_scopes are all DerivedMainline or repaired bounded surface scopes
native_hako_source_owner_present = 1
native_behavior_guard_green = 1
explicit_mutation_api_projection_green = 1
replace_owned_map_native_api = 1
generator_overwrite_guard = 1
rust_bootstrap_retained = 1
rust_oracle_retained = 1
generated_artifact_manual_edit = 0
full_variable_context_claim = 0
returned_borrow_selected = 0
raw_variable_map_alias_selected = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
manual_family_selection = 0
```

## Non-Claims

```text
full VariableContext HakoAdopted = 0
returned read borrow repair = 0
returned mutable borrow repair = 0
BorrowView implementation = 0
Rust reference parity = 0
Source Selfhost = 0
Rust deletion = 0
runtime fallback = 0
new backend route = 0
new ABI = 0
```

## Closeout

```text
output_contract=rust-lifecycle-variable-context-explicit-mutation-api-hako-adoption-decision-v0
surface_id=VariableContextNativeSurfaceExplicitMutationApiOnlyV1
decision=Adopt
native_hako_source_owner_present=1
native_behavior_guard_green=1
explicit_mutation_api_projection_green=1
replace_owned_map_native_api=1
generator_overwrite_guard=1
rust_bootstrap_retained=1
rust_oracle_retained=1
full_variable_context_claim=0
returned_borrow_selected=0
raw_variable_map_alias_selected=0
source_selfhost_claim=0
runtime_fallback=0
new_backend_route=0
new_abi=0
manual_family_selection=0
summary=ok
```
