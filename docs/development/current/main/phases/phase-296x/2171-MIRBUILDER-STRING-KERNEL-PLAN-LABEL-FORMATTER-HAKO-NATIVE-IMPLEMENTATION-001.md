# 2171 - MIRBUILDER-STRING-KERNEL-PLAN-LABEL-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001

## Token

```text
MIRBUILDER-STRING-KERNEL-PLAN-LABEL-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Purpose

Add the hand-authored `.hako` implementation for
`string_kernel_plan_label_formatter`.

The implementation is a pure scalar label formatter. It is not generated from
Rust and is not native edit authority for broader MirBuilder behavior.

## Evidence

```text
hako_source:
  lang/src/compiler/lib/string_kernel_plan_label_formatter.hako
```

## Acceptance

```text
hako_source_exists = 1
generated_artifact_as_native_edit_authority = 0
source_selfhost_claim = 0
hako_adopted_decision = 0
string_kernel_plan_construction_migration = 0
backend_lowering_migration = 0
mir_mutation_migration = 0
```

## Decision

```text
decision:
  SelectParityGate

selected_next_card:
  MIRBUILDER-STRING-KERNEL-PLAN-LABEL-FORMATTER-PARITY-GATE-001
```
