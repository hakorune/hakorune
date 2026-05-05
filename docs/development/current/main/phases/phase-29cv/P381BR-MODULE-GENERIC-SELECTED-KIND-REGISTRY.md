---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: consolidate Stage0 module generic selected-set contract side registries
Related:
  - docs/development/current/main/phases/phase-29cv/P381BC-STAGE0-CAPSULE-EXIT-TASK-MAP.md
  - docs/development/current/main/phases/phase-29cv/P381BQ-GENERIC-STRING-OR-VOID-SENTINEL-TARGET-SHAPE-RETIRE.md
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_plan.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P381BR: Module Generic Selected Kind Registry

## Problem

After the temporary target-shape capsules were retired, Stage0 still carried
parallel selected-set registries for parser Program(JSON) and static string
array definitions:

- `planned_module_parser_program_json_symbols[]`
- `planned_module_static_string_array_symbols[]`

Both registries duplicated the already-selected generic same-module symbol set.
That made the selected-set owner boundary wider than necessary before the
uniform emitter cleanup.

## Decision

Store direct-contract identity as a kind on
`planned_module_generic_string_symbols[]`.

The selected-set registry now records:

- generic body
- parser Program(JSON) direct contract
- static string array direct contract

This removes the parallel parser/static planned-symbol arrays. The body emitter
still uses the kind to choose the existing parser Program(JSON) body path and to
allow static string array body behavior. This card does not delete those body
paths.

## Boundary

Allowed:

- consolidate selected-set bookkeeping
- preserve existing body emission behavior
- keep parser/static direct contracts MIR-owned through LoweringPlan facts

Not allowed:

- infer parser/static identity from source-owner names
- delete parser/static body emitters in this card
- add new target shapes or new C body-specific emitters

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
bash tools/checks/dev_gate.sh quick
```

## Result

Done:

- parser Program(JSON) and static string array planned symbols are now selected
  generic symbols with a stored kind
- parallel parser/static planned-symbol arrays and lookup helpers are removed
- existing parser/static body behavior remains intact for the next cleanup

Next:

1. shrink body-emitter special cases behind the same selected-kind registry
2. continue toward uniform MIR function body emission
