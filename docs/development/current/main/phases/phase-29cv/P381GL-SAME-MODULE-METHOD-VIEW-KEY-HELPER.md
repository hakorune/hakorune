# P381GL Same-Module Method View Key Helper

Date: 2026-05-06
Scope: put MIR JSON `key_const_text` ownership on the same-module method-view seam.

## Context

`hako_llvmc_ffi_same_module_function_emit.inc` already had a helper for reading
`key_const_text` from a `LoweringPlanGenericMethodView`, while
`hako_llvmc_ffi_same_module_method_views.inc` repeated the same raw read across
the MIR JSON map-field predicates.

The method-view file owns LoweringPlan generic-method predicate vocabulary, so
the key reader belongs there. Emit-side origin publication should consume the
same helper instead of carrying a duplicate local definition.

## Change

- Moved the `same_module_method_view_key_const_text` helper to
  `hako_llvmc_ffi_same_module_method_views.inc`.
- Rewired MIR JSON field predicates to use that helper instead of repeating the
  raw `read_str(entry, "key_const_text")` expression.
- Removed the duplicate helper definition from
  `hako_llvmc_ffi_same_module_function_emit.inc`.

## Result

The same-module method-view seam now owns both:

- route/proof/shape predicate helpers
- MIR JSON field-key observation helper

This is behavior-preserving BoxShape cleanup. It does not change the accepted
field-key lists or widen any Stage0 route.

## Validation

```bash
bash tools/build_hako_llvmc_ffi.sh
git diff --check
```
