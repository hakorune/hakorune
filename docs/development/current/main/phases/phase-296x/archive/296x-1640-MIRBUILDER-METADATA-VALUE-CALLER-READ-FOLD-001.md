# 296x-1640 MIRBUILDER-METADATA-VALUE-CALLER-READ-FOLD-001

Status: Closed
Date: 2026-06-23

## Slice

Live Rust source:

```text
src/mir/builder/module_lifecycle.rs
src/mir/builder/calls/lowering.rs
```

Shape:

```rust
for (k, v) in self.metadata_ctx.value_origin_callers().iter() {
    origin_callers.insert(*k, v.clone());
}
```

Lowering:

```text
BorrowUseFacts
  -> StorageAccessFacts
  -> ElideToReadFold
  -> MapReadFoldOwnedCopy
```

## Contract

```text
standalone value_origin_callers() -> Deny(ReturnedReadBorrow)
known call-local read fold -> ElideToReadFold
raw aggregate alias = 0
element reference escape = 0
owner mutation during fold = 0
key_domain_roundtrip = CanonicalI64Text
```

## Verification

```text
bash tools/checks/rust_lifecycle_metadata_context_value_caller_derived_artifact_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

The generated executable proves the fold with `destination.get(7)`, not with a
string-key lookup.

## Next

```text
VARIABLE-MAP-ORDERED-OBSERVER-READ-FOLD-001
```
