# 2142 - MIRBUILDER-CLOSURE-CALL-SHAPE-CLASSIFIER-PARITY-GATE-001

## Token

```text
MIRBUILDER-CLOSURE-CALL-SHAPE-CLASSIFIER-PARITY-GATE-001
```

## Purpose

Add the Rust-oracle parity gate for the hand-authored
`closure_call_shape_classifier` `.hako` implementation.

## Gate

```text
tools/checks/rust_lifecycle_mirbuilder_closure_call_shape_classifier_parity_gate.sh
```

## Acceptance

```text
output_contract =
  rust-lifecycle-mirbuilder-closure-call-shape-classifier-parity-gate-v0

parity_rows = 4
parity_status = green
source_selfhost_claim = 0
hako_adopted_decision = 0
```

## Decision

```text
decision:
  SelectHakoAdoptionDecision

reason_token:
  ClosureCallShapeClassifierParityGateGreen

selected_next_card:
  MIRBUILDER-CLOSURE-CALL-SHAPE-CLASSIFIER-HAKO-ADOPTION-DECISION-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision in this card
no generated artifact edit authority
no callsite canonicalization migration
no NewClosure rewrite migration
no backend fail-fast boundary migration
no MIR instruction mutation migration
```
