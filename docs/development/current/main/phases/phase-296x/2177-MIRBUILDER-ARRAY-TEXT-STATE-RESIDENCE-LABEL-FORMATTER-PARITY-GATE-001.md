# 2177 - MIRBUILDER-ARRAY-TEXT-STATE-RESIDENCE-LABEL-FORMATTER-PARITY-GATE-001

## Token

```text
MIRBUILDER-ARRAY-TEXT-STATE-RESIDENCE-LABEL-FORMATTER-PARITY-GATE-001
```

## Purpose

Add and run the `.hako` EXE parity gate for
`array_text_state_residence_label_formatter`.

## Evidence

```text
parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_array_text_state_residence_label_formatter_parity_gate.sh
```

## Acceptance

```text
parity_gate = green
parity_rows = 5

source_selfhost_claim = 0
hako_adopted_decision = 0
array_text_route_matching_migration = 0
exact_shape_payload_migration = 0
executor_planning_migration = 0
backend_lowering_migration = 0
mir_mutation_migration = 0
```

## Decision

```text
decision:
  SelectHakoAdoptionDecision

selected_next_card:
  MIRBUILDER-ARRAY-TEXT-STATE-RESIDENCE-LABEL-FORMATTER-HAKO-ADOPTION-DECISION-001
```
