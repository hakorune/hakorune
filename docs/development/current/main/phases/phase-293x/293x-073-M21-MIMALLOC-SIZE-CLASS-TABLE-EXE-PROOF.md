---
Status: done
Date: 2026-05-09
Scope: M21 mimalloc size-class table EXE proof
---

# 293x-073 M21 Mimalloc Size-Class Table EXE Proof

## Decision

`M21 mimalloc size-class table EXE proof` locks the first allocator-shaped app
slice that composes source `static const u16[]` size-class tables with the
M14-M20 raw-page pure-first EXE route surface.

This row proves that an allocator app can use MIR-owned `static_data_plans` for
size-class metadata instead of constructing runtime `ArrayBox` / `MapBox`
tables. It adds only the narrow pure-first reader for MIR-owned
`static_data_plans` and `static_data_load` over `u16` elements. The C shim emits
readonly globals and loads only from MIR facts; it must not infer table names or
allocator policy.

## Owned

- `apps/mimalloc-size-class-table-proof/`
- A pure-first EXE guard for the fixture.
- A narrow pure-first static-data reader/lowerer for `u16` plans.
- MIR JSON checks that:
  - `MI_SIZE_CLASS` and `MI_CLASS_CAP` are static `u16` data plans.
  - the workload reads those tables through `static_data_load`.
  - the raw-page body still routes through RawBuf / RawArray generic-i64 facts.

## Not Owned

- New static table element types.
- Const fn or references to other consts.
- Runtime `ArrayBox` / `MapBox` size-class materialization.
- App-specific symbol matching in `.inc`.
- Bounds-proof widening for static table loads.
- New RawArray vocabulary.
- Native allocator fast-path ownership.

## Acceptance

```bash
bash tools/checks/k2_wide_mimalloc_size_class_table_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Result: green on 2026-05-09.

## Next Reading

After M21, the allocator lane can either add the next allocator app slice that
uses existing route facts, or split the next concrete blocker into a fresh
capability row. Do not make VM parity the default owner for new rows.
