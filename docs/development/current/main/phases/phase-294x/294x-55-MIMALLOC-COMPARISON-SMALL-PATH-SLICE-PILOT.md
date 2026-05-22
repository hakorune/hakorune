---
Status: Landed
Date: 2026-05-23
Scope: V2 small-path comparison slice pilot for the mimalloc comparison vertical slice.
Blocker: MIMALLOC-COMPARISON-VSLICE-003
Related:
  - docs/development/current/main/phases/phase-294x/294x-53-MIMALLOC-COMPARISON-VERTICAL-SLICE-WORKLOAD-PACK.md
  - apps/hako-alloc-mimalloc-comparison-small-path-slice-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_small_path_slice_guard.sh
---

# 294x-55 Mimalloc Comparison Small-Path Slice Pilot

## Decision

Start V2 with a model-only small allocation comparison slice that composes the
existing exact-`usize` owners instead of reopening the old production facade
heap lane.

The proof app uses:

```text
SizeClassBox
HakoAllocPageModel
HakoAllocPageQueue
HakoAllocPageMap
HakoAllocPageMapReleaseSeam
```

and emits a stable comparison-facing output schema:

```text
workload
requests
bins
block_sizes
queue
register
release
page_small
page_medium
map
summary_fields
summary
```

This is not yet the V2 closeout. It is the first small-path schema pilot for
`MIMALLOC-COMPARISON-VSLICE-003`.

Next blocker:

```text
MIMALLOC-COMPARISON-VSLICE-004:
  decide whether V2 needs a representative exact-MIR EXE closeout now, or can
  move directly to V3 realloc/aligned schema composition with this L2 pilot as
  the small-path anchor.
```

## Stop Line

This row does not open:

- remote free;
- TLS / worker-local behavior;
- atomics;
- OSVM/page-source behavior;
- provider activation;
- host allocator replacement;
- hooks;
- `#[global_allocator]`;
- C mimalloc execution;
- backend owner-name matchers.

It does not migrate the older `page_heap_box.hako` fields. The comparison slice
intentionally uses the newer exact-`usize` page model / page queue / page-map
owners.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_small_path_slice_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
