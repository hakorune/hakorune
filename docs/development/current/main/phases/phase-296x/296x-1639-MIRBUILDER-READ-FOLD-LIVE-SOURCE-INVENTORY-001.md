# 296x-1639 MIRBUILDER-READ-FOLD-LIVE-SOURCE-INVENTORY-001

Status: Closed
Date: 2026-06-23

## Blocker

`ElideToReadFold` should be added only for live Rust source shapes. The
selected source:

```rust
for (k, v) in self.metadata_ctx.value_origin_callers().iter() {
    origin_callers.insert(*k, v.clone());
}
```

is not present in `src/mir/builder/emission/phi_lifecycle.rs`, but it is present
in current production source at:

```text
src/mir/builder/module_lifecycle.rs
src/mir/builder/calls/lowering.rs
```

Current live shape:

```rust
builder
    .metadata_ctx
    .value_origin_callers()
    .get(&dst)
    .cloned()
```

That shape is already covered by:

```text
BorrowUseFacts -> StorageAccessFacts -> ElideToLeafProjection -> MapGetOption
```

## Required Inventory

Find a real current source slice before adding a read-fold operation:

```text
aggregate borrow
  -> iterator/fold/read-only traversal
  -> output owns copied or cloned values
  -> aggregate alias does not escape
  -> owner is not mutated during use
```

Each candidate must record:

```text
file:line
borrowed field/API
consumer kind
order requirement
key/value transport
escape/mutation facts
selected lowering or Deny reason
```

## MapBox Key-Domain Note

If the selected live slice uses `MapBox.keys()` with `ValueIdAsI64` transport,
the verifier must make key-domain roundtrip explicit.

Current runtime implementation:

```text
MapKeyDomain::from_text("7") == MapKeyDomain::from_i64(7)
```

Therefore canonical numeric public text can round-trip to the i64 key domain,
but this must be stated as a verifier condition and behavior-tested. Do not
use `destination.get("7")` as proof for a `ValueIdAsI64` fold.

## Non-Goals

```text
fake generated methods for planned-only source shapes
read-view / lease framework
new Hako pointer syntax
string-concatenated key codecs
source-name-specific hardcode
runtime try-Hako-then-Rust fallback
```

## Acceptance

```text
read-fold candidate inventory has exact live file:line evidence
planned/docs-only shapes rejected as converter inputs
selected next slice has StorageAccessFacts classification
if MapBox.keys() is used, canonical i64 key-domain roundtrip is verified
metadata value-caller artifact guard remains green
current_state_pointer_guard green
```

## Result

```text
selected_slice=MetadataContext.value_origin_callers iter owned-copy
source_sites=module_lifecycle.rs,calls/lowering.rs
lowering=StorageAccessFacts -> ElideToReadFold -> MapReadFoldOwnedCopy
key_domain_roundtrip=CanonicalI64Text
```
