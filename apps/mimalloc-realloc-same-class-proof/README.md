# mimalloc-realloc-same-class-proof

M174 proof app. It freezes the no-move realloc contract while alloc-copy-release
fallback is still out of scope.

The proof keeps four cases explicit:

1. shrinking within the current live block returns the same pointer id;
2. growing beyond the current block rejects without releasing or unregistering;
3. released-block and stale-page rejects keep the existing live ownership entry
   observable;
4. seam release/unregister counters and page-local `used` / `local_free` state
   stay unchanged across every M174 attempt.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_realloc_same_class_guard.sh
```
