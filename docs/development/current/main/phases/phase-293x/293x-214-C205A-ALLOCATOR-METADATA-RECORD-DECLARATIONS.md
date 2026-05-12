# 293x-214: C205a Allocator Metadata Record Declarations

Status: Complete

## Purpose

Name the allocator metadata records that will later replace selected scalar
metadata columns, without changing live allocator behavior.

C205a is declaration-only. It proves that `hako_alloc` can carry allocator
metadata record declarations through the existing record metadata pipeline while
keeping the current M178/M180 scalar columns as runtime truth.

## Records

`allocator_metadata_records.hako` declares:

```hako
record HakoAllocAlignedSmallMeta {
    ptr: i64
    alignment: i64
    padded_size: i64
}

record HakoAllocHugePageMeta {
    page_id: i64
    ptr: i64
    requested_size: i64
    committed_size: i64
    live: i64
}
```

These map to the future migration targets:

- M178: `meta_ptrs`, `meta_alignments`, `meta_padded_sizes`
- M180: `page_ids`, `ptrs`, `requested_sizes`, `committed_sizes`, `live_flags`

## Stop Line

C205a does not:

- construct allocator metadata records
- read record fields in hako_alloc code
- replace scalar metadata arrays
- enable compiler auto-use of `ArrayStorage::InlineRecord`
- add backend, `.inc`, or Python lowering matchers
- touch provider/native allocator activation

## Acceptance

- `lang/src/hako_alloc/memory/allocator_metadata_records.hako` exists and is
  exported by `hako_module.toml`.
- MIR JSON for the declaration file includes `record_decls`,
  `record_layout_plans`, and `array_record_storage_plans` for both records.
- M178 and M180 owners still contain their scalar metadata columns.
- Existing M178/M180 guards remain green.
