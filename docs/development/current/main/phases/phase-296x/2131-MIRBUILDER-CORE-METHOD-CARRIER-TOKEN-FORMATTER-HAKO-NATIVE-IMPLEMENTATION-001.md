# 2131 - MIRBUILDER-CORE-METHOD-CARRIER-TOKEN-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001

## Token

```text
MIRBUILDER-CORE-METHOD-CARRIER-TOKEN-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Purpose

Add the hand-authored `.hako` implementation for the
`core_method_carrier_token_formatter` narrow parity pilot.

## Implementation

```text
lang/src/compiler/lib/core_method_carrier_token_formatter.hako
```

## Included Surface

```text
CoreMethodOp token formatting
CoreMethodOpProof token formatting
CoreMethodLoweringTier manifest/plan/emit token formatting
LoweringPlanTier JSON token formatting
LoweringPlanEmitKind JSON token formatting
```

## Excluded Surface

```text
CoreMethodContract manifest generation
method resolution
carrier route collection
lowering execution
backend emission
```

## Decision

```text
decision:
  SelectParityGate

reason_token:
  CoreMethodCarrierTokenFormatterHakoImplementationAdded

selected_next_card:
  MIRBUILDER-CORE-METHOD-CARRIER-TOKEN-FORMATTER-PARITY-GATE-001
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
