# 2100 - MIRBUILDER-STORAGE-CLASS-CLASSIFIER-RUST-ORACLE-FIXTURE-001

## Token

```text
MIRBUILDER-STORAGE-CLASS-CLASSIFIER-RUST-ORACLE-FIXTURE-001
```

## Purpose

Create the Rust oracle fixture for the first hand-authored `.hako` native owner
pilot: `storage_class_classifier`.

This card fixes only the pure classifier boundary:

```text
MirType -> StorageClass
```

It does not migrate metadata refresh, function traversal, or
`value_storage_classes` mutation.

## Oracle Surface

```text
rust_file:
  src/mir/storage_class.rs

rust_function:
  classify_mir_type_storage_class

fixture:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    mirbuilder-storage-class-classifier-rust-oracle-v0.json
```

## Rows

The fixture covers:

```text
primitive MIR types:
  Integer
  Bool
  Float
  String

primitive boxes:
  IntegerBox
  BoolBox
  FloatBox
  StringBox

non-primitive box:
  MyUserBox

opaque cases:
  Array(Integer)
  Future(Integer)
  WeakRef
  Void
  Unknown
```

## Decision

```text
decision:
  SelectHakoNativeImplementation

selected_next_card:
  MIRBUILDER-STORAGE-CLASS-CLASSIFIER-HAKO-NATIVE-IMPLEMENTATION-001
```

## Non-Claims

```text
source_selfhost_claim = 0
hako_adopted_decision = 0
native_seed_materialization = 0
generated_artifact_as_native_edit_authority = 0
metadata_refresh_migration = 0
```
