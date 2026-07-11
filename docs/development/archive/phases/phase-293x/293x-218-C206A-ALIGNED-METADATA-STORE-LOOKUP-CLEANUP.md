# 293x-218: C206a Aligned Metadata Store Lookup Cleanup

Status: Complete

## Scope

C206a is a BoxShape cleanup after the C205c/C205d metadata-store migrations.
It adds `HakoAllocAlignedSmallMetaStore.findIndex(ptr)` and makes
`alignmentFor(ptr)` / `paddedSizeFor(ptr)` delegate to that single lookup seam.

## Non-Goals

C206a does not:

- add new language syntax or accepted compiler shapes;
- enable `ArrayStorage::InlineRecord` compiler auto-use;
- introduce a shared generic metadata-store abstraction;
- change aligned allocation behavior, huge allocation behavior, release,
  realloc, secure-list, provider, hook, backend, or native allocator behavior.

## Acceptance

Run:

```bash
bash tools/checks/k2_wide_aligned_small_metadata_record_store_guard.sh
bash tools/checks/k2_wide_huge_page_metadata_record_store_guard.sh
```

The existing M178 aligned-small proof output must remain unchanged.
