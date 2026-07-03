# 2123 - MIRBUILDER-SAME-MODULE-DEFINITION-KIND-FORMATTER-HAKO-ADOPTION-DECISION-001

## Token

```text
MIRBUILDER-SAME-MODULE-DEFINITION-KIND-FORMATTER-HAKO-ADOPTION-DECISION-001
```

## Purpose

Adopt `same_module_definition_kind_formatter` as the fifth narrow Rust-oracle
parity pilot owner after a green 2-row `.hako` EXE parity gate.

This decision adopts only the pure enum formatter:

```text
SameModuleDefinitionKind -> JSON name
```

It does not adopt same-module definition closure collection, global call route
traversal, user box method route traversal, MirModule mutation, backend C shim
emission, Source Selfhost, or full MirBuilder conversion.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-same-module-definition-kind-formatter-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/same_module_definition_kind_formatter.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_same_module_definition_kind_formatter_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-same-module-definition-kind-formatter-hako-adoption-decision-v0.json
```

## Acceptance

```text
parity_gate = green
parity_rows = 2
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
same_module_definition_closure_migration = 0
global_call_route_traversal_migration = 0
user_box_method_route_traversal_migration = 0
mir_module_mutation_migration = 0
backend_c_shim_emission_migration = 0
```

## Decision

```text
decision:
  Adopt

reason_token:
  SameModuleDefinitionKindFormatterRustOracleParityGateGreen

selected_next_card:
  MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-005
```

## Non-Claims

```text
no Source Selfhost claim
no Rust deletion
no runtime fallback
no new backend route
no new ABI
no generated artifact edit authority
no same-module definition closure migration
no MirModule mutation migration
no backend C shim emission migration
```
