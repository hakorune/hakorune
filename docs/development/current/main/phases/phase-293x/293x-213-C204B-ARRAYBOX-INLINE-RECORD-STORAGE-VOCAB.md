# 293x-213: C204b ArrayBox Inline-Record Storage Vocabulary

Status: Complete

## Purpose

Add the runtime-private `ArrayBox` storage vocabulary needed by the
record/packed-array lane without enabling compiler auto-use or allocator
metadata migration yet.

C204a described the future columnar shape in MIR metadata. C204b gives
`ArrayBox` an internal storage variant for that shape:

```text
ArrayStorage::InlineRecord(ArrayInlineRecordStorage)
```

## Runtime Vocabulary

`ArrayInlineRecordStorage` carries:

- `layout_id`
- row `len`
- scalar columns (`i64`, `bool`, `f64` in this row)

Supported in this row:

- len / capacity
- reserve / clear
- clone
- same-lane structural equality
- stable debug / string summary
- slice preserving inline-record storage

## Materialization Boundary

C204b intentionally does not create visible record objects from
`InlineRecord` rows.

Visible element operations that would need record materialization return a
stable unmaterialized diagnostic boundary or fail the raw helper:

```text
[array/inline-record/unmaterialized] ...
```

This keeps C204b as storage vocabulary only. A later row may add boxed record
materialization once record construction/read lowering is ready.

## Stop Line

C204b does not add:

- compiler auto-use of `ArrayStorage::InlineRecord`
- hako_alloc metadata migration
- record construction/read lowering
- boxed record materialization
- backend lowering
- `.inc` / Python lowering matchers
- provider, hook, or process allocator replacement behavior

## Acceptance

- `ArrayStorage::InlineRecord` exists only under `src/boxes/array`.
- Tests cover len/capacity/debug, materialization boundary, clone/equality,
  clear, and slice behavior.
- Public ArrayBox behavior does not silently fabricate record objects.
- `lang/src/hako_alloc/**`, `lang/c-abi/shims/**`, and
  `src/llvm_py/instructions/**` do not mention inline-record storage.
