# P381DZ Same-Module Function Registry Names

Date: 2026-05-06
Scope: rename Stage0 selected-set registry symbols after the uniform owner split.

## Context

P381DY renamed the definition requirement helper around the selected
same-module function contract. The selected-set registry still used
`module_generic_string` names even though it now records any same-module
function definition required by MIR metadata, including `uniform_mir` owners.

## Change

Renamed the selected-set registry identifiers from module-generic string wording
to same-module function wording:

- planned symbol registry
- emitted symbol registry
- symbol planned/emitted lookup helpers
- "has planned definition" helper used by runtime declaration prescan

The registry capacity and diagnostic string are unchanged.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/stage0_shape_inventory_guard.sh
git diff --check
```

## Result

The Stage0 selected-set registry now names the contract it actually owns:
same-module function definitions selected from MIR-owned LoweringPlan facts.
