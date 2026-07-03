# 2151 - MIRBUILDER-LOOP-ROUTE-KIND-LABEL-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001

## Token

```text
MIRBUILDER-LOOP-ROUTE-KIND-LABEL-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Purpose

Add the hand-authored `.hako` implementation for the
`loop_route_kind_label_formatter` narrow parity pilot.

## Implementation

```text
lang/src/compiler/lib/loop_route_kind_label_formatter.hako
```

## Included Surface

```text
LoopRouteKind name
LoopRouteKind semantic_label
LoopRouteKind pattern_id
LoopRouteKind is_recognized
LoopRouteKind has_special_control_flow
LoopRouteKind has_phi_merge
```

## Excluded Surface

```text
loop feature extraction
loop route classification
planner route selection
lowering execution
MIR mutation
```

## Decision

```text
decision:
  SelectParityGate

reason_token:
  LoopRouteKindLabelFormatterHakoImplementationAdded

selected_next_card:
  MIRBUILDER-LOOP-ROUTE-KIND-LABEL-FORMATTER-PARITY-GATE-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision
no generated artifact edit authority
no loop feature extraction migration
no route classification migration
no planner route selection migration
no lowering execution migration
no MIR mutation migration
```
