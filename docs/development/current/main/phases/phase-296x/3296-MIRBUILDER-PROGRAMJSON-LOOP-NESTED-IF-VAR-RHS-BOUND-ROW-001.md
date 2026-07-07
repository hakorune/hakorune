# 3296 - MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-VAR-RHS-BOUND-ROW-001

## Purpose
Open one exact Loop-body nested If Var-rhs compare producer row after the scan
seam extraction.

## Contract
- Owner: `LoopStmtHandler` with `LoopNestedIfCondScanBox`
- Shape: Loop body nested `If(i < n) { return ... }` followed by assignment
- Guard route: ProgramJSON -> `LoopStmtHandler.handle_state_values`
  owner-direct AOT
- `cond_rhs_kind_code=2` and `cond_rhs_symbol_id` are published for the nested
  If condition facts
- `cond_rhs_int` is not published for this Var-rhs row
- The nested If `cond_recipe` remains analysis-only and read-only
- Top-level Loop condition support remains Int-rhs only in this card

## Explicit Non-Claims
- Top-level Loop Var-rhs row: `0`
- Length / length-minus bounds: `0`
- Reversed Var/Var context-aware canonicalization: `0`
- MIR Compare/Branch emission: `0`
- BasicBlock mutation / ValueId allocation: `0`
- Route selection / runtime route switch: `0`
- Full phase-state dispatcher authority: `0`
- ProgramJSON runtime authority / runtime fallback: `0`
- Source Selfhost: `0`

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_loop_nested_if_var_rhs_bound_row_gate.sh
```
