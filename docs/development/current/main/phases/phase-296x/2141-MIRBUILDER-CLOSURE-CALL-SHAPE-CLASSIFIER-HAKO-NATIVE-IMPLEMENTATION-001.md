# 2141 - MIRBUILDER-CLOSURE-CALL-SHAPE-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001

## Token

```text
MIRBUILDER-CLOSURE-CALL-SHAPE-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Purpose

Add the hand-authored `.hako` implementation for the
`closure_call_shape_classifier` narrow parity pilot.

## Implementation

```text
lang/src/compiler/lib/closure_call_shape_classifier.hako
```

## Included Surface

```text
dst_present + arg_count -> ClosureCallShape
ClosureCallShape -> reject code
```

## Excluded Surface

```text
callsite canonicalization
NewClosure rewrite
backend fail-fast boundary
MIR instruction mutation
```

## Decision

```text
decision:
  SelectParityGate

reason_token:
  ClosureCallShapeClassifierHakoImplementationAdded

selected_next_card:
  MIRBUILDER-CLOSURE-CALL-SHAPE-CLASSIFIER-PARITY-GATE-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision
no generated artifact edit authority
no callsite canonicalization migration
no NewClosure rewrite migration
no backend fail-fast boundary migration
no MIR instruction mutation migration
```
