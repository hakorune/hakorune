# 293x-223: C206e Metadata Store Indexed Read Cleanup

Status: Complete

## Scope

C206e is a behavior-preserving cleanup for the allocator metadata stores added
by C205c/C205d.

This row adds index-based read seams so callers that already resolved a pointer
to a metadata index do not need to re-run the pointer lookup for every field:

- `HakoAllocAlignedSmallMetaStore.alignmentAt(index)`
- `HakoAllocAlignedSmallMetaStore.paddedSizeAt(index)`
- `HakoAllocHugePageMetaStore.pageIdAt(index)`
- `HakoAllocHugePageMetaStore.requestedSizeAt(index)`
- `HakoAllocHugePageMetaStore.committedSizeAt(index)`
- `HakoAllocHugePageMetaStore.markReleasedAt(index)`

Pointer-based APIs remain available and delegate through the indexed seams.

## Non-Goals

C206e does not:

- change allocation, release, aligned, huge, or secure-list behavior.
- enable packed `ArrayBox` compiler auto-use.
- migrate hako_alloc state into packed array storage.
- add public ArrayBox APIs or materialize record values.
- touch backend lowering, provider activation, hooks, or process allocator
  replacement.

## Acceptance

- `tools/checks/k2_wide_metadata_store_indexed_read_guard.sh` passes.
- Existing C205c/C205d store guards still pass.
- Existing proof apps keep their exact output contracts.
