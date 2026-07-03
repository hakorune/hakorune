# 2145 - MIRBUILDER-REGION-REF-SLOT-KIND-CLASSIFIER-RUST-ORACLE-FIXTURE-001

## Token

```text
MIRBUILDER-REGION-REF-SLOT-KIND-CLASSIFIER-RUST-ORACLE-FIXTURE-001
```

## Purpose

Create the Rust-oracle fixture for the tenth narrow hand-authored `.hako`
native owner parity pilot: `region_ref_slot_kind_classifier`.

## Fixture

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-region-ref-slot-kind-classifier-rust-oracle-v0.json
```

## Oracle Surface

```text
MirType -> RefSlotKind
```

## Acceptance

```text
oracle_row_count = 10
selected_surface_is_pure_type_classifier = 1
source_selfhost_claim = 0
hako_adopted_decision = 0
```

## Decision

```text
decision:
  SelectHakoNativeImplementation

reason_token:
  RegionRefSlotKindClassifierRustOracleFixtureCreated

selected_next_card:
  MIRBUILDER-REGION-REF-SLOT-KIND-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001
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
