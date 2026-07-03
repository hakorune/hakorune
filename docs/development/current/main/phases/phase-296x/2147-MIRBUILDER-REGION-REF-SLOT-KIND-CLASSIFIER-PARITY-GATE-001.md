# 2147 - MIRBUILDER-REGION-REF-SLOT-KIND-CLASSIFIER-PARITY-GATE-001

## Token

```text
MIRBUILDER-REGION-REF-SLOT-KIND-CLASSIFIER-PARITY-GATE-001
```

## Purpose

Add the Rust-oracle parity gate for the hand-authored
`region_ref_slot_kind_classifier` `.hako` implementation.

## Gate

```text
tools/checks/rust_lifecycle_mirbuilder_region_ref_slot_kind_classifier_parity_gate.sh
```

## Acceptance

```text
output_contract =
  rust-lifecycle-mirbuilder-region-ref-slot-kind-classifier-parity-gate-v0

parity_rows = 10
parity_status = green
source_selfhost_claim = 0
hako_adopted_decision = 0
```

## Decision

```text
decision:
  SelectHakoAdoptionDecision

reason_token:
  RegionRefSlotKindClassifierParityGateGreen

selected_next_card:
  MIRBUILDER-REGION-REF-SLOT-KIND-CLASSIFIER-HAKO-ADOPTION-DECISION-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision in this card
no generated artifact edit authority
no Region construction migration
no slot metadata collection migration
no GC retain/release migration
no Region trace/log migration
no MIR mutation migration
```
