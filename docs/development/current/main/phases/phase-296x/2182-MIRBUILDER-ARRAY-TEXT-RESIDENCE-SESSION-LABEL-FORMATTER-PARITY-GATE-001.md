# 2182 - MIRBUILDER-ARRAY-TEXT-RESIDENCE-SESSION-LABEL-FORMATTER-PARITY-GATE-001

## Token

```text
MIRBUILDER-ARRAY-TEXT-RESIDENCE-SESSION-LABEL-FORMATTER-PARITY-GATE-001
```

## Purpose

Add and run the `.hako` EXE parity gate for
`array_text_residence_session_label_formatter`.

## Evidence

```text
parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_array_text_residence_session_label_formatter_parity_gate.sh
```

## Acceptance

```text
parity_gate = green
parity_rows = 12
source_selfhost_claim = 0
hako_adopted_decision = 0
session_derivation_migration = 0
region_mapping_migration = 0
executor_plan_assembly_migration = 0
backend_lowering_migration = 0
mir_mutation_migration = 0
```

## Decision

```text
selected_next_card:
  MIRBUILDER-ARRAY-TEXT-RESIDENCE-SESSION-LABEL-FORMATTER-HAKO-ADOPTION-DECISION-001
```
