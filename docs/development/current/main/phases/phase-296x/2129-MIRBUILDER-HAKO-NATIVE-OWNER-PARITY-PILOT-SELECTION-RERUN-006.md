# 2129 - MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-006

## Token

```text
MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-006
```

## Purpose

Select the seventh small hand-authored `.hako` native owner parity pilot after
the `user_box_method_type_label_formatter` adoption.

This card selects only a pilot target. It does not adopt new `.hako` code and
does not claim Source Selfhost.

## Selected Pilot

```text
selected_owner:
  core_method_carrier_token_formatter

selected_rust_surface:
  src/mir/core_method_op.rs CoreMethodOp / lowering tier token formatters

selected_next_card:
  MIRBUILDER-CORE-METHOD-CARRIER-TOKEN-FORMATTER-RUST-ORACLE-FIXTURE-001
```

## Included Surface

```text
CoreMethodOp -> manifest token
CoreMethodOpProof -> proof token
CoreMethodLoweringTier -> manifest token
CoreMethodLoweringTier -> plan tier token
CoreMethodLoweringTier -> emit kind token
LoweringPlanTier -> JSON token
LoweringPlanEmitKind -> JSON token
```

## Excluded Surface

```text
CoreMethodContract manifest generation
method resolution
carrier route collection
lowering execution
backend emission
```

## Evidence

```text
selection_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-hako-native-owner-parity-pilot-selection-rerun-006-v0.json

source_file:
  src/mir/core_method_op.rs
```

## Decision

```text
decision:
  SelectRustOracleFixture

reason_token:
  CoreMethodCarrierTokenFormatterSelectedAsSeventhParityPilot

selected_next_card:
  MIRBUILDER-CORE-METHOD-CARRIER-TOKEN-FORMATTER-RUST-ORACLE-FIXTURE-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision
no generated artifact edit authority
no CoreMethodContract manifest migration
no method resolution migration
no carrier route collection migration
no lowering execution migration
no backend emission migration
no runtime fallback
no new backend route
no new ABI
```
