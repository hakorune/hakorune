# 2170 - MIRBUILDER-STRING-KERNEL-PLAN-LABEL-FORMATTER-RUST-ORACLE-FIXTURE-001

## Token

```text
MIRBUILDER-STRING-KERNEL-PLAN-LABEL-FORMATTER-RUST-ORACLE-FIXTURE-001
```

## Purpose

Record the 12-row Rust-oracle fixture for
`string_kernel_plan_label_formatter`.

The fixture covers only Display label text for the narrow `StringKernelPlan*`
enum surfaces. It does not adopt plan construction, legality analysis,
publication logic, backend lowering, or MIR mutation.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-string-kernel-plan-label-formatter-rust-oracle-v0.json
```

## Acceptance

```text
fixture.kind = MirBuilderStringKernelPlanLabelFormatterRustOracleV1
row_count = 12

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
  SelectHakoNativeImplementation

selected_next_card:
  MIRBUILDER-STRING-KERNEL-PLAN-LABEL-FORMATTER-HAKO-NATIVE-IMPLEMENTATION-001
```
