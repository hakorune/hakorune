---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: phase-29cv selected declaration tightening for same-module ny-llvmc emission
Related:
  - docs/development/current/main/phases/phase-29cv/P381AV-SELECTED-SET-FIRST-SLICE.md
  - docs/development/current/main/phases/phase-29cv/P381AT-UNIFORM-MULTI-FUNCTION-EMITTER-GAP-PLAN.md
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
---

# P381AW: Selected Declarations Only

## Problem

After P381AV, Stage0 already emitted same-module bodies from the lowering-plan
selected set, but `emit_module_function_declarations()` still walked a broader
module view than necessary.

That left one remaining mismatch:

- selected bodies were planner-owned
- same-module declarations were still derived from a broader module scan

So unselected same-module functions could still appear as implicit external
declarations even though direct same-module calls are now expected to pass
through selected-set/fail-fast gates.

## Decision

Tighten module-function declarations to the same selected symbol inventory that
drives same-module body emission.

Implemented:

- `emit_module_function_declarations()` now iterates
  `planned_module_generic_string_symbols[]`
- it resolves the corresponding same-module function and emits a forward
  declaration only for that selected symbol
- unselected same-module functions are no longer emitted as declarations by
  default

## Boundary

Allowed:

- declaration narrowing that follows the existing lowering-plan selected set
- forward declarations for selected same-module functions that are defined later

Not allowed:

- reintroducing whole-module declaration scans
- adding new body semantics to Stage0
- relaxing fail-fast checks for unsupported same-module direct calls

## Acceptance

```bash
cargo fmt --all
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

## Result

Done:

- same-module declarations now follow the selected-set owner instead of the
  broader module inventory
- unselected same-module functions are no longer implicitly externalized by the
  declaration pass

Next:

1. retire temporary `GlobalCallTargetShape::*Body` capsules now that selected
   decls + bodies are aligned
2. finish the delete-last compat sweep after those capsules are gone
