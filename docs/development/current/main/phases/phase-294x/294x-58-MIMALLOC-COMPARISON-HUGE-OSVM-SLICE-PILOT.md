---
Status: Landed
Date: 2026-05-23
Scope: V4 huge/OSVM comparison schema pilot.
Blocker: MIMALLOC-COMPARISON-VSLICE-006
Related:
  - docs/development/current/main/phases/phase-294x/294x-57-MIMALLOC-COMPARISON-REALLOC-ALIGNED-SLICE-PILOT.md
  - apps/hako-alloc-mimalloc-comparison-huge-osvm-slice-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_huge_osvm_slice_guard.sh
---

# 294x-58 Mimalloc Comparison Huge/OSVM Slice Pilot

## Decision

Start V4 with a huge/OSVM comparison schema pilot that composes existing
owners:

```text
HakoAllocHugeThresholdRouter
HakoAllocHugePageModel
HakoAllocHugeReleaseSeam
HakoAllocOsVmFastPathPurgeRoute
HakoAllocOsVmBackedFastPathHeap
```

The proof app emits stable comparison-facing fields:

```text
workload
route
huge
osvm
osvm_bases
summary_fields
summary
```

The row consumes the already-existing OSVM page-source seam. It does not create
a new OSVM owner and does not turn OSVM into a provider or host allocator
activation path.

## Validation Note

The VM reference runner cannot execute the OSVM extern family. This row is
therefore validated as a representative exact-MIR pure-first EXE slice:

```text
MIR JSON emit
route preflight
exact MIR artifact -> pure-first EXE
EXE output schema
```

The guard also verifies that required `usize` typed-object fields remain exact
storage in the huge and OSVM-backed owners.

## Stop Line

This row does not open:

- new OSVM ownership;
- OSVM unreserve/release ownership;
- remote free;
- TLS / worker-local behavior;
- atomics;
- provider activation;
- host allocator replacement;
- hooks;
- `#[global_allocator]`;
- C mimalloc execution;
- backend owner-name matchers.

## Next Blocker

```text
MIMALLOC-COMPARISON-VSLICE-007:
  close the comparison vertical slice by unifying the `.hako` report schema
  with the C mimalloc runner/report lane, keeping provider activation and host
  replacement parked.
```

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_huge_osvm_slice_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
