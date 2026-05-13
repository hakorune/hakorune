# 293x-219: C206b ArrayBox Inline-Record Probe

Status: Complete

## Scope

C206b adds an explicit test-only probe owner for `ArrayStorage::InlineRecord`
residence:

- `src/boxes/array/inline_record_probe.rs`
- `ArrayInlineRecordProbe::build(layout_id, columns)`

The probe centralizes test construction of inline-record arrays while preserving
the existing visible materialization stop line.

## Non-Goals

C206b does not:

- enable compiler auto-use of `ArrayStorage::InlineRecord`;
- expose a public ArrayBox constructor or source-level API;
- migrate `hako_alloc` metadata stores to packed ArrayBox storage;
- materialize record values as ordinary boxes;
- touch backend, `.inc`, LLVM, provider, hook, or native allocator behavior.

## Acceptance

Run:

```bash
bash tools/checks/k2_wide_arraybox_inline_record_probe_guard.sh
bash tools/checks/k2_wide_arraybox_inline_record_storage_guard.sh
```

The probe must stay `#[cfg(test)]` and must not be referenced outside
`src/boxes/array` tests.
