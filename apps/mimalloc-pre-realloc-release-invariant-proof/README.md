# mimalloc-pre-realloc-release-invariant-proof

M173 proof app. It freezes the pre-realloc release contract by observing the
existing `HakoAllocPageMapReleaseSeam` through
`HakoAllocPageMapReleaseObserver`.

The proof keeps three cases explicit before realloc exists:

1. successful release expires the handle and advances release/unregister counts;
2. released-block reject keeps the handle live and leaves release/unregister
   deltas at zero;
3. stale/unknown rejects stay observable without widening into realloc, byte
   copy, aligned allocation, huge allocation, hooks, or provider work.

Run:

```bash
bash tools/checks/k2_wide_mimalloc_pre_realloc_release_invariant_guard.sh
```
