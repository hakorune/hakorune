# mimalloc-aligned-small-path-proof

M178 proof app. It freezes the small aligned allocation path without widening
into huge-page routing or native alignment claims.

The proof keeps five boundaries explicit:

1. aligned small requests normalize through `HakoAllocAlignmentPolicy`;
2. successful aligned allocations register through `HakoAllocPageMap`;
3. alignment metadata stays observable only while the ptr is live;
4. invalid alignment, oversized padded size, and exhausted small-path capacity
   reject explicitly;
5. release still goes through the existing M172 seam.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_aligned_small_path_guard.sh
```
