# 3320 - MIRBUILDER-COMPARE-BOOLRECIPE-TO-MIR-COMPARE-BRANCH-CLOSEOUT-001

## Purpose
Close the scoped BoolRecipe-to-MIR Compare/Branch bridge chain.

## Boundary
This card is a closeout proof. It does not add a new Rust owner. It proves the
previous bridge sequence is green up to conditional Branch emission:

```text
RHS ValueId request/response ABI
LiteralI64 constant emission bridge
SymbolRef no-shadow local lookup bridge
LocalSSA finalize_compare bridge
MIR Compare emission bridge
Branch emission bridge
```

Runtime route authority remains unchanged. The next card is a design-stop for
whether and where this bridge chain may approach runtime route authority.

## Positive Claims
- `boolrecipe_to_mir_compare_branch_closeout = 1`
- `compare_branch_lowering_bridge_chain_green = 1`
- RHS ValueId, LocalSSA, Compare emission, and Branch emission bridge gates are
  green.

## Explicit Non-Claims
- route selection / runtime route switch: `0`
- ProgramJSON runtime authority / runtime fallback: `0`
- Source Selfhost: `0`

## Guard
```bash
bash tools/checks/rust_lifecycle_mirbuilder_compare_boolrecipe_to_mir_compare_branch_closeout_guard.sh
```
