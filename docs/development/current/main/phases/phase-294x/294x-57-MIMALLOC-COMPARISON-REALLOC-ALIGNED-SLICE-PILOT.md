---
Status: Landed
Date: 2026-05-23
Scope: V3 realloc/aligned comparison schema pilot.
Blocker: MIMALLOC-COMPARISON-VSLICE-005
Related:
  - docs/development/current/main/phases/phase-294x/294x-56-MIMALLOC-COMPARISON-V2-L3-DEFERRAL-AND-V3-SELECTION.md
  - apps/hako-alloc-mimalloc-comparison-realloc-aligned-slice-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_realloc_aligned_slice_guard.sh
---

# 294x-57 Mimalloc Comparison Realloc/Aligned Slice Pilot

## Decision

Start V3 with a model-only schema pilot that composes the existing realloc and
aligned-small owners:

```text
HakoAllocPageMapReallocSameClassPath
HakoAllocPageMapReallocAllocCopyReleasePath
HakoAllocPageMapAlignedSmallPath
```

The proof app emits stable comparison-facing fields:

```text
workload
same
grow
aligned
requested_bytes
copied_bytes_model
live_handles
rejects
release_count
alignment_meta
summary_fields
summary
```

This is not the V3 closeout. It anchors the realloc/aligned comparison schema
before the vertical slice moves to huge/OSVM composition.

## Stop Line

This row does not open:

- byte-copy execution;
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

## Next Blocker

```text
MIMALLOC-COMPARISON-VSLICE-006:
  start V4 huge/OSVM comparison schema pilot, reusing M179-M181 and the
  OSVM page-source composition seam.
```

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_realloc_aligned_slice_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
