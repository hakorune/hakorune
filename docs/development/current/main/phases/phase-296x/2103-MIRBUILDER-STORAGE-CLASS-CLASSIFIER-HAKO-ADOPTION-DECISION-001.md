# 2103 - MIRBUILDER-STORAGE-CLASS-CLASSIFIER-HAKO-ADOPTION-DECISION-001

## Token

```text
MIRBUILDER-STORAGE-CLASS-CLASSIFIER-HAKO-ADOPTION-DECISION-001
```

## Purpose

Adopt `storage_class_classifier` as the first narrow Rust-oracle parity pilot
owner.

This decision adopts only the pure classifier:

```text
MirType-shaped value -> StorageClass text
```

It does not adopt metadata refresh, function/module traversal, Rust deletion,
Source Selfhost, or full MirBuilder conversion.

## Evidence

```text
rust_oracle_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-storage-class-classifier-rust-oracle-v0.json

hako_source:
  lang/src/compiler/lib/storage_class_classifier.hako

parity_gate:
  tools/checks/rust_lifecycle_mirbuilder_storage_class_classifier_parity_gate.sh

adoption_fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-storage-class-classifier-hako-adoption-decision-v0.json
```

## Acceptance

```text
parity_gate = green
parity_rows = 14
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
metadata_refresh_migration = 0
```

## Decision

```text
decision:
  Adopt

reason_token:
  StorageClassClassifierRustOracleParityGateGreen

selected_next_card:
  MIRBUILDER-HAKO-NATIVE-OWNER-PARITY-PILOT-SELECTION-RERUN-001
```

## Non-Claims

```text
no Source Selfhost claim
no Rust deletion
no runtime fallback
no new backend route
no new ABI
no generated artifact edit authority
no metadata refresh migration
```
