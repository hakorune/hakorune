---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: MIMAP-006 first executable near-transcription slice selection.
Related:
  - docs/development/current/main/design/mimalloc-hakorune-blueprint-task-breakdown-ssot.md
  - docs/development/current/main/design/mimalloc-hakorune-brand-type-vocabulary-ssot.md
  - docs/development/current/main/design/mimalloc-hakorune-record-vocabulary-ssot.md
  - docs/development/current/main/design/mimalloc-hakorune-capability-surface-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
---

# mimalloc First Executable Slice SSOT

## Decision

The first executable mimalloc-shaped slice is the size-class/bin lookup pilot.

```text
selected row:
  MIMAP-007 size-class / bin map executable pilot

why:
  near-transcription
  no OSVM
  no atomic
  no TLS
  no raw block residence
  no provider/global allocator behavior
```

## Rejected First Slices

| Candidate | Reason not first |
| --- | --- |
| stats snapshot/counter accumulation | Safe, but less likely to expose allocator table/record ergonomics. |
| local page free-count model | Useful, but lifecycle and transition semantics should come after a table pilot. |
| page queue selection model | Depends on size-class/bin vocabulary anyway. |
| segment purge/decommit model | Requires `uses osvm`. |
| cross-thread free model | Requires `uses atomic` and likely `uses tls`. |
| raw block free-list model | Requires `uses rawbuf` / `Span` / no-escape view. |

## Slice Shape

The pilot should model a table-driven mapping:

```text
input:
  requested_size: Bytes

output:
  SizeClassEntry or Result<SizeClassEntry, AllocLifecycleError>
```

Initial table row vocabulary comes from `MIMAP-005B`:

```hako
record SizeClassEntry {
    size_class_id: SizeClassId
    block_size: Bytes
    usable_size: Bytes
    page_size: Bytes
    blocks_per_page: BlockCount
    bin_index: Index
}
```

## Allowed Surface

```text
allowed:
  type aliases
  brands
  record literal
  Array<SizeClassEntry>
  loop i in 0..count
  guard
  Result / enum constructors
  check proof list

not required:
  PackedArray<T>
  const fn
  OSVM
  atomic
  TLS
  rawbuf
  provider hooks
```

## Acceptance Shape for MIMAP-007

MIMAP-007 should produce an executable pilot with a small proof surface:

```text
lookup(0) fails or maps to the minimum supported allocation policy explicitly
lookup(small boundary) returns expected size class
lookup(size just below boundary) returns previous/expected class
lookup(size just above boundary) returns next/expected class
lookup(too large for pilot) returns explicit error
```

The pilot may use a deliberately small table first. It must document that the
table is a pilot subset, not full mimalloc parity.

## Fail-Fast Requirements

```text
empty table:
  fail-fast / Err

unsorted table:
  proof failure

size above pilot range:
  explicit Err, not fallback to huge allocation

unsupported backend feature:
  not applicable; this slice should avoid backend-specific capabilities
```

## Next Row

`MIMAP-007` should implement the selected size-class/bin pilot as the first
executable slice.
