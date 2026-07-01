# 2030 - MIRBUILDER-ID-SCALAR-STATE-TARGET-ENUMERATION-BASIS-001

## Token

```text
MIRBUILDER-ID-SCALAR-STATE-TARGET-ENUMERATION-BASIS-001
```

## Purpose

Enumerate semantic state targets for the four ID scalar owner-edge candidates
from machine-derived operation vocabulary rows.

## Result

```text
input_candidate_count = 4
state_targets_enumerated_owner_edge_count = 4
state_target_count = 22
all_targets_inside_owner_scope_count = 2
cross_owner_state_target_count = 4
mutation_frame_required_owner_edge_count = 3

decision:
  StateTargetBasisDefined

reason_token:
  IdScalarStateTargetsEnumerated

selected_next_card:
  MIRBUILDER-ID-SCALAR-OWNER-SCOPE-BOUNDEDNESS-RESOLUTION-002
```

## Boundary

State targets are semantic resources grouped by `owner_edge`. Source paths,
surface counts, and owner names are evidence only, not authority. This basis
does not choose a native seed owner or materialize SourcePlanAndRecipe.

## Output

```text
fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-id-scalar-state-target-enumeration-basis-v0.json

tool:
  tools/rust_lifecycle/
    mirbuilder_id_scalar_state_target_enumeration_basis.py

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_id_scalar_state_target_enumeration_basis_guard.sh
```

## Non-Claims

```text
manual_owner_selection = 0
manual_axis_selection = 0
surface_count_as_proof = 0
source_file_path_as_authority = 0
source_plan_materialization = 0
behavior_recipe_materialization = 0
verifier_result_materialization = 0
native_seed_materialization = 0
hako_generation = 0
hako_adopted_decision = 0
source_selfhost_claim = 0
```
