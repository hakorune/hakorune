# P381EJ Uniform MIR Helper Name

Date: 2026-05-06
Scope: clarify Stage0 metadata helper naming for uniform MIR direct-call definitions.

## Context

P381EI clarified the metadata helper for `module_generic` plus
`generic_i64_or_leaf` owner families. The neighboring `uniform_mir` helper still
used candidate wording even though it is an owner-family predicate.

## Change

Renamed:

- `lowering_plan_global_call_view_is_uniform_mir_function_candidate` ->
  `lowering_plan_global_call_view_uses_uniform_mir_definition`

No behavior changed.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/stage0_shape_inventory_guard.sh
git diff --check
```

## Result

The metadata owner-family helpers now use matching `uses_*_definition`
vocabulary for `module_generic_or_i64` and `uniform_mir`.
