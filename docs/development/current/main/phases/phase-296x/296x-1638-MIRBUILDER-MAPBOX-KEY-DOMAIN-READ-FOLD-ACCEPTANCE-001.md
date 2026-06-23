# 296x-1638 MIRBUILDER-MAPBOX-KEY-DOMAIN-READ-FOLD-ACCEPTANCE-001

Status: Superseded by 296x-1639 and 296x-1640
Date: 2026-06-23

## Blocker

Desired source:

```rust
for (k, v) in self.metadata_ctx.value_origin_callers().iter() {
    origin_callers.insert(*k, v.clone());
}
```

Target lowering:

```text
BorrowUseFacts -> StorageAccessFacts -> ElideToReadFold -> owned output insert
```

Initial concern was that a naive Hako lowering using `MapBox.keys()` might not
be semantics-preserving for `HashMap<ValueId, String>`:

```text
source key transport:
  ValueIdAsI64

observed Hako surface:
  MapBox.keys() exposes public text keys

bad result:
  destination.set("7", value)
  destination.get(7) misses
```

Do not make the test pass by looking up `"7"` as a string.

Follow-up evidence:

```text
MapKeyDomain::from_text("7") == MapKeyDomain::from_i64(7)
```

The accepted path is not a string-key workaround. It requires an explicit
`CanonicalI64Text` roundtrip verifier condition and proves the fold with
`destination.get(7)`.

## Required Decision

Choose one:

```text
A. Add a key-domain preserving MapBox fold surface
   - key is copied as ValueId/i64
   - value is cloned/owned
   - raw map alias escapes = 0

B. Use a different storage representation for ValueId-key read-folds
   - only if it preserves key equality and selected observation semantics

C. Deny this slice
   - Deny(UnsupportedKeyTransport)
   - keep value_origin_callers().iter() parked
```

## Non-Goals

```text
read-view / lease framework
new Hako pointer syntax
string-concatenated key codecs
source-name-specific hardcode
runtime try-Hako-then-Rust fallback
```

## Acceptance

```text
ValueId key copied as i64, not public text
destination.get(7) succeeds after fold
destination.get("7") is not the proof
raw aggregate return = 0
element reference escape = 0
converter matrix green
```
