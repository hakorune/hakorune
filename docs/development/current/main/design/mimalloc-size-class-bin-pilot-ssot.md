---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: MIMAP-007 size-class / bin map executable pilot.
Related:
  - docs/development/current/main/design/mimalloc-first-executable-slice-ssot.md
  - docs/development/current/main/design/mimalloc-hakorune-record-vocabulary-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-198-M187-SIZE-CLASS-USIZE-POLICY.md
  - lang/src/hako_alloc/memory/size_class_box.hako
  - apps/mimalloc-size-class-usize-policy-proof/main.hako
---

# mimalloc Size-Class / Bin Pilot SSOT

## Decision

Adopt the existing `SizeClassBox` size-class policy and proof app as the first
executable near-transcription slice for the mimalloc blueprint lane.

This avoids duplicating an allocator table implementation while still giving the
mimalloc port a real executable foothold.

## Executable Artifacts

```text
policy owner:
  lang/src/hako_alloc/memory/size_class_box.hako

proof app:
  apps/mimalloc-size-class-usize-policy-proof/main.hako

existing guard:
  tools/checks/k2_wide_mimalloc_size_class_usize_policy_guard.sh
```

## Why This Fits MIMAP-007

| Requirement | Status |
| --- | --- |
| near-transcription table/arithmetic shape | satisfied by `SizeClassBox` |
| no OSVM | satisfied |
| no atomic | satisfied |
| no TLS/default heap | satisfied |
| no rawbuf/raw pointer residence | satisfied |
| no provider/global allocator hooks | satisfied |
| explicit oversized failure | satisfied through signed `-1` sentinel for existing API |
| `usize` input surface | satisfied by M187 facade |

## Current Policy Shape

`SizeClassBox` owns only pure size/bin decisions:

```text
size_to_bin(size)
bin_size(bin)
good_size(size)
accepts(size)
size_to_bin_usize(size: usize)
good_size_usize(size: usize)
bin_size_usize(bin: usize)
accepts_usize(size: usize)
```

It does not own pages, free lists, OS memory, allocator hooks, or provider
activation.

## Pilot Boundaries

```text
this pilot may:
  classify request sizes
  return a bin index
  return a good/usable size
  reject oversized requests explicitly

this pilot must not:
  allocate memory
  mutate page state
  inspect raw blocks
  invoke OSVM
  use atomics
  route provider hooks
```

## Relationship to Record Vocabulary

The existing executable policy returns scalar values. A later refinement may wrap
these values in the `SizeClassEntry` record from MIMAP-005B, but that is not
required to close MIMAP-007.

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

## Next Row

`MIMAP-008` should build the page/free-list model pilot on top of the existing
size-class policy, while staying no-OSVM/no-atomic/no-rawbuf for the first slice.
