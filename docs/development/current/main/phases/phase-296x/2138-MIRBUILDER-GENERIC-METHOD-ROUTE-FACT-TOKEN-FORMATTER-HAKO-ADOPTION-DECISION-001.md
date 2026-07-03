# 2138 - MIRBUILDER-GENERIC-METHOD-ROUTE-FACT-TOKEN-FORMATTER-HAKO-ADOPTION-DECISION-001

## Token

```text
MIRBUILDER-GENERIC-METHOD-ROUTE-FACT-TOKEN-FORMATTER-HAKO-ADOPTION-DECISION-001
```

## Purpose

Adopt `generic_method_route_fact_token_formatter` as the eighth narrow
Rust-oracle parity pilot owner after a green 12-row `.hako` EXE parity gate.

This decision adopts only the pure token formatter surface. It does not adopt
receiver origin resolution, key route classification, const i64 extraction,
generic method route planning, backend emission, Source Selfhost, or full
MirBuilder conversion.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-generic-method-route-fact-token-formatter-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/generic_method_route_fact_token_formatter.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_generic_method_route_fact_token_formatter_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-generic-method-route-fact-token-formatter-hako-adoption-decision-v0.json
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
receiver_origin_resolution_migration = 0
key_route_classification_migration = 0
const_i64_extraction_migration = 0
generic_method_route_planning_migration = 0
backend_emission_migration = 0
```

## Decision

```text
decision:
  Adopt

reason_token:
  GenericMethodRouteFactTokenFormatterRustOracleParityGateGreen

selected_next_card:
  MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-008
```

## Non-Claims

```text
no Source Selfhost claim
no Rust deletion
no runtime fallback
no new backend route
no new ABI
no generated artifact edit authority
no receiver origin resolution migration
no key route classification migration
no const i64 extraction migration
no generic method route planning migration
no backend emission migration
```
