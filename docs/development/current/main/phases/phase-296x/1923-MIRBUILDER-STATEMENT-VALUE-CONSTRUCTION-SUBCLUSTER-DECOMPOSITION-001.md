# 1923 - MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-SUBCLUSTER-DECOMPOSITION-001

## Token

```text
MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-SUBCLUSTER-DECOMPOSITION-001
```

## Purpose

Decompose the selected `StatementValueConstructionCluster` before any
projection policy is selected.

The selected cluster contains diagnostic string helpers, block predicates,
box field initialization, record value construction, free-variable collection,
and lexical-scope stack mutation. These are not one projection owner.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_statement_value_construction_subcluster_decomposition.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-statement-value-construction-subcluster-decomposition-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_statement_value_construction_subcluster_decomposition_guard.sh
```

## Subclusters

```text
DiagnosticStringHelpers:
  undefined_variable_message

BlockTerminationPredicate:
  is_current_block_terminated

BoxFieldInitialization:
  build_new_expression_with_field_initializers
  build_box_field_initializers

RecordValueConstruction:
  is_record_constructor_class
  build_record_literal_value
  build_record_update_value

FreeVariableCollection:
  collect_free_vars

LexicalScopeStack:
  push_lexical_scope
  pop_lexical_scope
```

## Decision

```text
kind = SelectSubclusterProjectionPolicy
selected_subcluster = DiagnosticStringHelpers

next_card =
  MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-DIAGNOSTIC-HELPERS-PROJECTION-POLICY-001
```

## Acceptance

```text
source_count = 10
subcluster_count = 6
whole_cluster_projection_policy = 0
whole_cluster_keep_parent_owner = 0
manual_family_selection = 0
hako_generation = 0
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
no Hako projection selected
no Hako emitted
no HakoAdopted decision
no native source seed materialization
no Source Selfhost claim
```
