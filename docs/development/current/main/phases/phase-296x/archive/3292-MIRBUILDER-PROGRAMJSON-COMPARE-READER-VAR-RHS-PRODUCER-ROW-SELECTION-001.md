# 3292 - MIRBUILDER-PROGRAMJSON-COMPARE-READER-VAR-RHS-PRODUCER-ROW-SELECTION-001

Status: landed

## Purpose

Select the first producer row that consumes the shared compare reader's
`rhs=Var` / `SymbolRef` output.

## Decision

Select:

```text
MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-VAR-RHS-BOUND-ROW-001
```

The selected row is a narrow top-level If producer row:

```text
If Compare(lhs=Var, op=<, rhs=Var), then Return, else null
```

It should attach a `BoolRecipe::Compare` cond_recipe with `bound_kind=SymbolRef`.

## Rationale

Top-level If is the narrowest producer boundary. It can prove the
`Var op Var` reader output is usable by a RecipeItem cond_recipe sidecar
without opening Loop route/release semantics, Loop body-shape consumers, MIR
Compare/Branch emission, BasicBlock mutation, or ValueId allocation.

Loop nested If and top-level Loop remain deferred because they still have
multiple `cond_rhs_int` consumers in body-shape and route-sensitive surfaces.

## Gate

`tools/checks/rust_lifecycle_mirbuilder_programjson_compare_reader_var_rhs_producer_row_selection_guard.sh`

## Claims

- `var_rhs_producer_row_selection = 1`
- `top_level_if_var_rhs_row_selected = 1`

## Non-Claims

- top-level If Var rhs row implementation
- Loop nested If Var rhs producer row
- top-level Loop Var rhs producer row
- length / length-minus bound producer rows
- BoolRecipe lowering execution
- MIR Compare emission
- Branch emission
- BasicBlock mutation
- ValueId allocation
- route selection
- runtime route switch
- ProgramJSON runtime route authority
- runtime fallback
- Source Selfhost

## Next

`MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-VAR-RHS-BOUND-ROW-001`
