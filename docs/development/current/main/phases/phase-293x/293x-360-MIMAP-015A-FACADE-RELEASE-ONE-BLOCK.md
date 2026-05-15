# 293x-360 MIMAP-015A Facade Release One Block

Status: landed
Date: 2026-05-15

## Decision

`MIMAP-015A` is the next primary allocator row after `MIMAP-014C`. It adds a
facade-owned release/free route for one known block returned by the small
allocation observers.

## Scope

- Keep release routed through `HakoAllocObjectLifecycleFacade`.
- Release one known `(page id, block id)` pair that was allocated by the facade
  small allocation route.
- Use existing `HakoAllocPageModel.releaseLocal(block_id)`.
- Expose scalar release result observers:
  - released page id
  - released block id
  - release reason code
  - success/failure summary
- Add one proof app and one LLVM/EXE-primary guard.

## Non-goals

- No realloc route.
- No alignment route.
- No double-release / stale-release diagnostics; that belongs to `MIMAP-015B`.
- No page-map lookup or arbitrary pointer resolution.
- No OSVM/page-source route.
- No provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.
- No selected-object return through the facade.
- No backend-specific matcher shortcuts.

## Expected Files

```text
apps/mimalloc-facade-release-one-block-proof/main.hako
tools/checks/k2_wide_mimalloc_facade_release_one_block_exe_guard.sh
```

Likely implementation owner:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
```

## Acceptance

Expected proof output shape:

```text
mimalloc-facade-release-one-block-proof
alloc=<page id>,<block id>
release=<page id>,<block id>,<reason code>
summary=ok
```

Required guard evidence:

```text
[mimap015a-mir-json] ok
[k2-wide-mimalloc-facade-release-one-block-exe] ok
```

## Implementation

`HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock(page_id, block_id)`
resolves one known page id inside the facade-owned object lifecycle queue and
delegates to `HakoAllocPageModel.releaseLocal(block_id)`.

The route is intentionally bounded to the queue pages already used by the
facade proof rows. It does not add arbitrary pointer lookup, page-map lookup, or
double/stale release diagnostics.

Scalar release observers:

- `objectLifecycleReleasePageId()`
- `objectLifecycleReleaseBlockId()`
- `objectLifecycleReleaseReason()`
- `objectLifecycleReleaseOk()`

## Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_release_one_block_exe_guard.sh
[mimap015a-mir-json] ok
[k2-wide-mimalloc-facade-release-one-block-exe] ok

bash tools/checks/k2_wide_mimalloc_facade_small_alloc_stats_exe_guard.sh
[mimap014c-mir-json] ok
[k2-wide-mimalloc-facade-small-alloc-stats-exe] ok

bash tools/checks/k2_wide_mimalloc_facade_small_alloc_fallback_exe_guard.sh
[mimap014b-mir-json] ok
[k2-wide-mimalloc-facade-small-alloc-fallback-exe] ok

bash tools/checks/k2_wide_mimalloc_facade_small_alloc_exe_guard.sh
[mimap014a-mir-json] ok
[k2-wide-mimalloc-facade-small-alloc-exe] ok

bash tools/checks/dev_gate.sh quick
[dev-gate] profile=quick ok
```

## Stop Lines

If release needs arbitrary pointer-to-page lookup, stop and split a page-map
handoff row instead of widening this row.

If double-release or stale-release fail-fast becomes necessary, stop and keep it
for `MIMAP-015B`.

If selected-object return becomes necessary, stop this row and land `MIR-ROW-C`
instead of broadening `MIMAP-015A`.

## Follow-up

After `MIMAP-015A` lands:

```text
MIMAP-015B:
  double-release / stale-release fail-fast route
```
