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

VerifiedResolvedFunctionV1::resolved_loop_source_forest(root)
  -> VerifiedResolvedLoopSourceForestV1
  -> bind_resolved_loop_source_forest_v1(forest)
  -> VerifiedLoopSourceForestBindingV1
  -> into_source_binding(&VerifiedLoopRecipeV1)
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
the resolver forest is consumed by the caller-zero D1 adapter and its recipe
correspondence is verified.

The Direct Accum S0 projection keeps the AST-bearing observation in
`mir/compiler/direct_accum_projection.rs`. That adapter navigates only through
`FunctionSourceViewV1` and the shared child-role vocabulary, then issues the
AST-free `DirectAccumStructuralShapeV1` product. `VerifiedResolvedFunctionV1`
resolves its exact expression sites to `BindingRefV1`; names and raw indices
are never re-resolved. A source-issued `LoopExecutionFrameKeyV1` is carried by
the winner, facts, and source capability so selected demand rejects a foreign
execution frame before sealing.

The DirectAccum pilot separates shape from route exclusivity:

```text
DirectAccumStructuralShapeV1
  -> VerifiedDirectAccumDisjointnessV1
  -> VerifiedDirectAccumSingletonObservationV1
```

The disjointness proof is issued only after the resolved projector has checked
the exact two-assignment, no-control-flow grammar. It contains no route ID or
raw cursor. Shape access alone cannot mint the proof, and policy must not
fabricate the other-route decline rows without consuming this observation.
The legacy `CanonicalLoopFacts` schedule remains a parity oracle only.

This S0 remains caller-zero: it does not construct a Recipe, invoke the
legacy scheduler, mutate a Builder, or become a PHI/SSA writer. Production
physical ownership remains `CanonicalCfgSessionV1` + function-owned
`BindingSsaBuilderV1` + `PhiTxn`.

The selected D3-S2-S0 child is a cfg(test)-only resolver observation task:
`JOINIR-GENERIC-RESOLVED-CARRIER-TYPED-PROVENANCE-OBSERVATION0-D3-S2-S0`.
It may observe forest/frame and exact `BindingRefV1` role/ancestry relations
through the compiler-side projector, but it must not add a Generic snapshot,
logical-key issuer, seed/opaque input, selector, or Builder caller. The
existing DirectAccum `selected_demand` and its hard-coded recipe keys remain
separate owners; Generic key assignment requires a later design.

That child is closed with four focused tests and no production import/caller.
The witness is private to the registry test module; this neutral module still
does not own a Generic snapshot, logical-key issuer, seed, or selection input.
Its coordinate-only forest/frame evidence is not an owner-branded cross-session
capability; a dedicated brand audit must close that premise before this layer
can consume a shared Generic provenance product.

The D3-S2-P2 Generic snapshot is a separate `cfg(test)` sibling module,
`generic_resolved_carrier_facts_snapshot.rs`. It consumes one sealed P1
`VerifiedResolvedCarrierProvenanceV1` and adds only the mode-neutral
`NestedWriteWithPostLoopRead` disposition. It intentionally does not modify
`LoopFacts`, `LoopStructuralFactsPayloadV1`, Generic V0/V1 facts, or any
production caller. P1 typed rejects remain the sole source/owner/frame gate;
the snapshot does not re-validate or re-issue them.

M3-B and the selected-demand issuer are intentionally caller-zero. Production
wiring belongs to a later card at the located source carrier before Loop syntax
is decomposed.
