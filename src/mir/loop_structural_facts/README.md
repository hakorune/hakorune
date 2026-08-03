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

M3-B and the selected-demand issuer are intentionally caller-zero. Production
wiring belongs to a later card at the located source carrier before Loop syntax
is decomposed.
