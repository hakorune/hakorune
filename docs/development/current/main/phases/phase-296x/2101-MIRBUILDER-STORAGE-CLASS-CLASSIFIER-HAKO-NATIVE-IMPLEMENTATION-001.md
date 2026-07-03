# 2101 - MIRBUILDER-STORAGE-CLASS-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001

## Token

```text
MIRBUILDER-STORAGE-CLASS-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Purpose

Add the first hand-authored `.hako` native owner pilot implementation for the
Rust-oracle parity path:

```text
owner:
  storage_class_classifier

contract:
  MirType-shaped value -> StorageClass text
```

This card implements only the pure classifier. It does not migrate metadata
refresh, function traversal, or `value_storage_classes` mutation.

## Native Source

```text
lang/src/compiler/lib/storage_class_classifier.hako
```

The implementation accepts the same JSON-shaped inputs as the Rust oracle
fixture through pure shape-specific entrypoints:

```text
primitive strings:
  Integer
  Bool
  Float
  String

object variants:
  {"Box": "..."}
  {"Array": "..."}
  {"Future": "..."}
```

The parity harness dispatches JSON object variants to the explicit MapBox /
kind entrypoints instead of requiring dynamic JSON type inspection inside the
classifier. That keeps the owner pure and AOT-friendly.

## Verification

```bash
bash tools/bin/hako --backend mir --verify \
  lang/src/compiler/lib/storage_class_classifier.hako
```

The follow-up parity gate compares the `.hako` output against:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-storage-class-classifier-rust-oracle-v0.json
```

## Decision

```text
decision:
  SelectParityGate

selected_next_card:
  MIRBUILDER-STORAGE-CLASS-CLASSIFIER-PARITY-GATE-001
```

## Non-Claims

```text
source_selfhost_claim = 0
hako_adopted_decision = 0
native_seed_materialization = 0
generated_artifact_as_native_edit_authority = 0
metadata_refresh_migration = 0
```
