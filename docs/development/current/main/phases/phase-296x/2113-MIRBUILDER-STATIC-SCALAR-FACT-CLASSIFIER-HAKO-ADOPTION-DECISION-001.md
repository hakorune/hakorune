# 2113 - MIRBUILDER-STATIC-SCALAR-FACT-CLASSIFIER-HAKO-ADOPTION-DECISION-001

## Token

```text
MIRBUILDER-STATIC-SCALAR-FACT-CLASSIFIER-HAKO-ADOPTION-DECISION-001
```

## Purpose

Adopt `static_scalar_fact_classifier` as the third narrow Rust-oracle parity
pilot owner after a green 8-row `.hako` EXE parity gate.

This decision adopts only the pure scalar-shape classifier:

```text
normalized zero-arg return literal shape -> static scalar fact text
```

It does not adopt static-scalar const emission, full AST traversal, method
dispatch, MirBuilder state mutation, Source Selfhost, or full MirBuilder
conversion.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-static-scalar-fact-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/static_scalar_fact_classifier.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_static_scalar_fact_classifier_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-static-scalar-fact-classifier-hako-adoption-decision-v0.json
```

## Acceptance

```text
parity_gate = green
parity_rows = 8
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
emit_static_scalar_fact_const_migration = 0
full_ast_json_traversal_migration = 0
method_call_dispatch_migration = 0
mirbuilder_state_mutation_migration = 0
```

## Decision

```text
decision:
  Adopt

reason_token:
  StaticScalarFactClassifierRustOracleParityGateGreen

selected_next_card:
  MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-003
```

## Non-Claims

```text
no Source Selfhost claim
no Rust deletion
no runtime fallback
no new backend route
no new ABI
no generated artifact edit authority
no const emission migration
no full AST traversal migration
no MirBuilder state mutation migration
```
