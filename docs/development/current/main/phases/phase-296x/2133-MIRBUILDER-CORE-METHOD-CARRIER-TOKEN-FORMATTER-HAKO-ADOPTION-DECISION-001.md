# 2133 - MIRBUILDER-CORE-METHOD-CARRIER-TOKEN-FORMATTER-HAKO-ADOPTION-DECISION-001

## Token

```text
MIRBUILDER-CORE-METHOD-CARRIER-TOKEN-FORMATTER-HAKO-ADOPTION-DECISION-001
```

## Purpose

Adopt `core_method_carrier_token_formatter` as the seventh narrow Rust-oracle
parity pilot owner after a green 32-row `.hako` EXE parity gate.

This decision adopts only the pure token formatter surface. It does not adopt
method resolution, route collection, lowering execution, backend emission,
Source Selfhost, or full MirBuilder conversion.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-core-method-carrier-token-formatter-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/core_method_carrier_token_formatter.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_core_method_carrier_token_formatter_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-core-method-carrier-token-formatter-hako-adoption-decision-v0.json
```

## Acceptance

```text
parity_gate = green
parity_rows = 32
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
core_method_contract_manifest_migration = 0
method_resolution_migration = 0
route_collection_migration = 0
lowering_execution_migration = 0
backend_emission_migration = 0
```

## Decision

```text
decision:
  Adopt

reason_token:
  CoreMethodCarrierTokenFormatterRustOracleParityGateGreen

selected_next_card:
  MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-007
```

## Non-Claims

```text
no Source Selfhost claim
no Rust deletion
no runtime fallback
no new backend route
no new ABI
no generated artifact edit authority
no manifest migration
no method resolution migration
no carrier route collection migration
no lowering execution migration
no backend emission migration
```
