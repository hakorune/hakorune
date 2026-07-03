# 2136 - MIRBUILDER-GENERIC-METHOD-ROUTE-FACT-TOKEN-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001

## Token

```text
MIRBUILDER-GENERIC-METHOD-ROUTE-FACT-TOKEN-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Purpose

Add the hand-authored `.hako` implementation for the
`generic_method_route_fact_token_formatter` narrow parity pilot.

## Implementation

```text
lang/src/compiler/lib/generic_method_route_fact_token_formatter.hako
```

## Included Surface

```text
GenericMethodKeyRoute token formatting
GenericMethodValueDemand token formatting
GenericMethodReturnShape token formatting
GenericMethodPublicationPolicy token formatting
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
  SelectParityGate

reason_token:
  GenericMethodRouteFactTokenFormatterHakoImplementationAdded

selected_next_card:
  MIRBUILDER-GENERIC-METHOD-ROUTE-FACT-TOKEN-FORMATTER-PARITY-GATE-001
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
