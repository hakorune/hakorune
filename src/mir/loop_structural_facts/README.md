# Loop Structural Facts

This module is the neutral, Builder-free boundary from sealed resolved Loop
identity to the portable Loop recipe source binding.

Authority flows in one direction:

```text
VerifiedResolvedFunctionV1::resolved_loop_source(site)
  -> VerifiedResolvedLoopSourceV1
  -> bind_resolved_loop_root_v1(token)
  -> VerifiedLoopRootSourceV1
  -> into_root_claim(&VerifiedLoopRecipeV1)
  -> LoopRecipeSourceBindingV1
```

`VerifiedResolvedLoopSourceV1` and `VerifiedLoopRootSourceV1` are local source
authority: both originate in the sealed resolved product and are non-Clone.
`LoopRecipeSourceBindingV1` is different. It is a serializable wire claim, and
even the contract module's structurally verified claim capability proves only
recipe coverage and path shape—not source existence or AST correspondence.

The adapter consumes the non-Clone resolved token. Root-key binding consumes
the local authority and reads the key only from `VerifiedLoopRecipeV1`; an
arbitrary `LoopNodeKeyV1` cannot be injected. The adapter must not scan AST,
inspect `LoopRouteContext`, read route-local facts, infer an index from body
contents, or call Builder/Planner/Lower. Only the declared-function root and
the closed `Body` / `ScopeBody` / `LoopBody` statement lineage are portable in
this slice. Lambda and Program owners, every other ancestor, and orphan
body-root markers fail with typed rejection.

`selected_demand.rs` is the next neutral handoff. It consumes one opaque policy
winner, one AST-free structural identity witness, and one exact resolved-source
capability, then returns a non-Clone selected demand. It performs identity
checking only; it does not create a Recipe, select a family, or touch PHI/SSA.
Direct Accum is caller-zero evidence. Nested source-bound is not claimed until
the resolver can issue a sealed root-plus-child forest capability.

The Direct Accum S0 projection keeps the AST-bearing observation in
`mir/compiler/direct_accum_projection.rs`. That adapter navigates only through
`FunctionSourceViewV1` and the shared child-role vocabulary, then issues the
AST-free `DirectAccumStructuralShapeV1` product. `VerifiedResolvedFunctionV1`
resolves its exact expression sites to `BindingRefV1`; names and raw indices
are never re-resolved. A source-issued `LoopExecutionFrameKeyV1` is carried by
the winner, facts, and source capability so selected demand rejects a foreign
execution frame before sealing.

This S0 remains caller-zero: it does not construct a Recipe, invoke the
legacy scheduler, mutate a Builder, or become a PHI/SSA writer. Production
physical ownership remains `CanonicalCfgSessionV1` + function-owned
`BindingSsaBuilderV1` + `PhiTxn`.

M3-B and the selected-demand issuer are intentionally caller-zero. Production
wiring belongs to a later card at the located source carrier before Loop syntax
is decomposed.
