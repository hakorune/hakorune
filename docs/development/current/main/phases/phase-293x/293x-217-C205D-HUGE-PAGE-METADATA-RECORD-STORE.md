# 293x-217: C205d Huge-Page Metadata Record Store

Status: Complete

## Scope

C205d migrates M180 huge-page metadata ownership behind a record-shaped store
without changing the huge allocation/release behavior:

- Add `lang/src/hako_alloc/memory/huge_page_meta_store_box.hako`.
- Keep `HakoAllocHugePageModel` as the M180 orchestration owner.
- Move `page_ids`, `ptrs`, `requested_sizes`, `committed_sizes`, and
  `live_flags` into `HakoAllocHugePageMetaStore`.
- Construct `HakoAllocHugePageMeta` at the store append boundary and read its
  fields immediately through C205b builder-local record scalarization.
- Preserve the M180/M181 VM proof outputs.

## Non-Goals

C205d does not:

- enable `ArrayStorage::InlineRecord` compiler auto-use;
- migrate small-page/page-local state vectors;
- materialize record values as ordinary `NewBox` objects;
- add backend, `.inc`, LLVM, provider, hook, or native allocator behavior;
- change huge release, page-map unregister, OS release, or secure-list logic.

## Acceptance

Run:

```bash
bash tools/checks/k2_wide_huge_page_metadata_record_store_guard.sh
bash tools/checks/k2_wide_mimalloc_huge_page_model_guard.sh
bash tools/checks/k2_wide_allocator_metadata_record_declarations_guard.sh
bash tools/checks/k2_wide_allocator_record_construction_read_guard.sh
bash tools/checks/k2_wide_mimalloc_numeric_field_inventory_delta_guard.sh
```

The C205d guard verifies that the store append function contains no
`HakoAllocHugePageMeta` `NewBox` or field-get MIR after record scalarization.
