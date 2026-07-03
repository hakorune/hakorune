# 2135 - MIRBUILDER-GENERIC-METHOD-ROUTE-FACT-TOKEN-FORMATTER-RUST-ORACLE-FIXTURE-001

## Token

```text
MIRBUILDER-GENERIC-METHOD-ROUTE-FACT-TOKEN-FORMATTER-RUST-ORACLE-FIXTURE-001
```

## Purpose

Create the Rust-oracle fixture for the eighth narrow hand-authored `.hako`
native owner parity pilot: `generic_method_route_fact_token_formatter`.

## Fixture

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-generic-method-route-fact-token-formatter-rust-oracle-v0.json
```

## Oracle Surface

```text
GenericMethodKeyRoute -> metadata token
GenericMethodValueDemand -> metadata token
GenericMethodReturnShape -> metadata token
GenericMethodPublicationPolicy -> metadata token
```

## Acceptance

```text
oracle_row_count = 12
selected_surface_is_pure_token_formatter = 1
source_selfhost_claim = 0
hako_adopted_decision = 0
```

## Decision

```text
decision:
  SelectHakoNativeImplementation

reason_token:
  GenericMethodRouteFactTokenFormatterRustOracleFixtureCreated

selected_next_card:
  MIRBUILDER-GENERIC-METHOD-ROUTE-FACT-TOKEN-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision
no generated artifact edit authority
no receiver origin resolution migration
no key route classification migration
no const i64 extraction migration
no generic method route planning migration
no backend emission migration
```
