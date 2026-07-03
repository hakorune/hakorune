# 2144 - MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-009

## Token

```text
MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-009
```

## Purpose

Select the tenth small hand-authored `.hako` native owner parity pilot after
the `closure_call_shape_classifier` adoption.

This card selects only a pilot target. It does not adopt new `.hako` code and
does not claim Source Selfhost.

## Selected Pilot

```text
selected_owner:
  region_ref_slot_kind_classifier

selected_rust_surface:
  src/mir/region/mod.rs MirType -> RefSlotKind classifier

selected_next_card:
  MIRBUILDER-REGION-REF-SLOT-KIND-CLASSIFIER-RUST-ORACLE-FIXTURE-001
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
  SelectRustOracleFixture

reason_token:
  RegionRefSlotKindClassifierSelectedAsTenthParityPilot

selected_next_card:
  MIRBUILDER-REGION-REF-SLOT-KIND-CLASSIFIER-RUST-ORACLE-FIXTURE-001
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
