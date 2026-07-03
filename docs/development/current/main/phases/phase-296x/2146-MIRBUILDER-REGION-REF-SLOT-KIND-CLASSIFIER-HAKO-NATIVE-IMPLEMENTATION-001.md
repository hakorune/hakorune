# 2146 - MIRBUILDER-REGION-REF-SLOT-KIND-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001

## Token

```text
MIRBUILDER-REGION-REF-SLOT-KIND-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Purpose

Add the hand-authored `.hako` implementation for the
`region_ref_slot_kind_classifier` narrow parity pilot.

## Implementation

```text
lang/src/compiler/lib/region_ref_slot_kind_classifier.hako
```

## Included Surface

```text
MirType -> RefSlotKind
```

## Excluded Surface

```text
Region construction
slot metadata collection
GC retain/release insertion
Region trace/log emission
MIR mutation
```

## Decision

```text
decision:
  SelectParityGate

reason_token:
  RegionRefSlotKindClassifierHakoImplementationAdded

selected_next_card:
  MIRBUILDER-REGION-REF-SLOT-KIND-CLASSIFIER-PARITY-GATE-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision
no generated artifact edit authority
no Region construction migration
no slot metadata collection migration
no GC retain/release migration
no Region trace/log migration
no MIR mutation migration
```
