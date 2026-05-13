# 293x-227 C210 Aligned-Small Metadata Packed-Store Pilot

Status: Complete

## Purpose

C210 applies the C209 private packed `ArrayBox` pilot to the aligned-small
metadata shape. It does not rewrite `.hako` source storage or expose compiler
feature names to `hako_alloc`.

## Decision

Decision: accepted.

Add a MIR-owned `hako_alloc_aligned_small_packed_store_pilot_plans` row that
connects:

- `HakoAllocAlignedSmallMeta`
- `HakoAllocAlignedSmallMetaStore`
- C209 `array_record_packed_autouse_pilot_plans`
- the private i64-column runtime storage/read seam

The live `.hako` owner remains record-shaped and source-level scalar-column
compatible until a later migration row removes that compatibility requirement.

## Row Contract

C210 emits:

```text
pilot_kind = aligned_small_metadata_i64_columns_v0
record_name = HakoAllocAlignedSmallMeta
store_owner = HakoAllocAlignedSmallMetaStore
ptr_column = 0
alignment_column = 1
padded_size_column = 2
private_runtime_storage_enabled = true
hako_alloc_source_mentions_compiler = false
live_scalar_columns_retained = true
public_array_get_materialization_enabled = false
backend_lowering_enabled = false
```

The runtime proof uses only the C209 crate-private direct i64-column seam.

## Stop Lines

- Do not move huge-page metadata in this row.
- Do not remove the aligned-small source scalar columns yet.
- Do not add `InlineRecord`, `ArrayStorage`, `PlanProbe`, or compiler feature
  names to `lang/src/hako_alloc`.
- Do not enable public record materialization.
- Do not add backend lowering or silent boxed fallback.
- Do not route through provider activation, hooks, native allocator replacement,
  or `.inc` string matching.

## Acceptance

- MIR metadata includes `hako_alloc_aligned_small_packed_store_pilot_plans`.
- MIR JSON exposes the new C210 rows.
- C210 consumes only the C209 packed pilot for `HakoAllocAlignedSmallMeta`.
- The field order is fixed as `ptr`, `alignment`, `padded_size`.
- A runtime proof reads aligned-small metadata through private packed i64
  columns without materializing records.
- The C210 guard stays local-run / index-listed and is not added to quick/dev
  gates.

## Verification

```bash
bash tools/checks/k2_wide_aligned_small_metadata_packed_store_pilot_guard.sh
cargo test -q mir::hako_alloc_aligned_small_packed_store_pilot
cargo test -q runner::mir_json_emit::tests::decl_values::collect_hako_alloc_aligned_small_packed_store_pilot_plan_values_preserves_pilot_limits
cargo test -q boxes::array::tests::aligned_small_metadata_packed_store_pilot_reads_metadata_columns
```
