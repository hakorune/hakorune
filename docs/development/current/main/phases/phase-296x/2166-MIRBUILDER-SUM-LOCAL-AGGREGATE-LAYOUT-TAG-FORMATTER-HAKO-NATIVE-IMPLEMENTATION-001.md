# 2166 - MIRBUILDER-SUM-LOCAL-AGGREGATE-LAYOUT-TAG-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001

## Token

```text
MIRBUILDER-SUM-LOCAL-AGGREGATE-LAYOUT-TAG-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Purpose

Add the hand-authored `.hako` implementation for the
`sum_local_aggregate_layout_tag_formatter` narrow parity pilot.

## Implementation

```text
lang/src/compiler/lib/sum_local_aggregate_layout_tag_formatter.hako
```

## Included Surface

```text
SumLocalAggregateLayout Display text
```

## Excluded Surface

```text
payload-type layout binding
sum placement layout refresh
layout summary formatting
lowering execution
MIR mutation
```

## Decision

```text
decision:
  SelectParityGate

reason_token:
  SumLocalAggregateLayoutTagFormatterHakoImplementationAdded

selected_next_card:
  MIRBUILDER-SUM-LOCAL-AGGREGATE-LAYOUT-TAG-FORMATTER-PARITY-GATE-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision
no generated artifact edit authority
no payload-type layout binding migration
no sum placement layout refresh migration
no lowering execution migration
no MIR mutation migration
```
