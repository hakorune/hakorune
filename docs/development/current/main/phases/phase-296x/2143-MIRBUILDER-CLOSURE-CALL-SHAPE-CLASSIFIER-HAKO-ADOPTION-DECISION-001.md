# 2143 - MIRBUILDER-CLOSURE-CALL-SHAPE-CLASSIFIER-HAKO-ADOPTION-DECISION-001

## Token

```text
MIRBUILDER-CLOSURE-CALL-SHAPE-CLASSIFIER-HAKO-ADOPTION-DECISION-001
```

## Purpose

Adopt `closure_call_shape_classifier` as the ninth narrow Rust-oracle parity
pilot owner after a green 4-row `.hako` EXE parity gate.

This decision adopts only the pure shape/reject-code surface. It does not
adopt callsite canonicalization, NewClosure rewrite, backend fail-fast
boundary, MIR instruction mutation, Source Selfhost, or full MirBuilder
conversion.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-closure-call-shape-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/closure_call_shape_classifier.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_closure_call_shape_classifier_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-closure-call-shape-classifier-hako-adoption-decision-v0.json
```

## Acceptance

```text
parity_gate = green
parity_rows = 4
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
callsite_canonicalization_migration = 0
new_closure_rewrite_migration = 0
backend_fail_fast_boundary_migration = 0
mir_instruction_mutation_migration = 0
```

## Decision

```text
decision:
  Adopt

reason_token:
  ClosureCallShapeClassifierRustOracleParityGateGreen

selected_next_card:
  MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-009
```

## Non-Claims

```text
no Source Selfhost claim
no Rust deletion
no runtime fallback
no new backend route
no new ABI
no generated artifact edit authority
no callsite canonicalization migration
no NewClosure rewrite migration
no backend fail-fast boundary migration
no MIR instruction mutation migration
```
