# 3283 - MIRBUILDER-BOOL-RECIPE-COMPARE-LOWERING-BOUNDARY-CONSULTATION-001

Status: landed

## Purpose

Decide the first BoolRecipe::Compare lowering-facing boundary after shared
If/Loop `cond_recipe` parity.

## Decision

Select an observe-only lowering intent pilot:

```text
BoolRecipe::Compare
  -> BoolRecipeCompareLoweringIntentSnapshotV1
```

Do not emit MIR Compare or Branch instructions in this card.

Current evidence:

- `BoolRecipeBox` is data-only and explicitly non-responsible for MIR
  Compare/Branch emission, lowering, mutation, ID allocation, and route switch.
- The current BoolRecipe publication is read-only and keeps
  `lowering_executed=0`.
- Rust Compare emission is owned by `ops/comparison.rs` and
  `emission/compare.rs`.
- Branch emission is owned by `emission/branch.rs` and control-flow lowering
  entries.

Actual MIR emission would mix value resolution, block ownership, ValueId
allocation, route/lowering integration, and mutation. The next slice should
first define the read-only intent boundary.

## Gate

`tools/checks/rust_lifecycle_mirbuilder_bool_recipe_compare_lowering_boundary_consultation_guard.sh`

The guard proves:

- the consultation selects the observe-only lowering intent pilot;
- BoolRecipe and publication surfaces remain non-lowering;
- MIR Compare/Branch emission owners remain Rust-side and unclaimed;
- route selection, runtime switch, fallback, and Source Selfhost remain zero.

## Claims

- `lowering_boundary_consultation = 1`
- `observe_only_lowering_intent_next = 1`
- `bool_recipe_compare_lowering_intent_selected = 1`

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

`MIRBUILDER-BOOL-RECIPE-COMPARE-LOWERING-OBSERVE-ONLY-PILOT-001`
