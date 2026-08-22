# Loop Structural Facts

## M8 S6A variable-accum recurrence boundary

The `VariableAccumRecurrenceV1` observer is the sole source-family
owner for `acc = acc + i; i = i + 1`. Its compiler projection may use private
partial observations, but exactly one AST-free move-only aggregate,
`VerifiedVariableAccumRecurrenceFactsV1`, crosses this module boundary.

The aggregate retains resolver-issued owner/origin/source-kind/site/frame and
scope identity, two distinct local `BindingRef` roles, two initializer inputs,
the condition/update/step roles, and complete source coverage. It does not
mint Recipe/JoinSig keys, route IDs, ValueId/BasicBlockId, or physical facts.

The source-role map is sealed with these exact cardinalities:

```text
input-source relations          = 2
Recipe binding relations        = 2
Core binding-effect relations   = 8
item-source operation relations = 11
```

Private DTOs are not independent verified products. The producer consumes a
Candidate once and projects only into the existing neutral Recipe/Core/input/
operation owners. `NoSafeSlice` is a development status, not a source
disposition. Source outcomes are `Candidate`, `Declined`, `Unresolved`, or
`Rejected`, with identity conflict first, missing evidence second, and fully
observed shape mismatch third. `print(acc)` and `return 0` remain callable-tail
evidence outside this Facts boundary.

The resolver-backed projection and atomic Facts issuer are landed in
`mir/compiler/variable_accum_recurrence_projection.rs` and
`variable_accum_recurrence.rs`; source-site coherence checks live in the
separate `variable_accum_recurrence_validation.rs` module so this Facts owner
remains below 800 lines. The producer-facing Recipe mapping is tested through
the existing source-bound owners, and the normal `Main.main` resolver ingress,
full C/D/U/R envelope, duplicate-role rejection, and source-site negatives are
closed. Bounds and deltas are general `I64` literals with exact source anchors;
the fixture values `4` and `1` are not hard-coded policy.

## M8 S6B variable-accum break/fallthrough boundary

`VariableAccumBreakV1` is the single source-family observer for the natural
`loop_break_plan_subset_min.hako` shape: `i < 10`, an `if i == 5` arm with
`sum = sum + 10; break`, implicit normal fallthrough, `sum = sum + 1`, and
`i = i + 1`. Private condition/branch/update observations are co-sealed into
one AST-free `VerifiedVariableAccumBreakFactsV1`; no partial observation is a
second authority. Facts retain resolver-issued source sites, frame, two
distinct bindings, two initializer/input relations, and complete body-role
coverage, but never mint Recipe/JoinSig/physical keys.

The bounded source receipt is intentionally split between logical control and
operation evidence. The normalized existing Recipe has 2 bindings, 2 inputs,
3 blocks, 20 item rows (18 operation rows plus one `If` and one `Break`), 17
values, two carriers, and one break exit. The common source relation counts
are 2 input rows, 2 binding rows, 10 Core binding-effect rows, and 18
item-source operation rows. `If` and `Break` anchors remain in a small
control-source receipt rather than being misclassified as operations.

Source disposition is typed and ordered: identity/owner conflicts are
`Rejected`, unavailable resolver or coverage evidence is `Unresolved`, and a
fully observed non-matching operator/body/else shape is `Declined`. Exact
shape alone yields `Candidate`. `NoSafeSlice` remains a development status,
not a fifth source outcome. The producer and JoinSig tests are caller-zero;
there is no Builder/MIR/CFG/PHI, selector, retry/fallback, production route,
callable tail, or legacy-deletion authority in S6B.

## M8 S6C ScanWithInit frontier

The typed-input relation, exact length/substring source-bound call relation,
and non-Clone Exit/Tail source co-seal are landed. The co-seal consumes the
existing Completion, requires exactly two explicit value returns, relates the
TextEq-owned If-then `return index` to the selected Loop, and relates the root
body tail only through the resolver's exact `Minus(Integer(1))` source rows.
It lends one HRTB view and owns no Recipe key or physical identity.

`VerifiedS6CScanWithInitFactsV1` now consumes the co-sealed source products and
closes the exact statement/expression/effect/relation surface from the same
resolver body-shape seal. Its HRTB view carries no AST, source-order guess,
Recipe key, MIR ID, physical ID, fallback, or selector. Compound assignment
and extra source statements are rejected before Facts publication. The next
boundary is the sole Recipe producer; `SplitScan`, `CharMap`, `ArrayJoin`, and
`BoolPredicateScan` remain separate source-family rows.

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

The Generic G0 S0B/S0C cumulative products
(`VerifiedGenericSourceBundleG0` and `VerifiedGenericTypedSourceBundleG0`)
are owned here as move-only capabilities. Compiler projection modules issue
them, but policy and later source observers consume the neutral products
without importing compiler aggregate types. This behavior-neutral owner move
does not open Generic selection, Recipe, Builder/MIR, or production callers.

The source-to-policy boundary is the move-only
`VerifiedGenericG0PolicyHandoffV1`. The compiler-side projector co-seals a
private resolver/source brand borrowed from the canonical selector window, the
typed S0C bundle, exact role/return `BindingRef` relations, target, and
post-loop completion relation. This module owns the neutral handoff product;
policy consumes it by value and retains it in the candidate observation. The
handoff does not retain a second window lease. The older candidate-envelope
witness remains a cfg(test)-only source lease witness and is not a second
production authority. The handoff implementation is caller-zero and does not
open demand, Recipe, Builder/MIR, retry, fallback, or legacy deletion.

The S3 demand witness is now a production-capable caller-zero seam in
`loop_recipe_contract::generic_g0_demand`. It consumes the selected Generic
product once, keeps the canonical lease plus the borrowed brand, typed source
bundle, post-loop tail read, and an opaque role proof, and does not copy or
reconstruct role/BindingRef rows. The compiler-side
`generic_g0_source_parent` issuer co-seals that product with the exact
resolver input and two source-parameter entry rows through one callback-scoped
non-Clone parent. Physical demand, Builder/MIR, retry, fallback, and
production selection remain closed.

`generic_g0_observation.rs` owns the neutral S1 source-attempt transport. It
stores only the existing typed candidate or C/D/U/R source outcome together
with owner/origin/source-kind/site/frame identity, mode, and coverage. It does
not inspect AST or issue policy, Recipe, Builder, MIR, retry, fallback, or
production decisions. The compiler adapter is test-only; ambiguous
`ForestShape`/`BindingLookup` evidence stays unresolved until a separate
resolver-side distinction is designed.

Authority flows in one direction:

```text
VerifiedResolvedFunctionV1::resolved_loop_source(site)
  -> VerifiedResolvedLoopSourceV1
  -> bind_resolved_loop_root_v1(token)
  -> VerifiedLoopRootSourceV1
  -> into_root_claim(&VerifiedLoopRecipeV1)
  -> LoopRecipeSourceBindingV1

VerifiedLoopRootSourceV1
  -> into_root_claim_v2(&VerifiedLoopRecipeV2)
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
the local authority and reads the key only from `VerifiedLoopRecipeV1` or
`VerifiedLoopRecipeV2`; an arbitrary `LoopNodeKeyV1` cannot be injected. The adapter must not scan AST,
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

LoopTrue S1 now follows the same boundary. Its AST-free projection carries the
complete owner/origin/kind/site/frame identity, while the neutral source
attempt/identity/mode/coverage DTO remains compiler-free. Nine policy tests and
eight projection tests are green; selection, Recipe, Builder/MIR, retry,
fallback, and production callers remain closed.

LoopCond S1 now follows the same caller-zero boundary for one bounded
non-true-loop/explicit-else Break/Continue shape. Its AST-free projection
retains only resolver-owned sites, typed direct-exit origin/target evidence,
and owner/origin/kind/site/frame identity; it does not retain BindingRef,
Recipe binding, condition-type/effect, carrier/update, return, nested-loop, or
physical claims. The neutral transport is compiler-free, with nine policy tests
and five projection tests green. The legacy LoopCond facts/Recipe variants
remain migration-only and selection is still closed.

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

D4-S3-D0 is historical schema evidence; the canonical common product is now
`VerifiedLoopFamilyAdmissionWindowV1`, owned by the route-policy admission
assembler rather than this facts layer. The resolver-owned prerequisite
`VerifiedLoopFamilyWindowLeaseV1` is now landed as caller-zero S0. The lease is
identity-only: it carries the resolver brand and source origin/kind/site/frame,
but no mode, coverage, or row data. The route-policy assembler later co-seals
those row-derived facts into `VerifiedLoopFamilyAdmissionWindowV1`, whose rows
are `Candidate|Declined|Unresolved|Rejected`. Legacy `Blocked` and the old
`VerifiedLoopFamilyObservationSetV1` witness name stay historical and must not
become a second authority. The canonical product must not contain
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

## LoopTrue S1 design boundary

The next neutral source-attempt DTO is specified by
`LOOP-FAMILY-LOOPTRUE-OBSERVATION-S1`. It will carry only the resolver/source
projection, owner/origin/kind/site/frame identity, exact mode snapshot, and
coverage seal. Known non-LoopTrue syntax is `Declined`; incomplete or missing
resolver facts are `Unresolved`; foreign, binding, and exit conflicts are
`Rejected`. This layer must not import the legacy schedule policy, issue a
Recipe/JoinSig/BindingKey, or call Builder/MIR. The finite task and required
post-implementation reference update are recorded in
`docs/development/current/main/investigations/loop-family-looptrue-observation-s1-design-task-2026-08-06.md`.

## Family row context-retention R0

The five route-policy observer variants now retain their expected and observed
identity/mode/coverage evidence on every `Candidate`, `Declined`,
`Unresolved`, and `Rejected` row. This is a neutral transport-preservation
boundary only; it does not issue a common admission window, selector, Recipe,
Builder/MIR product, retry, fallback, or production caller. The resolver
window lease S0 is landed; the route-policy common admission assembler is the
next owner. The assembler S1 is now landed in `loop_route_policy` and consumes
the lease plus five row envelopes; this facts layer remains identity/source
evidence only and does not own selector, Recipe, Builder, or MIR products.
