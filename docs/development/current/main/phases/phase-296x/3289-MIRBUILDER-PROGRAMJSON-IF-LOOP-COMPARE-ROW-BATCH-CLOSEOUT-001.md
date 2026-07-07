# 3289 - MIRBUILDER-PROGRAMJSON-IF-LOOP-COMPARE-ROW-BATCH-CLOSEOUT-001

Status: landed

## Purpose

Close the shared If/Loop ProgramJSON compare row batch before opening any
mutation-bearing lowering owner.

## Closed Surface

The closed surface is:

```text
ProgramJSON Compare
  -> shared compare reader
  -> BoolRecipe::Compare cond_recipe sidecar
```

Covered rows:

- top-level If `< <= > >=`: 4 rows
- Loop-body nested If `< <= > >=`: 4 rows

Legacy `cond_facts` remain present and preserved for the covered rows.

## Gate

`tools/checks/rust_lifecycle_mirbuilder_programjson_if_loop_compare_row_batch_closeout_guard.sh`

The guard runs:

- `rust_lifecycle_mirbuilder_programjson_if_cond_recipe_relational_row_batch_gate.sh`
- `rust_lifecycle_mirbuilder_programjson_loop_nested_if_cond_recipe_relational_row_batch_gate.sh`
- `rust_lifecycle_mirbuilder_bool_recipe_compare_lowering_emission_consultation_guard.sh`

## Claims

- `if_loop_compare_row_batch_closeout = 1`
- `top_level_if_relational_rows_closed = 1`
- `loop_nested_if_relational_rows_closed = 1`
- `shared_compare_reader_used = 1`
- `legacy_cond_facts_preserved = 1`
- `compare_reader_followon_selection_next = 1`

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

`MIRBUILDER-PROGRAMJSON-COMPARE-READER-FOLLOWON-SELECTION-001`
