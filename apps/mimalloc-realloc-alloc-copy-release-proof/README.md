# mimalloc-realloc-alloc-copy-release-proof

M175 proof app. It freezes the grow fallback contract after M174 rejects the
same-class branch.

The proof keeps five cases explicit:

1. grow fallback allocates a new ptr, models one copy, then releases the old
   ptr only after allocation succeeds;
2. same-class requests reject so the M174 no-move path stays the owner;
3. allocation failure keeps the old ptr live and does not add release or
   unregister deltas;
4. released-block, stale-page, and unknown rejects stay observable before the
   M176 failure matrix exists;
5. page-local deltas show old-page release and replacement-page acquire on the
   success path only.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_realloc_alloc_copy_release_guard.sh
```
