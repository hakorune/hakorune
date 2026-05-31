---
Status: SSOT
Decision: accepted
Date: 2026-06-01
Scope: No-escape Span contract for the direct-memory language substrate.
Related:
  - docs/reference/language/low-level-capabilities.md
  - docs/development/current/main/workstreams/direct-memory-current.md
  - docs/development/current/main/design/typed-numeric-memory-substrate-task-order-ssot.md
---

# Span No-Escape SSOT

## Decision

The first Span surface is a bounded, no-escape view over `DirectArrayI64`.

```text
SpanI64:
  read-only i64 element view

SpanMutI64:
  unique mutable i64 element view
```

Span is not a pointer. It does not expose pointer arithmetic, native addresses,
or `NativePtr` dereference. It is a compiler-visible view over a proven region.

## Ownership

```text
source owner:
  future Span syntax / API

semantic owner:
  no-escape Span contract in this document

fact owner:
  MIR metadata

consumer owner:
  future SpanAccessPlan / DirectMemoryAccessPlan
```

The first implementation must borrow only from `DirectArrayI64`. `Array`,
`ArrayBox`, `NativePtr`, `Bytes`, and LayoutSpan are outside v0.

## Allowed V0 Shape

The concrete source syntax is not finalized by this SSOT. The implementation may
choose a small pilot API, but it must preserve this semantic shape:

```hako
with xs = arr.span_i64() {
    local value = xs[i]
}

with xs = arr.span_mut_i64() {
    xs[i] = value
}
```

The `with` block is the lifetime scope. A Span value is only valid inside that
scope.

## Forbidden

The following must be rejected or left unplanned in v0:

```text
returning a Span
storing a Span in a field
capturing a Span in a closure
publishing or freezing a Span
passing a Span across provider/native/plugin boundaries
passing a mutable Span to unknown calls
owner resize/free while a Span is live
owner materialization that may move storage while a Span is live
escaping the `with` lifetime scope
creating two mutable Spans for the same region
creating mutable and shared Spans for the same region at the same time
```

Unsupported shapes must fail fast in verifier/planner rows. Silent fallback from
a selected Span plan is not allowed.

## Facts

The minimum fact vocabulary is:

```text
SpanBorrowFact:
  span_id
  region_value
  owner_value
  mutability = read | write
  element_type = i64
  start = 0
  length_value
  lifetime_scope
  no_escape = true
  owner_stable = true

RangeIndexFact:
  proves index interval

DirectArrayExtentFact:
  proves receiver extent covers required interval

RegionStabilityFact:
  proves storage is stable for the region
```

Span access planning must consume facts. It must not infer legality from method
names, `.hako` class names, or source spelling.

## Direct / Unsafe / Unchecked

Span does not merge these concepts:

```text
direct:
  fast-route requirement

unsafe memory:
  permission to create views over external/native memory

unchecked:
  proof result that removes checks
```

`SpanI64` / `SpanMutI64` are safe no-escape views over owned direct storage.
They are not `unsafe memory`.

## Acceptance For LANG-DM-005

Before implementing the minimal pilot, the selected slice must state:

```text
borrow_source=DirectArrayI64
span_kind=SpanI64|SpanMutI64
element_type=i64
no_escape_required=1
owner_resize_during_lifetime_allowed=0
unknown_call_crossing_allowed=0
selected_plan_silent_fallback_allowed=0
```

The first implementation should prefer one read-only access and one mutable
access fixture over a broad source surface.
