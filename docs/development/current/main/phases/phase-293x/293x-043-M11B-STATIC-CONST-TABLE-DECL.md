---
Status: Landed
Date: 2026-05-08
Lane: phase-293x real-app bringup
Card: 293x-043-M11B-STATIC-CONST-TABLE-DECL
Scope: M11b-decl source static const table declaration
---

# 293x-043 M11b Static Const Table Decl

## Decision

The first M11b declaration-only row is live.

Accepted source shape:

```hako
static const SIZE_CLASS: u16[] = [
  8, 16, 24, 32,
]
```

This row only declares readonly static data. It does not add table reads,
const expressions, or const fn.

## Flow

```text
Rust parser / .hako parser
-> AST or Program(JSON v0) static_data_plans
-> MIR module metadata static_data_plans
-> .hako ll_emit StaticDataRegistryBox row reader
-> LLVM readonly global
```

## Responsibility

- Parsers own only the narrow source declaration shape.
- MIR/module metadata owns `static_data_plans`.
- `StaticDataRegistryBox` reads rows and emits globals.
- Backend emitters do not infer allocator table meaning from names.
- Runtime does not construct `ArrayBox` / `MapBox` for fixed static tables.

## Accepted Limits

- Element type: `u16` only.
- Initializer: integer literals only.
- Range: `0..65535`.
- Table read syntax: unsupported.
- Const eval / const fn: unsupported.

## Gates

```bash
bash tools/checks/k2_wide_static_const_table_decl_guard.sh
bash tools/checks/k2_wide_static_data_first_row_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
