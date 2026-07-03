# 2173 - MIRBUILDER-STRING-KERNEL-PLAN-LABEL-FORMATTER-HAKO-ADOPTION-DECISION-001

## Token

```text
MIRBUILDER-STRING-KERNEL-PLAN-LABEL-FORMATTER-HAKO-ADOPTION-DECISION-001
```

## Purpose

Adopt `string_kernel_plan_label_formatter` as the fifteenth narrow
Rust-oracle parity pilot owner after a green 12-row `.hako` EXE parity gate.

This decision adopts only pure `StringKernelPlan*` Display label formatting. It
does not adopt string-kernel plan construction, legality analysis, publication
logic, backend lowering, MIR mutation, Source Selfhost, or full MirBuilder
conversion.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-string-kernel-plan-label-formatter-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/string_kernel_plan_label_formatter.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_string_kernel_plan_label_formatter_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-string-kernel-plan-label-formatter-hako-adoption-decision-v0.json
```

## Acceptance

```text
parity_gate = green
parity_rows = 12
decision = Adopt
hako_adopted = 1
rust_bootstrap_retained = 1
rust_oracle_retained = 1

source_selfhost_claim = 0
rust_deletion = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
generated_artifact_as_native_edit_authority = 0
string_kernel_plan_construction_migration = 0
string_kernel_legality_analysis_migration = 0
publication_logic_migration = 0
backend_lowering_migration = 0
mir_mutation_migration = 0
```

## Decision

```text
decision:
  Adopt

reason_token:
  StringKernelPlanLabelFormatterRustOracleParityGateGreen

selected_next_card:
  MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-015
```

## Non-Claims

```text
no Source Selfhost claim
no Rust deletion
no runtime fallback
no new backend route
no new ABI
no generated artifact edit authority
no string-kernel plan construction migration
no string-kernel legality analysis migration
no publication logic migration
no backend lowering migration
no MIR mutation migration
```
