# MIMAP-012 bounded object-backed lifecycle queue SSOT

Decision: accepted.

`MIMAP-012` moves the lifecycle-aware page queue proof from scalar page facts to
real page objects retained in `ArrayBox`.

## Backend acceptance

Acceptance backend: LLVM/EXE primary.

VM remains diagnostic only for this object-heavy route. VM green is useful but
not required for MIMAP-012 completion.

## Owner boundary

Object-backed queue owner:

```text
lang/src/hako_alloc/memory/object_lifecycle_page_queue_box.hako
```

The owner may:

- retain `HakoAllocPageModel` objects in `ArrayBox`
- skip `page.decommitted != 0`
- call `page.canReuse()` and `page.reuse()` for retired reusable pages
- call `page.freeCount()` for active pages
- return the selected page object to the caller through the stored page index

The owner must not:

- call OSVM/page-source APIs
- own segments, TLS, atomics, remote-free, abandoned reclaim, or page-map lookup
- activate provider/hook/global allocator behavior
- add backend-specific matcher shortcuts

## Proof and guard

```text
apps/mimalloc-object-lifecycle-queue-proof/main.hako
tools/checks/k2_wide_mimalloc_object_lifecycle_queue_exe_guard.sh
```

## Follow-up

A later row may compose this object-backed queue through the allocator facade.
MIMAP-012 itself proves the queue owner route first.
