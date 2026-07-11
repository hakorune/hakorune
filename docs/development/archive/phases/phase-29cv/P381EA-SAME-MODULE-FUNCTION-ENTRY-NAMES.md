# P381EA Same-Module Function Entry Names

Date: 2026-05-06
Scope: rename selected same-module function plan and definition entry points.

## Context

P381DZ renamed the selected-set registries to same-module function wording.
The outer planner and definition driver still used older
`module_generic_string` / `generic_pure_string` entry names even though they now
drive both `module_generic` and `uniform_mir` same-module definitions.

## Change

Renamed only the outer entry points:

- same-module function lookup helpers
- selected same-module function planning
- selected same-module function definition emission
- prescan and lowering call sites

The inner `module_generic_string_*` body helpers remain unchanged because they
still own the active body-lowering implementation and need a separate cleanup.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/stage0_shape_inventory_guard.sh
git diff --check
```

## Result

The selected-set entry API now names the same-module function contract. The
remaining `module_generic_string_*` surface is limited to body internals.
