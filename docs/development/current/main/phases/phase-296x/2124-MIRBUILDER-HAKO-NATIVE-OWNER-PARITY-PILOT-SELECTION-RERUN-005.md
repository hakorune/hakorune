# 2124 - MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-005

## Token

```text
MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-005
```

## Purpose

Select the sixth small hand-authored `.hako` native owner parity pilot after
the `same_module_definition_kind_formatter` adoption.

This card selects only a pilot target. It does not adopt new `.hako` code and
does not claim Source Selfhost.

## Selected Pilot

```text
selected_owner:
  user_box_method_type_label_formatter

selected_rust_surface:
  src/mir/user_box_method_route_plan.rs MirType -> user-box method route type label

selected_next_card:
  MIRBUILDER-USER-BOX-METHOD-TYPE-LABEL-FORMATTER-RUST-ORACLE-FIXTURE-001
```

## Included Surface

```text
MirType::Integer -> i64
MirType::Bool -> i1
MirType::Float -> f64
MirType::String -> StringBox
MirType::Void -> void
MirType::Box(name) -> name
MirType::Array(_) -> ArrayBox
MirType::Future(_) -> FutureBox
MirType::WeakRef -> WeakRef
MirType::Unknown -> unknown
```

## Excluded Surface

```text
user-box method route collection
route convergence
materialization fixpoint
MirModule mutation
backend emission
```

## Evidence

```text
selection_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-hako-native-owner-parity-pilot-selection-rerun-005-v0.json

source_file:
  src/mir/user_box_method_route_plan.rs
```

## Decision

```text
decision:
  SelectRustOracleFixture

reason_token:
  UserBoxMethodTypeLabelFormatterSelectedAsSixthParityPilot

selected_next_card:
  MIRBUILDER-USER-BOX-METHOD-TYPE-LABEL-FORMATTER-RUST-ORACLE-FIXTURE-001
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
