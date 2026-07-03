# 2119 - MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-004

## Token

```text
MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-004
```

## Purpose

Select the fifth small hand-authored `.hako` native owner parity pilot after
the `string_corridor_name_vocabulary_classifier` adoption.

This card selects only a pilot target. It does not adopt new `.hako` code and
does not claim Source Selfhost.

## Selected Pilot

```text
selected_owner:
  same_module_definition_kind_formatter

selected_rust_surface:
  src/mir/same_module_definition_plan.rs SameModuleDefinitionKind -> JSON name

selected_next_card:
  MIRBUILDER-SAME-MODULE-DEFINITION-KIND-FORMATTER-RUST-ORACLE-FIXTURE-001
```

## Included Surface

```text
SameModuleDefinitionKind::Function
  -> same_module_function

SameModuleDefinitionKind::LeafI64
  -> leaf_i64_function
```

## Excluded Surface

```text
same-module definition closure collection
global call route traversal
user box method route traversal
MirModule mutation
backend C shim emission
```

## Evidence

```text
selection_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-hako-native-owner-parity-pilot-selection-rerun-004-v0.json

source_file:
  src/mir/same_module_definition_plan.rs
```

## Decision

```text
decision:
  SelectRustOracleFixture

reason_token:
  SameModuleDefinitionKindFormatterSelectedAsFifthParityPilot

selected_next_card:
  MIRBUILDER-SAME-MODULE-DEFINITION-KIND-FORMATTER-RUST-ORACLE-FIXTURE-001
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
