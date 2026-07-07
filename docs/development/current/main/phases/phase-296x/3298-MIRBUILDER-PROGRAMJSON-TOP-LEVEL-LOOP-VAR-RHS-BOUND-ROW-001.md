# 3298 - MIRBUILDER-PROGRAMJSON-TOP-LEVEL-LOOP-VAR-RHS-BOUND-ROW-001

## Purpose
Open one exact top-level Loop Var-rhs compare producer row after the If and
Loop nested-If Var-rhs rows.

## Contract
- Owner: `LoopStmtHandler`
- Shape: `loop(i < n) { i = i + 1 }`
- Guard route: ProgramJSON -> `LoopStmtHandler.handle_state_values`
  owner-direct AOT
- `cond_rhs_kind_code=2` and `cond_rhs_symbol_id` are published for Loop
  condition facts
- `cond_rhs_int` is not published for this Var-rhs row
- The Loop `cond_recipe` remains analysis-only and read-only

## Explicit Non-Claims
- Full phase-state dispatcher authority: `0`
- Legacy Loop DTO/lowering update: `0`
- Length / length-minus bounds: `0`
- MIR Compare/Branch emission: `0`
- BasicBlock mutation / ValueId allocation: `0`
- Route selection / runtime route switch: `0`
- ProgramJSON runtime authority / runtime fallback: `0`
- Source Selfhost: `0`

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_top_level_loop_var_rhs_bound_row_gate.sh
```
