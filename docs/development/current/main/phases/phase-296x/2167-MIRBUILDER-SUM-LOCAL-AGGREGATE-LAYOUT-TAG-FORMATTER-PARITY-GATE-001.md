# 2167 - MIRBUILDER-SUM-LOCAL-AGGREGATE-LAYOUT-TAG-FORMATTER-PARITY-GATE-001

## Token

```text
MIRBUILDER-SUM-LOCAL-AGGREGATE-LAYOUT-TAG-FORMATTER-PARITY-GATE-001
```

## Purpose

Add the Rust-oracle parity gate for the
`sum_local_aggregate_layout_tag_formatter` narrow parity pilot.

## Gate

```text
tools/checks/rust_lifecycle_mirbuilder_sum_local_aggregate_layout_tag_formatter_parity_gate.sh
```

## Acceptance

```bash
bash tools/checks/rust_lifecycle_mirbuilder_sum_local_aggregate_layout_tag_formatter_parity_gate.sh
```

Expected output contract:

```text
output_contract=rust-lifecycle-mirbuilder-sum-local-aggregate-layout-tag-formatter-parity-gate-v0
owner=sum_local_aggregate_layout_tag_formatter
parity_rows=4
parity_status=green
source_selfhost_claim=0
hako_adopted_decision=0
```

## Decision

```text
decision:
  SelectHakoAdoptionDecision

reason_token:
  SumLocalAggregateLayoutTagFormatterParityGateGreen

selected_next_card:
  MIRBUILDER-SUM-LOCAL-AGGREGATE-LAYOUT-TAG-FORMATTER-HAKO-ADOPTION-DECISION-001
```

## Non-Claims

```text
no Source Selfhost claim
no Hako adoption decision in this card
no generated artifact edit authority
no payload-type layout binding migration
no sum placement layout refresh migration
no lowering execution migration
no MIR mutation migration
```
