# 3288 - MIRBUILDER-BOOL-RECIPE-COMPARE-LOWERING-EMISSION-CONSULTATION-001

Status: landed

## Purpose

Decide whether the read-only `BoolRecipe::Compare` lowering intent may start
emitting MIR Compare/Branch instructions.

## Decision

Do not open MIR emission yet. Select compare row batch closeout first:

```text
MIRBUILDER-PROGRAMJSON-IF-LOOP-COMPARE-ROW-BATCH-CLOSEOUT-001
```

The current evidence proves shared reader / cond_recipe / observe-only lowering
intent coverage, but not the mutation-bearing lowering owner.

MIR emission still needs separate ownership for:

- operand `ValueId` resolution
- rhs bound materialization
- Compare destination `ValueId` allocation
- Branch target `BasicBlock` ownership
- observe-only parity before any runtime authority switch

## Gate

`tools/checks/rust_lifecycle_mirbuilder_bool_recipe_compare_lowering_emission_consultation_guard.sh`

The guard proves:

- the observe-only lowering intent pilot is green;
- the Loop nested If relational row batch is green;
- MIR Compare/Branch emission remains Rust-owned and unclaimed;
- the next card is row batch closeout, not lowering emission.

## Claims

- `lowering_emission_consultation = 1`
- `compare_row_batch_closeout_next = 1`
- `bool_recipe_compare_emission_deferred = 1`

## Non-Claims

- BoolRecipe lowering executed
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

`MIRBUILDER-PROGRAMJSON-IF-LOOP-COMPARE-ROW-BATCH-CLOSEOUT-001`
