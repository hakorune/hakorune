# 2130 - MIRBUILDER-CORE-METHOD-CARRIER-TOKEN-FORMATTER-RUST-ORACLE-FIXTURE-001

## Token

```text
MIRBUILDER-CORE-METHOD-CARRIER-TOKEN-FORMATTER-RUST-ORACLE-FIXTURE-001
```

## Purpose

Create the Rust-oracle fixture for the seventh narrow hand-authored `.hako`
native owner parity pilot: `core_method_carrier_token_formatter`.

## Fixture

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-core-method-carrier-token-formatter-rust-oracle-v0.json
```

## Oracle Surface

```text
CoreMethodOp -> manifest token
CoreMethodOpProof -> proof token
CoreMethodLoweringTier -> manifest token
CoreMethodLoweringTier -> plan tier token
CoreMethodLoweringTier -> emit kind token
LoweringPlanTier -> JSON token
LoweringPlanEmitKind -> JSON token
```

## Acceptance

```text
oracle_row_count = 32
selected_surface_is_pure_token_formatter = 1
source_selfhost_claim = 0
hako_adopted_decision = 0

core_method_contract_manifest_migration = 0
method_resolution_migration = 0
route_collection_migration = 0
lowering_execution_migration = 0
backend_emission_migration = 0
```

## Decision

```text
decision:
  SelectHakoNativeImplementation

reason_token:
  CoreMethodCarrierTokenFormatterRustOracleFixtureCreated

selected_next_card:
  MIRBUILDER-CORE-METHOD-CARRIER-TOKEN-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision
no generated artifact edit authority
no manifest migration
no method resolution migration
no carrier route collection migration
no lowering execution migration
no backend emission migration
```
