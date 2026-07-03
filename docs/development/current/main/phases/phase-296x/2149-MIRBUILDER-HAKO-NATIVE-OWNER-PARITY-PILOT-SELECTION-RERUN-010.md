# 2149 - MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-010

## Token

```text
MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-010
```

## Purpose

Select the eleventh small hand-authored `.hako` native owner parity pilot after
the `region_ref_slot_kind_classifier` adoption.

This card selects only a pilot target. It does not adopt new `.hako` code and
does not claim Source Selfhost.

## Selected Pilot

```text
selected_owner:
  loop_route_kind_label_formatter

selected_rust_surface:
  src/mir/loop_route_detection/kind.rs LoopRouteKind label/id/flag formatter

selected_next_card:
  MIRBUILDER-LOOP-ROUTE-KIND-LABEL-FORMATTER-RUST-ORACLE-FIXTURE-001
```

## Included Surface

```text
LoopRouteKind -> name
LoopRouteKind -> semantic_label
LoopRouteKind -> pattern_id
LoopRouteKind -> is_recognized / has_special_control_flow / has_phi_merge
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
  SelectRustOracleFixture

reason_token:
  LoopRouteKindLabelFormatterSelectedAsEleventhParityPilot

selected_next_card:
  MIRBUILDER-LOOP-ROUTE-KIND-LABEL-FORMATTER-RUST-ORACLE-FIXTURE-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision
no generated artifact edit authority
no loop feature extraction migration
no loop route classification migration
no planner route selection migration
no lowering execution migration
no MIR mutation migration
```
