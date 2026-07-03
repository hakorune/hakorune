# 2172 - MIRBUILDER-STRING-KERNEL-PLAN-LABEL-FORMATTER-PARITY-GATE-001

## Token

```text
MIRBUILDER-STRING-KERNEL-PLAN-LABEL-FORMATTER-PARITY-GATE-001
```

## Purpose

Add and run the `.hako` EXE parity gate for
`string_kernel_plan_label_formatter`.

## Evidence

```text
parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_string_kernel_plan_label_formatter_parity_gate.sh
```

## Acceptance

```text
parity_gate = green
parity_rows = 12

source_selfhost_claim = 0
hako_adopted_decision = 0
string_kernel_plan_construction_migration = 0
string_kernel_legality_analysis_migration = 0
publication_logic_migration = 0
backend_lowering_migration = 0
mir_mutation_migration = 0
```

## Decision

```text
decision:
  SelectHakoAdoptionDecision

selected_next_card:
  MIRBUILDER-STRING-KERNEL-PLAN-LABEL-FORMATTER-HAKO-ADOPTION-DECISION-001
```
