# P381EB Same-Module Registry Diagnostic

Date: 2026-05-06
Scope: rename the selected same-module registry overflow diagnostic.

## Context

P381DZ and P381EA moved the selected-set registry and entry names to
same-module function wording. The capacity overflow diagnostic still reported
`module_generic_registry_full`, which now pointed at the old owner family
instead of the selected same-module definition contract.

## Change

Renamed the overflow reason to:

```text
same_module_function_registry_full
```

Updated the capacity card text so the documented registry and diagnostic names
match the current code.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/stage0_shape_inventory_guard.sh
git diff --check
```

## Result

Registry overflow diagnostics now name the same-module function registry instead
of the retired module-generic string wording.
