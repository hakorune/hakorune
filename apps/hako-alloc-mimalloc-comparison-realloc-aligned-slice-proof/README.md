# Hako Alloc Mimalloc Comparison Realloc/Aligned Slice Proof

Row: `294x-57`

This proof app composes the V3 comparison-facing realloc/aligned slice with a
production-facade realloc route plus existing aligned-owner composition:

- `HakoAllocProductionFacade` (allocate/reallocResult/isLiveHandle)
- `HakoAllocPageMapAlignedSmallPath`

It publishes stable comparison fields for requested bytes, modeled copied
bytes, live handles, reject counts, release counts, and alignment metadata
without opening byte-copy execution, remote-free, TLS, atomics, OSVM,
provider activation, host allocator replacement, hooks, or
`#[global_allocator]`.

Run:

```bash
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_realloc_aligned_slice_guard.sh
```
