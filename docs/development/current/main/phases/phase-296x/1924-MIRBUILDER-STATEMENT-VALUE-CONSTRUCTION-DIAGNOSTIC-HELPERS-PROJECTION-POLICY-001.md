# 1924 - MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-DIAGNOSTIC-HELPERS-PROJECTION-POLICY-001

## Token

```text
MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-DIAGNOSTIC-HELPERS-PROJECTION-POLICY-001
```

## Purpose

Resolve the `DiagnosticStringHelpers` subcluster selected by the
StatementValueConstruction subcluster decomposition.

The selected helper is diagnostic-only message construction:

```text
undefined_variable_message(name) -> String
```

It does not own statement value construction, routing, registry selection,
runtime fallback, or Hako projection policy. It remains parent-owned under
StatementValueConstruction diagnostics.

## Output

```text
tool:
  tools/rust_lifecycle/
    mirbuilder_statement_value_construction_diagnostic_helpers_projection_policy.py

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-statement-value-construction-diagnostic-helpers-projection-policy-v0.json

guard:
  tools/checks/
    rust_lifecycle_mirbuilder_statement_value_construction_diagnostic_helpers_projection_policy_guard.sh
```

## Decision

```text
policy = KeepParentOwner
owner_edge = mirbuilder::statement_value_construction_diagnostic_helpers
projection_surface_selected = 0
registry_descriptor_selected = 0

next_card =
  MIRBUILDER-STATEMENT-VALUE-CONSTRUCTION-BLOCK-TERMINATION-PREDICATE-PROJECTION-POLICY-001
```

## Evidence

```text
source_count = 1

roles:
  undefined_variable_diagnostic_message = 1

markers:
  Undefined variable:
  Stage-3 keyword
  parser_stage3_enabled()
  suggest_using_for_symbol(name)
  symbol appears in using module(s)
  Consider adding 'using <module> [as Alias]'
```

## Acceptance

```text
policy = KeepParentOwner
projection_surface_selected = 0
registry_descriptor_selected = 0
runtime_or_projection_policy_by_name = 0
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
no standalone Hako projection surface
no registry descriptor fixture
no Hako generation
no HakoAdopted decision
no native seed materialization
no Source Selfhost claim
```
