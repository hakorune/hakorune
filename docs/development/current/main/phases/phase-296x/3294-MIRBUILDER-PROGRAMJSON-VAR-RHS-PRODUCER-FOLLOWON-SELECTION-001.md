# 3294 - MIRBUILDER-PROGRAMJSON-VAR-RHS-PRODUCER-FOLLOWON-SELECTION-001

## Purpose
Select the next boundary after the top-level If Var rhs cond_recipe producer
row.

## Decision
Select `MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-COND-SCAN-SEAM-001`.

Loop nested If is the next semantic producer target, but direct row
implementation would edit `LoopStmtHandler` while it is already near the
800-line source limit and while its body scan still publishes Int-only
`if_cond_rhs_int` fields. The next step is therefore a BoxShape seam, not a
new accepted row.

## Explicit Non-Claims
- Loop nested If Var rhs row implementation: `0`
- Top-level Loop Var rhs row: `0`
- Length / length-minus bounds: `0`
- Accepted row added: `0`
- MIR Compare/Branch emission: `0`
- BasicBlock mutation / ValueId allocation: `0`
- Route selection / runtime route switch: `0`
- ProgramJSON runtime authority / runtime fallback: `0`
- Source Selfhost: `0`

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_var_rhs_producer_followon_selection_guard.sh
```
