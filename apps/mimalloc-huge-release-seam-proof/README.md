# mimalloc-huge-release-seam-proof

M181 proof app. It freezes huge release composition without using small page
free lists or OS release.

The proof keeps four boundaries explicit:

1. huge release starts from `HakoAllocPageMap.lookup`;
2. live huge state is retired in `HakoAllocHugePageModel`;
3. page-map ownership is unregistered only after the model accepts release;
4. unknown, double-release, and non-huge page-map handles reject explicitly.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_huge_release_seam_guard.sh
```
