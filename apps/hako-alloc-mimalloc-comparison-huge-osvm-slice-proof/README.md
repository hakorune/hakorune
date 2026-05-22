# Hako Alloc Mimalloc Comparison Huge/OSVM Slice Proof

Row: `294x-58`

This proof app composes the V4 comparison-facing huge/OSVM slice from existing
owners:

- `HakoAllocHugeThresholdRouter`
- `HakoAllocHugePageModel`
- `HakoAllocHugeReleaseSeam`
- `HakoAllocOsVmBackedFastPathHeap`

It publishes stable comparison fields for requested bytes, committed/backing
bytes, live handles, release count, page-source operation count, and reject
count without opening provider activation, host allocator replacement, hooks,
TLS, atomics, or a new OSVM ownership seam.

Run:

```bash
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_huge_osvm_slice_guard.sh
```
