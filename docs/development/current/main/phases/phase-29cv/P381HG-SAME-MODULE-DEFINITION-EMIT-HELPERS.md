# P381HG Same-Module Definition Emit Helpers

Date: 2026-05-06
Scope: deduplicate local same-module definition lookup/read preparation in the
Stage0 same-module emit seam.

## Context

The tail of `hako_llvmc_ffi_same_module_function_emit.inc` still repeated two
small setup patterns around same-module definition emission:

- planned-symbol lookup did `find_index(...)` and then fetched the same function
  again from `program.functions`
- definition emission repeated the `name` read plus the
  `eligible(...)` / `read_view(...)` preparation before the saved-context
  pipeline

The seam was local to Stage0 same-module emission and did not require any new
owner, target shape, or Program(JSON v0) widening.

## Change

- Added `same_module_function_find_planned_definition(...)` to resolve the
  planned definition's `fn` + `fn_index` in one helper and keep the entry-skip
  check local.
- Added `same_module_function_prepare_definition(...)` to share the repeated
  `name` lookup plus `same_module_function_definition_is_eligible(...)` /
  `same_module_function_read_view(...)` preparation.
- Rewired `emit_same_module_function_definition(...)` and
  `emit_planned_same_module_function_definition(...)` to use those helpers.

No behavior changed:

- planned same-module symbols still resolve by exact function name
- entry remains skipped
- eligibility and generic-pure view validation stay identical
- the saved-context → activate-context → emit-pipeline flow is unchanged

## Validation

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

The same-module definition emit seam now keeps one local lookup helper and one
local preparation helper instead of repeating the setup across the planned-entry
and emit-entry boundaries.
