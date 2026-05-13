# 293x-221: C206d ArrayBox Inline-Record Plan Probe

Status: Complete

## Scope

C206d adds a test-only adapter from MIR `ArrayRecordStoragePlan` metadata to the
explicit `ArrayInlineRecordProbe` runtime builder:

- `src/boxes/array/inline_record_plan_probe.rs`
- `ArrayInlineRecordPlanProbe::build_integer_lane_array(...)`

The adapter accepts `inline_record_columns_v0` plans whose columns are
integer-lane storage classes and whose field/column counts match. It rejects
unsupported storage shapes before an `ArrayBox` is built.

## Non-Goals

C206d does not:

- enable compiler auto-use of `ArrayStorage::InlineRecord`;
- expose a public ArrayBox constructor or source-level API;
- migrate `hako_alloc` metadata stores to packed ArrayBox storage;
- materialize record values as ordinary boxes;
- lower backend, `.inc`, LLVM, provider, hook, or native allocator behavior.

## Acceptance

Run:

```bash
bash tools/checks/k2_wide_arraybox_inline_record_plan_probe_guard.sh
bash tools/checks/k2_wide_arraybox_inline_record_probe_guard.sh
```

The adapter must stay `#[cfg(test)]`, accept integer-lane plans, and reject
handle columns.
