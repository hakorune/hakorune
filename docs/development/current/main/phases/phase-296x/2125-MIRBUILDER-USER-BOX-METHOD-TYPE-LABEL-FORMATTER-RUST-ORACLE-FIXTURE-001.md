# 2125 - MIRBUILDER-USER-BOX-METHOD-TYPE-LABEL-FORMATTER-RUST-ORACLE-FIXTURE-001

## Token

```text
MIRBUILDER-USER-BOX-METHOD-TYPE-LABEL-FORMATTER-RUST-ORACLE-FIXTURE-001
```

## Purpose

Create the Rust-oracle fixture for the sixth narrow hand-authored `.hako`
native owner parity pilot: `user_box_method_type_label_formatter`.

The fixture captures only the pure formatter:

```text
MirType -> user-box method route type label
```

It does not capture user-box method route collection, convergence,
materialization, module mutation, backend emission, Source Selfhost, or full
MirBuilder conversion.

## Evidence

```text
rust_source:
  src/mir/user_box_method_route_plan.rs

type_vocabulary_source:
  crates/hakorune_mir_core/src/types.rs

oracle_function:
  format_user_box_method_type_label

rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-user-box-method-type-label-formatter-rust-oracle-v0.json
```

## Fixture Rows

```text
Integer        -> i64
Bool           -> i1
Float          -> f64
String         -> StringBox
Void           -> void
Box(Counter)   -> Counter
Array(Integer) -> ArrayBox
Future(String) -> FutureBox
WeakRef        -> WeakRef
Unknown        -> unknown
```

## Acceptance

```text
oracle_row_count = 10
selected_surface_is_pure_scalar_formatter = 1

source_selfhost_claim = 0
hako_adopted_decision = 0
user_box_method_route_collection_migration = 0
route_convergence_migration = 0
materialization_fixpoint_migration = 0
mir_module_mutation_migration = 0
backend_emission_migration = 0
```

## Decision

```text
decision:
  SelectHakoNativeImplementation

reason_token:
  UserBoxMethodTypeLabelFormatterRustOracleFixtureReady

selected_next_card:
  MIRBUILDER-USER-BOX-METHOD-TYPE-LABEL-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision
no generated artifact edit authority
no user-box method route collection migration
no route convergence migration
no MirModule mutation migration
no runtime fallback
no new backend route
no new ABI
```
