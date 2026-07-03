# 2134 - MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-007

## Token

```text
MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-007
```

## Purpose

Select the eighth small hand-authored `.hako` native owner parity pilot after
the `core_method_carrier_token_formatter` adoption.

This card selects only a pilot target. It does not adopt new `.hako` code and
does not claim Source Selfhost.

## Selected Pilot

```text
selected_owner:
  generic_method_route_fact_token_formatter

selected_rust_surface:
  src/mir/generic_method_route_facts.rs generic method route fact enum tokens

selected_next_card:
  MIRBUILDER-GENERIC-METHOD-ROUTE-FACT-TOKEN-FORMATTER-RUST-ORACLE-FIXTURE-001
```

## Included Surface

```text
GenericMethodKeyRoute -> metadata token
GenericMethodValueDemand -> metadata token
GenericMethodReturnShape -> metadata token
GenericMethodPublicationPolicy -> metadata token
```

## Excluded Surface

```text
receiver origin resolution
key route classification
const i64 extraction
generic method route planning
backend emission
```

## Decision

```text
decision:
  SelectRustOracleFixture

reason_token:
  GenericMethodRouteFactTokenFormatterSelectedAsEighthParityPilot

selected_next_card:
  MIRBUILDER-GENERIC-METHOD-ROUTE-FACT-TOKEN-FORMATTER-RUST-ORACLE-FIXTURE-001
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
no runtime fallback
no new backend route
no new ABI
```
