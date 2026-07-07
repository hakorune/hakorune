# 3295 - MIRBUILDER-PROGRAMJSON-LOOP-NESTED-IF-COND-SCAN-SEAM-001

## Purpose
Extract the Loop-body nested If Compare condition scan from `LoopStmtHandler`
before opening the next Var rhs producer row.

## Contract
- New owner: `LoopNestedIfCondScanBox`
- Legacy output remains `if_cond_rhs_int` and `if_cond_start`
- Existing Loop nested If relational rows remain green
- No accepted row is added in this card
- `LoopStmtHandler` stays below 800 lines

## Explicit Non-Claims
- Loop nested If Var rhs row: `0`
- Top-level Loop Var rhs row: `0`
- Length / length-minus bounds: `0`
- MIR Compare/Branch emission: `0`
- BasicBlock mutation / ValueId allocation: `0`
- Route selection / runtime route switch: `0`
- ProgramJSON runtime authority / runtime fallback: `0`
- Source Selfhost: `0`

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_loop_nested_if_cond_scan_seam_guard.sh
```
