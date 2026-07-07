# 3297 - MIRBUILDER-PROGRAMJSON-VAR-RHS-PRODUCER-NEXT-SELECTION-001

## Purpose
Select the next Var-rhs compare producer after top-level If and Loop nested-If
Var-rhs rows are green.

## Decision
Select `MIRBUILDER-PROGRAMJSON-TOP-LEVEL-LOOP-VAR-RHS-BOUND-ROW-001`.

The selected row is owner-direct and observe-only:

- Guard route: ProgramJSON -> `LoopStmtHandler.handle_state_values`
  owner-direct AOT
- `cond_recipe` may carry `SymbolRef`
- legacy Loop DTO/lowering consumers that require `loop_cond_rhs_int` remain
  unchanged

## Explicit Non-Claims
- Top-level Loop Var-rhs implementation: `0`
- Length / length-minus bounds: `0`
- Full phase-state dispatcher authority: `0`
- MIR Compare/Branch emission: `0`
- BasicBlock mutation / ValueId allocation: `0`
- Route selection / runtime route switch: `0`
- ProgramJSON runtime authority / runtime fallback: `0`
- Source Selfhost: `0`

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_var_rhs_producer_next_selection_guard.sh
```
