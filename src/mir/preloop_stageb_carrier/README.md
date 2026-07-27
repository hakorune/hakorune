# Pre-loop Stage-B carrier boundary

This module owns the bounded source proof for one selected pre-loop carrier
handoff. It joins already-sealed authorities; it does not infer a result.

Owned here:

- the exact static `ExactI64` requirement projected by
  `callable_result_representation`;
- the structural selected `CallArgument(1)`;
- the existing sealed nested instance Integer contract;
- typed rejection before Builder effects.
- one owned, non-Clone activation row;
- one plan that owns the exact declaration-catalog allocation.

Not owned here:

- source navigation or AST re-walk;
- general callable-result solving;
- Builder, `ValueId`, `MirType`, or `TypeContext`;
- function activation, Call emission, assignment, or publication;
- production source inventory or function selection;
- fallback, retry, or route reselection.

The only construction path is:

```text
VerifiedStaticExactI64RequirementV1
  + PreparedPreloopLocatedArgumentV1
  -> seal_preloop_outer_carrier_result_v1
  -> SealedPreloopOuterCarrierResultContractV1
  -> prepare_preloop_stageb_carrier_rows_v1
  -> PreparedPreloopStageBCarrierRowsV1
  + exact Box<VerifiedSameModuleCallableDeclarationCatalogV1>
  -> VerifiedPreloopStageBCarrierActivationPlanV1
```

The first profile requires the exact structural argument ordinal `1` and the
exact static requirement set `[1]`. A broader required-argument solver belongs
to a separate decision.
