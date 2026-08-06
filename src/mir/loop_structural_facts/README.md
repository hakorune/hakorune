# Loop Structural Facts

## Generic G0 S0A

`generic_g0/` is the sole issuer of `VerifiedGenericStructuralFactsG0`.
The compiler-side `generic_g0_projection` module is the only AST-bearing
projector for this row; it navigates a natural `ResolvedFunctionLoweringInputV1`
and passes an AST-free observation here. S0A seals only exact nested-loop
shape/order, resolver-issued `BindingRefV1` relations, owner/source/frame
identity, and duplicate-free source coverage. It does not own numeric/type
policy, candidate selection, Recipe keys, Builder/MIR effects, retry, fallback,
or production routing. The product is move-only and remains caller-zero until
the later Generic rows explicitly open their handoff.

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

The NestedPredicate S1 source DTO and neutral observation transport now live in
this module. They retain resolver-owned forest/shape provenance only; they do
not issue Recipe keys, select a family, or enter Builder/MIR. The compiler
adapter is test-only and the policy observer owns the disposition matrix.

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

The D3-S2-P3 row is closed as a cfg(test)-only independent-column census:
`JOINIR-GENERIC-RESOLVED-CARRIER-FAMILY-OVERLAP-CENSUS0-D3-S2-P3`. Its raw
Generic rows retain mode, V0/V1 presence, carrier observation, and raw
schedule; its resolved rows retain only NestedPredicate, DirectAccum, A+, or
an explicit canonical rejection. The columns have no source/owner/frame
bridge. A non-empty pair is reported only as
`UnresolvedStop(FamilyOverlap)`; no winner, selector, Recipe, BindingKey,
Builder, MIR, or production caller is authorized.

M3-B and the selected-demand issuer are intentionally caller-zero. Production
wiring belongs to a later card at the located source carrier before Loop syntax
is decomposed.

## D4 transport boundary

D4-WITNESS0 is closed outside this module as a private `#[cfg(test)]` source
window. It lends paired raw/resolved views from one non-`Clone`
resolver-owned receipt and proves only owner/site/frame/forest identity plus
typed pre-effect rejects. It does not modify `LoopFacts`, issue a Generic
snapshot or `LoopBindingKeyV1`, select a family, or call Builder/MIR.

D4-S1 route design is accepted, but this module remains caller-zero. The
existing resolved preflight seam and Recipe/physical ownership are still
protected by the D4-S2 family-boundary design stop.

D4-S1-S0 is closed as cfg(test)-only evidence outside this module. The witness
reuses the D4 resolver-owned source receipt and the existing DirectAccum probe;
it does not widen `LoopFacts`, issue a selector input, or add a Recipe/key
consumer. The accepted D4-S2 owner map is now followed by the D4-S3 docs-only
selection-authority stop; this neutral layer remains caller-zero until that
stop defines the observation set before any canonical family product arrives.

D4-S2-S0 is closed as a sibling private `#[cfg(test)]` six-row legacy census.
It lends raw and resolved observations from one resolver-owned receipt per
fixture/mode row, but does not widen this neutral layer or publish a reusable
Generic observation product. The measured raw carrier/schedule and existing
preflight family remain `legacy_*` retirement inventory; no selector, Recipe,
key, Builder/MIR caller, retry, or fallback is implied. D4-S3-D0 is closed and
the private D4-S3-S0 witness now owns the observation-set transport; this layer
must wait for D4-S3-S1 and a later design before gaining a canonical family
consumer.

D4-S3-D0 now closes that schema decision without activating this layer. The
future `VerifiedLoopFamilyObservationSetV1` is one resolver-branded,
non-`Clone` source receipt/window plus exact mode and coverage seals and
family-tagged `Candidate|Declined|Blocked|Unresolved` rows. It must not contain
route IDs, raw cursors, AST, Recipe/key, Builder/MIR/ValueId/PHI, or retry and
fallback state. D4-S3-S0 is now a closed private witness; only a later
family-level selector may consume the sealed set, while A+/Trivial remain in
the separate whole-unit stage.

D4-S3-S0 is closed without widening this facts boundary. Its six private
sets retain only a resolver receipt, exact mode snapshot, loop-window coverage
seal, and unresolved family-tag rows. D4-S3-S1 is also closed outside this
facts layer: nine private source-backed fixture/mode sets record V0Only,
V1Only, Both, and Neither cells while preserving NoStandaloneRow and planner
freeze as separate typed evidence. No schedule policy, selector, or Recipe
input is issued here. D4-S3-S2 is now closed as a separate test-only neutral
selector consumer; it does not widen this facts boundary. D4-S4-D0 records
that current Generic facts and P2 labels cannot become Recipe demand; only a
resolver-issued AST-free candidate proof plus one-shot source/BindingRef lease
may cross the future handoff. D4-S4-S0 is closed as `NoSafeSlice`.
D4-S4-S0-D0 fixes the future move-only source lease, AST-free shape/candidate,
policy observation, and Generic demand chain. GENERIC-SEMANTIC-SHAPE-SCHEMA-D1
is now closed as the typed shape contract. The bounded cfg(test)-only
source-lease witness is closed; this layer remains caller-zero. CarrierProof is
also closed as a separate test-only consumer. Full role extension is a design
stop and may not re-resolve roles or issue a selector input.
