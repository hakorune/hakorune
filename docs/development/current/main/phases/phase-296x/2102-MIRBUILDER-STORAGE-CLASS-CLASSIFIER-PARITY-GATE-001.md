# 2102 - MIRBUILDER-STORAGE-CLASS-CLASSIFIER-PARITY-GATE-001

## Token

```text
MIRBUILDER-STORAGE-CLASS-CLASSIFIER-PARITY-GATE-001
```

## Purpose

Lock the first Rust-oracle parity gate for the hand-authored `.hako` native
owner pilot:

```text
owner:
  storage_class_classifier

rust oracle:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-storage-class-classifier-rust-oracle-v0.json

hako implementation:
  lang/src/compiler/lib/storage_class_classifier.hako
```

The gate generates a temporary `.hako` app from the Rust oracle rows, emits an
EXE through the primary MIR route, runs it, and compares normalized stdout
against the expected `StorageClass` strings.

## Gate

```bash
bash tools/checks/rust_lifecycle_mirbuilder_storage_class_classifier_parity_gate.sh
```

Output contract:

```text
output_contract=rust-lifecycle-mirbuilder-storage-class-classifier-parity-gate-v0
parity_rows=14
parity_status=green
```

## Scope Boundary

The gate covers only:

```text
MirType-shaped oracle rows -> StorageClass text
```

It does not cover:

```text
metadata.value_storage_classes mutation
refresh_module_storage_class_facts
refresh_function_storage_class_facts
Source Selfhost claim
```

## Decision

```text
decision:
  SelectHakoAdoptionDecision

selected_next_card:
  MIRBUILDER-STORAGE-CLASS-CLASSIFIER-HAKO-ADOPTION-DECISION-001
```

## Non-Claims

```text
source_selfhost_claim = 0
hako_adopted_decision = 0
native_seed_materialization = 0
generated_artifact_as_native_edit_authority = 0
metadata_refresh_migration = 0
```
