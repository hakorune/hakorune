# P381HA Same-Module Function Find Helper

Date: 2026-05-07
Scope: deduplicate same-module function lookup loops in the Stage0 same-module
function-plan seam.

## Context

After P381GW, the same-module function-plan seam still duplicated the same
function-name scan in two helpers:

- `same_module_function_find(...)`
- `same_module_function_find_index(...)`

Both walked `program.functions` with the same name match logic and only differed
in whether they returned the function pointer or the index.

## Change

- Added `same_module_function_find_with_index(...)` as the shared lookup helper.
- Rewired `same_module_function_find(...)` to call it with `NULL`.
- Rewired `same_module_function_find_index(...)` to treat its non-NULL result as
  success.

No behavior changed:

- function lookup still matches by exact function name
- callers still receive either the `yyjson_val*` or the discovered index
- Stage0 ownership, lowering-plan routing, and same-module emission semantics
  are unchanged

## Result

The same-module function-plan seam now owns one lookup loop instead of two,
continuing the targeted helper dedup lane without widening accept shapes.

## Validation

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
