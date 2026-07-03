# 2120 - MIRBUILDER-SAME-MODULE-DEFINITION-KIND-FORMATTER-RUST-ORACLE-FIXTURE-001

## Token

```text
MIRBUILDER-SAME-MODULE-DEFINITION-KIND-FORMATTER-RUST-ORACLE-FIXTURE-001
```

## Purpose

Create the Rust-oracle fixture for the fifth narrow hand-authored `.hako`
native owner parity pilot: `same_module_definition_kind_formatter`.

The fixture captures only the pure enum formatter:

```text
SameModuleDefinitionKind -> JSON name
```

It does not capture same-module definition closure collection, route traversal,
module mutation, backend C shim emission, Source Selfhost, or full MirBuilder
conversion.

## Evidence

```text
rust_source:
  src/mir/same_module_definition_plan.rs

oracle_function:
  SameModuleDefinitionKind::as_json_name

rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-same-module-definition-kind-formatter-rust-oracle-v0.json
```

## Fixture Rows

```text
Function -> same_module_function
LeafI64  -> leaf_i64_function
```

## Acceptance

```text
oracle_row_count = 2
selected_surface_is_pure_scalar_formatter = 1

source_selfhost_claim = 0
hako_adopted_decision = 0
same_module_definition_closure_migration = 0
global_call_route_traversal_migration = 0
user_box_method_route_traversal_migration = 0
mir_module_mutation_migration = 0
backend_c_shim_emission_migration = 0
```

## Decision

```text
decision:
  SelectHakoNativeImplementation

reason_token:
  SameModuleDefinitionKindFormatterRustOracleFixtureReady

selected_next_card:
  MIRBUILDER-SAME-MODULE-DEFINITION-KIND-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision
no generated artifact edit authority
no same-module definition closure migration
no MirModule mutation migration
no runtime fallback
no new backend route
no new ABI
```
