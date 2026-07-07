# 3299 - MIRBUILDER-PROGRAMJSON-VAR-RHS-PRODUCER-CLOSEOUT-001

## Purpose
Close the first Var-rhs compare producer surface after three owner-direct rows
are green:

- top-level If Var-rhs condition
- Loop nested-If Var-rhs condition
- top-level Loop Var-rhs condition

## Contract
- Closeout only; no new accepted row in this card
- Guard route remains owner-direct AOT rows only
- Full phase-state dispatcher authority remains unclaimed
- Legacy Loop DTO/lowering consumers that require `cond_rhs_int` remain
  unchanged

## Explicit Non-Claims
- Length / length-minus bounds: `0`
- Reversed Var/Var context-aware canonicalization: `0`
- MIR Compare/Branch emission: `0`
- BasicBlock mutation / ValueId allocation: `0`
- Route selection / runtime route switch: `0`
- ProgramJSON runtime authority / runtime fallback: `0`
- Source Selfhost: `0`

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_var_rhs_producer_closeout_guard.sh
```
