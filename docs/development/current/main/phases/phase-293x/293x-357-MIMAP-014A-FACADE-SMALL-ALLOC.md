# 293x-357 MIMAP-014A Facade Small Allocation Fast-Path

Status: landed
Date: 2026-05-15

## Decision

`MIMAP-014A` is the next primary allocator row. It proves one small allocation
through the thin object lifecycle facade and keeps the result observable through
scalar facade observers.

## Scope

- Add a narrow facade allocation method over the existing object lifecycle
  queue.
- Select one reusable page through the facade-owned queue.
- Call `HakoAllocPageModel.acquire(size)` on that page.
- Expose scalar result observers:
  - selected page id
  - allocated block id
  - allocation reason code
  - allocation success/failure summary
- Add one proof app and one LLVM/EXE-primary guard.

## Non-goals

- No release/free route.
- No realloc route.
- No alignment route.
- No OSVM/page-source route.
- No provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.
- No selected-object return through the facade.
- No broad dense proof read bundle inside the `.hako` app.
- No backend-specific matcher shortcuts.

## Expected files

```text
apps/mimalloc-facade-small-alloc-proof/main.hako
tools/checks/k2_wide_mimalloc_facade_small_alloc_exe_guard.sh
```

Likely implementation owner:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
```

## Acceptance

Expected proof output shape:

```text
mimalloc-facade-small-alloc-proof
alloc_page=<page id>
alloc_block=<block id>
alloc_reason=<reason code>
summary=ok
```

Required guard evidence:

```text
[mimap014a-mir-json] ok
[k2-wide-mimalloc-facade-small-alloc-exe] ok
```

## Implementation

`HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc(size)` selects through
the existing facade-owned queue, reads the queue's scalar selected index, then
retrieves the selected page from the queue-owned `pages` collection before
calling `HakoAllocPageModel.reuse()` and `HakoAllocPageModel.acquire(size)`.

The selected object is not returned through the facade. The proof app consumes
only scalar observers:

- `objectLifecycleAllocPageId()`
- `objectLifecycleAllocBlockId()`
- `objectLifecycleAllocReason()`
- `objectLifecycleAllocOk()`

## Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_small_alloc_exe_guard.sh
[mimap014a-mir-json] ok
[k2-wide-mimalloc-facade-small-alloc-exe] ok

bash tools/checks/k2_wide_mimalloc_facade_object_lifecycle_queue_exe_guard.sh
[mimap013-mir-json] ok
[k2-wide-mimalloc-facade-object-lifecycle-queue-exe] ok

bash tools/checks/current_state_pointer_guard.sh
[current-state-pointer-guard] ok

bash tools/checks/dev_gate.sh quick
[dev-gate] profile=quick ok
```

The quick gate initially stopped at the existing MIR root facade allowlist drift.
Cleanup kept semantic metadata imports on `crate::mir::function::*` instead of
re-exporting them from `crate::mir::*`; `mir-root-facade-guard` now reports
`ok exports=108`.

## Stop lines

If helper-call object-loop shape blocks MIR JSON or LLVM/EXE, stop this row and
land `MIR-ROW-B` with a minimized fixture.

If selected-object return becomes necessary, stop this row and land `MIR-ROW-C`
instead of broadening `MIMAP-014A`.

If dense observer reads block MIR emit, stop this row and land `MIR-ROW-D` with
a minimized fixture.

## Follow-up

After `MIMAP-014A` lands:

```text
MIMAP-014B:
  reusable-page preference, active-page fallback, and allocation miss reason

MIMAP-014C:
  allocation fast-path stats observers
```
