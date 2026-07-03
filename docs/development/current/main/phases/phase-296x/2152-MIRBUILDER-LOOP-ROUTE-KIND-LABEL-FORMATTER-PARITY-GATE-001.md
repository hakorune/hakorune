# 2152 - MIRBUILDER-LOOP-ROUTE-KIND-LABEL-FORMATTER-PARITY-GATE-001

## Token

```text
MIRBUILDER-LOOP-ROUTE-KIND-LABEL-FORMATTER-PARITY-GATE-001
```

## Purpose

Add the Rust-oracle parity gate for the
`loop_route_kind_label_formatter` narrow parity pilot.

## Gate

```text
tools/checks/rust_lifecycle_mirbuilder_loop_route_kind_label_formatter_parity_gate.sh
```

## Acceptance

```bash
bash tools/checks/rust_lifecycle_mirbuilder_loop_route_kind_label_formatter_parity_gate.sh
```

Expected output contract:

```text
output_contract=rust-lifecycle-mirbuilder-loop-route-kind-label-formatter-parity-gate-v0
owner=loop_route_kind_label_formatter
parity_rows=7
parity_status=green
source_selfhost_claim=0
hako_adopted_decision=0
```

## Decision

```text
decision:
  SelectHakoAdoptionDecision

reason_token:
  LoopRouteKindLabelFormatterParityGateGreen

selected_next_card:
  MIRBUILDER-LOOP-ROUTE-KIND-LABEL-FORMATTER-HAKO-ADOPTION-DECISION-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision in this card
no generated artifact edit authority
no loop feature extraction migration
no route classification migration
no planner route selection migration
no lowering execution migration
no MIR mutation migration
```
