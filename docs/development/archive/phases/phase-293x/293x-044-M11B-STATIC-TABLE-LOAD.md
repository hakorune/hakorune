---
Status: Landed
Date: 2026-05-08
Lane: phase-293x real-app bringup
Card: 293x-044-M11B-STATIC-TABLE-LOAD
Scope: M11b-load static const table read route
---

# 293x-044 M11b Static Table Load

## Decision

The first M11b table-read row is live for source-declared `u16` static const
tables.

Accepted source shape:

```hako
static const SIZE_CLASS: u16[] = [8, 16, 24, 32]

static box Main {
  main() {
    local i = 2
    return SIZE_CLASS[i]
  }
}
```

This row adds a MIR-owned static-data load operation. It does not add const
evaluation, const fn, general pointer arithmetic, or runtime collection
publication.

## Flow

```text
source Index(NAME, index)
-> MIR StaticDataLoad when NAME matches module static_data_plans
-> MIR JSON op static_data_load
-> VM reads module static_data_plans with bounds fail-fast
-> .hako ll_emit emits direct LLVM getelementptr/load/zext
```

## Responsibility

- Source parsers already own the `static const NAME: u16[] = [...]`
  declaration shape.
- MIR owns the decision that `NAME[index]` is a static-data load.
- MIR JSON carries the load row fields needed by backend readers:
  `source_name`, `symbol`, `element`, `len`, `align`, and `index`.
- VM reads the same module metadata row as the reference runtime behavior.
- `.hako` ll_emit reads the JSON row and emits a direct static global load.
- Backend emitters do not infer allocator semantics from `SIZE_CLASS` or any
  other table name.

## Accepted Limits

- Element type: `u16` only.
- Result lane: current `i64` value lane via zero-extension.
- Index: any current `i64` expression in source/MIR.
- VM bounds: fail-fast on negative or out-of-range index.
- LLVM bounds proof: not active yet. The first row emits direct load and relies
  on the source/fixture/gate contract; checked runtime helper lowering is a
  future row if needed.

## Non-Goals

- No `ArrayBox` / `MapBox` table object.
- No table mutation.
- No `u8/u32/u64` table reads.
- No compile-time expression evaluation in initializers.
- No const fn.
- No allocator-specific C shim switch.

## Gates

```bash
bash tools/checks/k2_wide_static_const_table_load_guard.sh
bash tools/checks/k2_wide_static_const_table_decl_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
