# 1926 - MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-PROJECTION-POLICY-001
```

## Purpose

Resolve the `BoxFieldInitialization` subcluster selected after the
StatementValueConstruction block termination predicate descriptor.

The selected surfaces compose new-box construction with field initializer
assignment:

```text
build_new_expression_with_field_initializers(class, arguments, field_initializers)
build_box_field_initializers(object_value, class, field_initializers)
```

This is not a direct projection surface yet. It rejects record constructors,
creates the destination box, checks duplicate and unknown initializer fields,
and delegates each initializer to `build_field_assignment_from_value`. The
mutation frame must be contracted before selecting any HakoShadow projector,
native seed, or adoption decision.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_statement_value_construction_box_field_initialization_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-statement-value-construction-box-field-initialization-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_statement_value_construction_box_field_initialization_projection_policy_guard.sh
```

## Decision

```text
selected_policy = MutationFrameContractRequired
projection_surface_selected = 0

next_card =
  MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-MUTATION-FRAME-CONTRACT-001
```

## Evidence

```text
source_count = 2

mutation frame evidence:
  record_constructor_field_initializers_rejected = 1
  new_box_value_created_before_field_initializers = 1
  field_initializer_loop_detected = 1
  duplicate_field_guard_detected = 1
  user_defined_box_field_membership_guard_detected = 1
  field_assignment_delegation_detected = 1
  object_field_state_mutated_by_delegate = 1
```

## Acceptance

```text
selected_policy = MutationFrameContractRequired
projection_surface_selected = 0
mutation_frame_contract_required = 1
selected_next_card =
  MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-MUTATION-FRAME-CONTRACT-001

manual_family_selection = 0
hako_generation = 0
hako_shadow_projector_selected = 0
hako_adopted_decision = 0
native_seed_materialization = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Non-Claims

```text
no Hako generation
no HakoShadow projector selection
no HakoAdopted decision
no native source seed
no Source Selfhost claim
no route repair
```
