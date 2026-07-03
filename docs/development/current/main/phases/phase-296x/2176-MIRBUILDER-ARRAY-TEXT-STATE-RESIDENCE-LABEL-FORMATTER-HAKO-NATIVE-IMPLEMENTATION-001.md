# 2176 - MIRBUILDER-ARRAY-TEXT-STATE-RESIDENCE-LABEL-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001

## Token

```text
MIRBUILDER-ARRAY-TEXT-STATE-RESIDENCE-LABEL-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Purpose

Add the hand-authored `.hako` implementation for
`array_text_state_residence_label_formatter`.

The implementation is a pure scalar label formatter. It is not generated from
Rust and is not native edit authority for broader MirBuilder behavior.

## Evidence

```text
hako_source:
  lang/src/compiler/lib/array_text_state_residence_label_formatter.hako
```

## Acceptance

```text
hako_source_exists = 1
generated_artifact_as_native_edit_authority = 0
source_selfhost_claim = 0
hako_adopted_decision = 0
array_text_route_matching_migration = 0
executor_planning_migration = 0
backend_lowering_migration = 0
mir_mutation_migration = 0
```

## Decision

```text
decision:
  SelectParityGate

selected_next_card:
  MIRBUILDER-ARRAY-TEXT-STATE-RESIDENCE-LABEL-FORMATTER-PARITY-GATE-001
```
