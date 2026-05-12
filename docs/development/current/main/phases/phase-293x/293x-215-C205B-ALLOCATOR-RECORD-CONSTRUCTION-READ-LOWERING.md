# 293x-215: C205b Allocator Record Construction/Read Lowering

Status: Complete

## Purpose

Make record values usable for small allocator metadata probes without turning
records into ordinary boxes.

C205b adds the first record value lowering seam:

- `new Record(args...)` is accepted only in builder-controlled scalarization
  contexts.
- direct record field reads return the constructor operand for that field.
- record values that escape as objects fail fast with stable diagnostics.

## Lowering Contract

The C205b route is builder-local:

```hako
local meta = new HakoAllocAlignedSmallMeta(ptr, alignment, padded_size)
local ptr2 = meta.ptr
```

lowers `meta.ptr` to the original `ptr` value. It does not emit a record
`NewBox`, does not materialize an object, and does not enter typed-object
plans.

Supported first-cut reads:

- `local meta = new Record(...); meta.field`
- `new Record(...).field`

Fail-fast boundaries:

- using the record value directly
- returning, printing, passing, or storing the record value as an object
- assigning to a record field
- unknown field
- constructor arity mismatch
- generic record construction

## Stop Line

C205b does not:

- migrate `hako_alloc` scalar metadata arrays
- enable `ArrayStorage::InlineRecord` from compiler lowering
- add record materialization or identity semantics
- reuse ordinary user-box factories, `NewBox`, or typed-object plans
- add backend, `.inc`, Python, provider, or native allocator lowering

## Acceptance

- a record construction plus field read emits MIR for the scalar field value
  without a `NewBox` for the record and without a record `field_get` in `main`
- escaped record values fail with `[record-value/escape]` or
  `[record-construction/escape]`
- unknown fields fail with `[record-field-read/unknown-field]`
- C205a scalar metadata arrays remain the live `hako_alloc` runtime truth
