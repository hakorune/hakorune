---
Status: SSOT
Date: 2026-06-23
Scope: MirBuilder Rust-to-Hako converter ordering capability boundary.
---

# MirBuilder Ordering Capability

## Decision

Use a generic ordering capability, not an `OrderedMapBox` or `RegionObserver`
special case.

The intermediate layer lives in the converter IR / capability seam:

```text
Rust source facts
  -> source semantics
     IterationOrder::KeyAscending(RustStringOrdV1)

tools/rust_lifecycle converter IR
  -> ReadFoldOwnedOutput(order=KeyAscending(RustStringOrdV1))
  -> CompareTotal(comparator=RustStringOrdV1)

backend capability proof
  -> RustStringOrdV1 accepted by VM / EXE / AOT

.hako library
  -> OrderedMapBox may use the comparator capability
```

Do not put this intermediate meaning in `crates/hakorune_mir_builder`. That
crate remains the Rust source / oracle owner. Do not put source-order policy in
`OrderedMapBox`; the collection may choose a representation strategy, but it
does not own Rust source semantics.

## Three Layers

```text
1. Source semantics
   What order the source requires.

2. Representation strategy
   How the target library maintains or produces that order.

3. Backend capability
   Whether the selected comparator and strategy are executable.
```

### Source Semantics

Use structured order facts:

```text
IterationOrder =
  Unobserved
  Unspecified
  Insertion
  KeyAscending(comparator_id)
  KeyDescending(comparator_id)
```

For `Rust BTreeMap<String, V>`:

```text
KeyAscending(RustStringOrdV1)
```

Future examples:

```text
Go map
  -> Unspecified

Python dict
  -> Insertion

Rust BTreeMap<i64, V>
  -> KeyAscending(SignedI64OrdV1)

C++ std::map<K, V, Compare>
  -> KeyAscending(concrete comparator id)
```

### Representation Strategy

Representation is separate from source semantics:

```text
OrderStrategy =
  MaintainAtWrite
  SortOnRead
  NativeOrderedStorage
```

`OrderedMapBox` currently models `MaintainAtWrite`. A future implementation may
switch to `SortOnRead` or `NativeOrderedStorage` without changing source facts
or converter IR.

### Backend Capability

Backend capability is expressed as:

```text
CompareTotal(comparator_id, left, right) -> -1 | 0 | 1
```

Initial selected comparator:

```text
RustStringOrdV1
```

The repository already has string compare exports such as `nyash.string.lt_hh`
and `nyash.string.eq_hh`; the implementation may either compose those or add a
single total-compare helper. The capability proof must cover VM / EXE / AOT
before `SourceOrdered` read-fold generation is allowed.

## Deny Reasons

Do not use `UnsupportedKeyTransport` for this blocker. The String key can be
transported; the missing piece is order capability.

Use:

```text
Deny(UnsupportedOrderCapability)
detail=ComparatorUnavailable
comparator=RustStringOrdV1
required_tiers=VM,EXE,AOT
```

Keep `UnsupportedKeyTransport` for cases where the key itself cannot be
represented safely:

```text
tuple key without structural representation
custom object key without equality/hash/ordering proof
nullable key ambiguity
```

## Ownership

```text
crates/hakorune_mir_builder
  Rust source / oracle only

tools/rust_lifecycle
  source facts
  IterationOrder facts
  ComparatorCapability
  converter IR
  route decision / Deny(reason)

apps/lib / lang/generated
  .hako library and generated execution artifacts

src/mir / backend / nyash_kernel
  executable comparator capability and backend proof
```

## Forbidden

```text
OrderedMapBox-name backend branch
RegionObserver-name backend branch
insertion-order substitution for Rust BTreeMap order
runtime try-Hako-then-Rust fallback
locale-dependent string ordering
new Hako pointer or borrow syntax
```

## Task Order

1. `Define total text ordering capability`

   ```text
   IterationOrder::KeyAscending(RustStringOrdV1)
   CompareTotal(RustStringOrdV1)
   UnsupportedOrderCapability deny reason
   ```

2. `Implement backend-accepted total text ordering capability`

   ```text
   comparator standalone VM / EXE / AOT proof
   equal / less / greater / prefix cases
   non-ASCII oracle case
   no OrderedMapBox / RegionObserver special cases
   ```

   Status: landed.

   Evidence:

   ```bash
   bash tools/checks/rust_lifecycle_text_order_rust_string_ord_v1_guard.sh
   ```

3. `Use total text ordering in OrderedMapBox`

   ```text
   b, a, args -> a, args, b
   update existing key
   remove
   clone_owned
   clear
   ```

   Status: landed.

   Evidence:

   ```bash
   bash tools/checks/rust_lifecycle_ordered_map_text_order_guard.sh
   ```

4. `Lower RegionObserver through verified source-ordered read-fold`

   ```text
   order = KeyAscending(RustStringOrdV1)
   comparator capability proof = VM/EXE/AOT accepted
   generated_hako MIR/EXE green
   raw aggregate borrow = 0
   insertion-order substitution = 0
   ```

   Status: current.

## Current Stop Line

Comparator execution is proven and OrderedMapBox consumes it, but source-ordered
read-fold generation remains closed until RegionObserver lowering is verified:

```text
source_ordered_read_fold_claim=0
generated_region_observer_artifact=0
runtime_fallback=0
```
