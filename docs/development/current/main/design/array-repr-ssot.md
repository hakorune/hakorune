---
Status: Active
Date: 2026-05-30
Scope: ArrayRepr bridge SSOT for public ArrayBox facade and DirectArray family storage substrate.
Related:
  - docs/development/current/main/phases/phase-296x/296x-378-ARRAY-REPR-DESIGN-ROW.md
  - docs/development/current/main/phases/phase-296x/296x-377-ARRAY-SLOT-NATIVEDIRECT-POST-RETIREMENT-PERF-OWNER-REFRESH.md
  - docs/development/current/main/phases/phase-296x/296x-374-DIRECTARRAY-FAMILY-STORAGE-SUBSTRATE-ROADMAP.md
  - docs/development/current/main/design/array-lane-extension-roadmap-ssot.md
  - docs/development/current/main/design/representation-direct-lowering-ssot.md
  - docs/development/current/main/design/representation-direct-storage-substrate-ssot.md
---

# ArrayRepr SSOT

## Purpose

Define the bridge between the public `ArrayBox` facade and the long-term
`DirectArray` storage family without exposing plugin internals as ABI.

`ArrayRepr` is a representation contract, not a new public array API surface.
It decides which array shape is carried in hot regions and when materialization
back to the public facade is required.

## Variants

```text
ArrayRepr
  DirectI64
  PublicArrayBoxFallback
```

### DirectI64

- exact `i64` hot storage
- candidate for NativeDirect lowering
- owned by the `DirectArray` family, not by `ArrayBox`

### PublicArrayBoxFallback

- public semantic facade / mixed-storage fallback
- keeps public `ArrayBox` behavior intact
- materialization target when the direct facts are not proven

## Ownership

```text
ArrayBox:
  public facade / materialized view / compatibility object

DirectArray family:
  storage substrate for direct exact-array regions

Representation planner:
  chooses ArrayRepr from facts only

Lowerer:
  consumes the selected ArrayRepr; it does not re-prove eligibility
```

## Materialization Route

Materialization must be explicit.

```text
DirectI64 region
  -> explicit materialization
  -> public ArrayBox fallback or snapshot
```

The bridge must not:

- reinterpret a public `ArrayBox` host handle as a direct pointer
- expose plugin internals as ABI
- no `nyash.array.birth_h` behavior change
- silently fall back after selecting a direct plan

## Fail-Fast

```text
selected ArrayRepr == DirectI64
and direct facts are not proven
  -> no direct plan

selected ArrayRepr == DirectI64
and public birth semantics would change
  -> fail-fast

selected plan silently falls back
  -> row failure
```

## Bridge Boundary

`ArrayRepr` is the design bridge that sits between:

- `representation-direct-lowering-ssot.md`
- `representation-direct-storage-substrate-ssot.md`
- `ArrayBox` public facade ownership
- `DirectArray` family storage ownership

It is intentionally narrow:

- `DirectI64` is the first exact-storage bridge
- `PublicArrayBoxFallback` preserves public semantics
- future variants may be added later, but not by widening this row

Future Array lane expansion is governed by
`docs/development/current/main/design/array-lane-extension-roadmap-ssot.md`.
Do not add `ArrayRepr` variants or `ArrayStorage` variants directly from this
row.
