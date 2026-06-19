# OrderedMapBox Boundary SSOT

Status: accepted design boundary
Date: 2026-06-20

## Purpose

`OrderedMapBox` provides a deterministic ordered map for `.hako` app and
compiler-construction code.

It exists because MirBuilder migration and RustSubset/crate handoff work need
Rust `BTreeMap<String, T>`-like deterministic iteration without importing
Rust's collection semantics into `.hako` compiler logic.

## Decision

```text
OrderedMapBox v0:
  owner = .hako library
  location = apps/lib/collections/ordered_map.hako
  key_domain = String only
  ordering = deterministic lexical key order
  implementation_goal = simple + stable
  performance_goal = none
```

`OrderedMapBox` does not start in ring0.

```text
ring0_enabled=0
ring1_provider_enabled=0
mapbox_semantics_changed=0
rust_btreemap_api_claim=0
```

## Layer Boundary

### OrderedMapBox Owns

```text
stable key order
insert/update lookup by String key
snapshot keys in sorted order
snapshot values in key order
small deterministic library behavior
```

### OrderedMapBox Does Not Own

```text
MapBox public semantics
MapBox key canonicalization
runtime raw map substrate
ring0 capability design
Rust name resolution
crate symbol linking
MirBuilder authority migration
```

## Why Not MapBox

`MapBox` is the dynamic public map surface. It owns general map behavior and
public runtime compatibility.

`OrderedMapBox` is a deterministic helper for compiler/app code. It should not
inherit `MapBox` behavior such as dynamic key canonicalization or runtime raw
map routing.

This keeps the meanings separate:

```text
MapBox:
  dynamic public container

OrderedMapBox:
  deterministic library collection
```

## Why Not Ring0 First

Ring0 is a substrate boundary. `OrderedMapBox` v0 does not require a substrate
capability; it requires a stable `.hako` collection API.

Starting in ring0 would force questions that the current caller does not need:

```text
host ABI
provider registration
raw storage substrate
cross-backend capability
performance contract
```

The v0 user needs only:

```text
String key order
stable snapshots
simple set/get/has/length
```

## Promotion Rule

Promotion is possible, but only with evidence.

```text
apps/lib -> ring1:
  allowed when two or more real users need a shared provider shape
  and the `.hako` API has fixture coverage

ring1 -> ring0:
  allowed only by a separate substrate capability card
  with backend/runtime evidence
```

Promotion must not change the source-level API.

## v0 API Contract

```text
set(key, value)
get(key)
has(key)
length()
keys()
key_at(index)
values()
```

The first implementation does not coerce keys. v0 callers must pass String
keys, and tests must stay in that domain. This keeps OrderedMapBox separate
from MapBox key-publication/canonicalization semantics.

`key_at(index): StringBox` is accepted as a v0 typed observer. It exists because
EXE/AOT can lose ArrayBox element type information through `keys().get(i)`;
`key_at` lets tests and compiler probes verify deterministic key order without
turning `keys()` into a backend-specific contract.

## MirBuilder Migration Boundary

`OrderedMapBox` may support BindingContext-like migration probes, but it is not
the MirBuilder owner.

Allowed:

```text
use OrderedMapBox in a focused BindingContext-style probe
compare deterministic output against Rust oracle
```

Not allowed:

```text
rewrite MirBuilder around OrderedMapBox in the first row
use OrderedMapBox to hide missing symbol-resolution semantics
claim Rust BTreeMap parity beyond String-key deterministic iteration
```

## Task Sequence

```text
1. HAKO-ORDERED-MAP-BOX-SSOT-001
   docs/reference/design only

2. HAKO-ORDERED-MAP-BOX-V0-001
   implement apps/lib/collections/ordered_map.hako
   add deterministic-order smoke

3. MIRBUILDER-BINDING-CONTEXT-ORDERED-MAP-PROBE-001
   use OrderedMapBox in a BindingContext-style probe
   no broad MirBuilder rewrite

4. ORDERED-MAP-RING1-PROMOTION-INVENTORY-001
   open only if multiple real users justify promotion
```

## Stop Line

```text
do not change MapBox
do not add ring0 substrate support
do not add ring1 provider registration
do not add Rust BTreeMap full API
do not accept arbitrary key domains
do not use OrderedMapBox as name-resolution owner
do not mix implementation with MirBuilder owner migration
```

## Reference

- `docs/reference/boxes-system/ordered-mapbox.md`
