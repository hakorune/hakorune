# Phase 87x SSOT

## Intent

`87x` reruns the embedded snapshot / wrapper repoint seam that `84x` deliberately deferred.

## Facts to Keep Stable

- `84x` repointed build/default contracts to canonical `lang/src/runner/entry/*`.
- `84x` explicitly deferred snapshot-coupled path pressure:
  - `src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json`
- top-level `.hako` wrappers remain explicit keep surfaces unless fresh caller evidence says otherwise.
- `86x` already reduced mirror/index pressure; this lane is source-facing again.

## Initial Focus

1. inventory snapshot-pinned top-level wrapper paths
2. separate repointable consumers from keep-now/public surfaces
3. keep stop-lines explicit before touching snapshot-coupled contracts

## Acceptance

1. snapshot/wrapper inventory is source-backed
2. any repoint is narrow and does not remove public keep surfaces by accident
3. proof keeps Stage1/mainline green
