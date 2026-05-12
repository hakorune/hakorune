# 293x-216: C205c Aligned-Small Metadata Record Store

Status: Complete

## Purpose

Move M178 aligned-small metadata ownership out of the allocation path owner and
behind a record-shaped store seam.

C205c is intentionally not the final packed `ArrayBox` migration. It connects
the C205b record construction/read lowering to live `hako_alloc` code while
keeping storage simple and backend-neutral.

## Implementation

New owner:

```text
lang/src/hako_alloc/memory/aligned_small_meta_store_box.hako
```

It owns:

- `ptrs`
- `alignments`
- `padded_sizes`
- `count`

and exposes:

- `append(ptr, alignment, padded_size)`
- `alignmentFor(ptr)`
- `paddedSizeFor(ptr)`

`append(...)` constructs `HakoAllocAlignedSmallMeta` and reads the fields
locally before updating the store columns. This proves the record value
lowering route in production allocator source without requiring record object
materialization.

## M178 Owner Change

`HakoAllocPageMapAlignedSmallPath` now delegates metadata writes and reads to
`HakoAllocAlignedSmallMetaStore`.

The allocation path remains responsible for:

- alignment policy
- page selection
- page-map registration
- allocation counters
- last-result observers

The metadata store owns aligned-small metadata residence and lookup.

## Stop Line

C205c does not:

- enable `ArrayStorage::InlineRecord` compiler auto-use
- materialize record objects
- migrate huge-page metadata
- add backend, `.inc`, Python, provider, hook, or native allocator lowering
- change M178 public proof output

## Acceptance

- the M178 proof app still prints the same summary and counters
- `page_map_aligned_small_path_box.hako` no longer owns direct
  `meta_ptrs` / `meta_alignments` / `meta_padded_sizes` columns
- `aligned_small_meta_store_box.hako` constructs
  `HakoAllocAlignedSmallMeta` and reads fields locally
- MIR for the store append route does not emit `NewBox` for
  `HakoAllocAlignedSmallMeta`
