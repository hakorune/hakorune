# Hako Alloc Mimalloc Comparison Small Path Slice Proof

Row: `294x-55`

This proof app composes the comparison-facing small allocation slice from
existing owners:

- `SizeClassBox`
- `HakoAllocPageModel`
- `HakoAllocPageQueue`
- `HakoAllocPageMap`
- `HakoAllocPageMapReleaseSeam`

It keeps the row model-only and does not open remote free, TLS, atomics, OSVM,
provider activation, host allocator replacement, hooks, or
`#[global_allocator]`.

Run:

```bash
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_small_path_slice_guard.sh
```
