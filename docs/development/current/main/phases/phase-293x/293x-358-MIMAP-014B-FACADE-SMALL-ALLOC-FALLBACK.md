# 293x-358 MIMAP-014B Facade Small Allocation Fallback

Status: landed
Date: 2026-05-15

## Decision

`MIMAP-014B` is the next primary allocator row after `MIMAP-014A`. It extends
the facade small allocation fast-path to prove reusable-page preference, active
page fallback, and miss/fail reason when no candidate page remains.

## Scope

- Keep allocation routed through `HakoAllocObjectLifecycleFacade`.
- Prefer one reusable page when available.
- Fall back to one active page after reusable candidates are unavailable.
- Expose scalar observer data only:
  - selected page id
  - allocated block id
  - selected kind
  - miss/fail reason
  - success/failure summary
- Add one proof app and one LLVM/EXE-primary guard.

## Non-goals

- No release/free route.
- No realloc route.
- No alignment route.
- No OSVM/page-source route.
- No provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.
- No selected-object return through the facade.
- No dense proof read bundle beyond the row output contract.
- No backend-specific matcher shortcuts.

## Expected Files

```text
apps/mimalloc-facade-small-alloc-fallback-proof/main.hako
tools/checks/k2_wide_mimalloc_facade_small_alloc_fallback_exe_guard.sh
```

Likely implementation owner:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
```

## Acceptance

Expected proof output shape:

```text
mimalloc-facade-small-alloc-fallback-proof
reusable_page=<page id>
active_page=<page id>
miss_reason=<reason code>
summary=ok
```

Required guard evidence:

```text
[mimap014b-mir-json] ok
[k2-wide-mimalloc-facade-small-alloc-fallback-exe] ok
```

## Implementation

`HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc(size)` now accepts
both queue selection kinds used by the object lifecycle queue:

- kind `1`: reusable page, calling `HakoAllocPageModel.reuse()` before
  `acquire(size)`
- kind `2`: active page fallback, calling `acquire(size)` directly

The proof keeps the selected page object inside the queue/facade route and reads
only scalar observers for page id, selected kind, miss reason, and success.

## Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_small_alloc_fallback_exe_guard.sh
[mimap014b-mir-json] ok
[k2-wide-mimalloc-facade-small-alloc-fallback-exe] ok

bash tools/checks/k2_wide_mimalloc_facade_small_alloc_exe_guard.sh
[mimap014a-mir-json] ok
[k2-wide-mimalloc-facade-small-alloc-exe] ok

bash tools/checks/k2_wide_mimalloc_facade_object_lifecycle_queue_exe_guard.sh
[mimap013-mir-json] ok
[k2-wide-mimalloc-facade-object-lifecycle-queue-exe] ok

bash tools/checks/dev_gate.sh quick
[dev-gate] profile=quick ok
```

## Stop Lines

If reusable and active allocation cannot stay in one scalar facade route, stop
and split the active-page fallback into a smaller sidecar row.

If selected-object return becomes necessary, stop this row and land `MIR-ROW-C`
instead of broadening `MIMAP-014B`.

If dense observer reads block MIR emit, stop this row and land `MIR-ROW-D` with
a minimized fixture.

## Follow-up

After `MIMAP-014B` lands:

```text
MIMAP-014C:
  allocation fast-path stats observers
```
