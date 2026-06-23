---
Status: Landed
Date: 2026-06-23
Scope: MirBuilder Rust-to-Hako converter ordering design
---

# 296x-1644: MirBuilder Ordering Capability Design

## Decision

Adopt Option 1:

```text
backend-accepted StringBox lexical comparison
```

but implement it as a generic comparator capability, not an
`OrderedMapBox.set/2` or `RegionObserver` special case.

SSOT:

```text
docs/development/current/main/design/mirbuilder-ordering-capability-ssot.md
```

## Key Boundary

The intermediate layer belongs to `tools/rust_lifecycle` converter IR /
capability code:

```text
IterationOrder::KeyAscending(RustStringOrdV1)
CompareTotal(RustStringOrdV1)
```

It does not belong to `crates/hakorune_mir_builder`.

## Next Implementation Slice

```text
DEFINE-TOTAL-TEXT-ORDERING-CAPABILITY-001
```

First implementation task:

```text
Define IterationOrder / ComparatorCapability structures and update the
RegionObserver route deny from UnsupportedKeyTransport to
UnsupportedOrderCapability.
```

This slice should not change backend behavior yet.
