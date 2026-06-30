# 1927 - MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-MUTATION-FRAME-CONTRACT-001

## Token

```text
MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-MUTATION-FRAME-CONTRACT-001
```

## Purpose

Fix the mutation-frame contract for StatementValueConstruction box field
initialization before any HakoShadow projector or native source seed is
selected.

The contract covers:

```text
build_new_expression_with_field_initializers
build_box_field_initializers
```

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_statement_value_construction_box_field_initialization_mutation_frame_contract.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-statement-value-construction-box-field-initialization-mutation-frame-contract-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_statement_value_construction_box_field_initialization_mutation_frame_contract_guard.sh
```

## Contract

```text
state_inputs:
  class
  arguments
  field_initializers
  MirBuilder.comp_ctx.user_defined_boxes
  MirBuilder.current_function_state
  MirBuilder.type_ctx

state_outputs:
  dst ValueId
  object field assignments through build_field_assignment_from_value
  MirBuilder.current_function_state
  MirBuilder.type_ctx

read_only_inputs:
  record constructor classifier
  MirBuilder.comp_ctx.user_defined_boxes

local_only_state:
  seen initializer field set

mutation_order:
  RejectRecordConstructorFieldInitializers
  CreateDestinationBox
  InitializeSeenFieldSet
  RejectDuplicateInitializerField
  ValidateUserDefinedBoxFieldMembership
  DelegateFieldAssignmentForInitializer
  ReturnDestinationValue
```

## Decision

```text
decision = SelectHakoShadowParity
selected_next_card =
  MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BOX-FIELD-INITIALIZATION-HAKO-SHADOW-PARITY-001
```

## Acceptance

```text
mutation_frame_contract_ready = 1
delegated_mutation_owner = build_field_assignment_from_value
record_constructor_field_initializers_rejected = 1
new_box_value_created_before_field_initializers = 1
duplicate_field_guard_before_assignment = 1
unknown_field_guard_before_assignment = 1
field_assignment_delegation_detected = 1

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
