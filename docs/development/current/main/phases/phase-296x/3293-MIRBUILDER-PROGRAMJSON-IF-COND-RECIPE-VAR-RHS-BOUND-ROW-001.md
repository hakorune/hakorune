# 3293 - MIRBUILDER-PROGRAMJSON-IF-COND-RECIPE-VAR-RHS-BOUND-ROW-001

## Purpose
Open one exact top-level If producer row for the shared ProgramJSON Compare
reader's Var rhs bound output.

## Scope
- Owner: `IfStmtHandler`
- Shape: `If Compare(lhs=Var, op=<, rhs=Var), then Return, else=null`
- Output: `RecipeItem.cond_recipe` with `BoolRecipe::Compare` and
  `bound_kind=SymbolRef`

## Contract
- `cond_rhs_kind=Var` and `cond_rhs_symbol_id` are published in legacy
  `cond_facts`.
- `cond_rhs_int` is not published for this Var rhs row.
- The row remains analysis-only.

## Explicit Non-Claims
- Loop nested If Var rhs row: `0`
- Top-level Loop Var rhs row: `0`
- Length / length-minus bounds: `0`
- Reversed Var/Var context-aware inference: `0`
- MIR Compare/Branch emission: `0`
- BasicBlock mutation / ValueId allocation: `0`
- Route selection / runtime route switch: `0`
- ProgramJSON runtime authority / runtime fallback: `0`
- Source Selfhost: `0`

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_if_cond_recipe_var_rhs_bound_row_gate.sh
```
