# JOINIR Loop Recursive Recipe Producer M7-D0

Status: Design boundary fixed; production wiring intentionally out of scope.

Task: `JOINIR-LOOP-RECURSIVE-RECIPE-PRODUCER-FACADE0-D0`

## Decision

The portable `LoopRecipeArtifactV1` is the future semantic contract, while the
existing `RecipeTree + Parts + Composer/CorePlan/PlanLowerer` remains the
production authority and parity oracle until M10b. M7 introduces only a
caller-zero migration facade and bounded producer adapters. It does not add a
new scheduler, lowerer, PHI writer, SSA owner, or publication boundary.

PHI/SSA authority is already fixed elsewhere:

- `CanonicalCfgSessionV1` owns physical CFG, terminators, and predecessor truth.
- one function-owned `BindingSsaBuilderV1` owns reaching bindings and PHI
  requirements.
- one caller-owned `PhiTxn` owns provisional PHI lifecycle and rollback.
- `LoopPhiMaterializerV1` remains caller-zero mechanical evidence only.

M7 must consume these owners later; it must not recreate or wrap them as a
family-specific implementation. Logical `JoinSig` obligations are not physical
PHI/SSA and may be extended only in the shared JoinSig owner.

## Facade contract

The first implementation slice is a single, non-`Clone` test-only facade:

```text
sealed policy winner + owned structural input + migration receipt
  -> VerifiedLoopRecipeProducerFacadeV1
  -> Result<VerifiedLoopRecipeProductV1, LoopRecipeProducerRejectV1>
  -> owned LoopRecipeV1
  -> LoopRecipeVerifierV1::verify
  -> LoopJoinSigElaboratorV1
```

The facade consumes one already-selected demand. It never enumerates routes,
selects a family, retries a suffix, reads a Builder, imports CorePlan or
PlanLowerer, allocates physical IDs, or publishes a candidate. Route/family is
diagnostic provenance only and is not visible in the portable artifact,
semantic normalization, or JoinSig.

`RecipeTree + Parts` may be read by a separate test-only legacy oracle. The
oracle projects accepted legacy output to a normalized semantic digest; it is
not allowed to become a downstream authority of the portable producer.

M7's producer product is the semantic recipe. A source-bound
`LoopRecipeArtifactV1` is only a fixture/contract-layer claim until the source
owner can issue one sealed root+child witness for every nested loop. The
producer must not promote the current root-only source binding, use raw index
paths, or hand-build child paths from AST. Source-bound artifact production is
therefore a separately gated source-forest task (or the later located handoff),
not an implicit M7 green.

## M7-D0 representative matrix

| Cohort | Initial disposition | Recipe/JoinSig scope | Required stop |
| --- | --- | --- | --- |
| Direct Accum (`AccumConstLoop`) | accept | existing predicate, i64 operations, carriers, self exits | reject missing carrier/value before effects |
| Nested (`NestedLoopMinimal`) | accept only `Always` child | inherited carrier visibility, child break, post-child update | reject nested predicate; never infer inner semantics from outer provenance |
| LoopTrue (`LoopTrueBreakContinue`) | deferred until shared branch-exit closure | `Always` root, conditional `Break`/`Continue` | do not collapse conditional exit to a direct `Exit` |
| LoopCond (`LoopCondBreakContinue`) | deferred until shared branch/merge closure | predicate root, conditional exits and fallthrough merge | reject `BranchMergeMismatch`/unsupported closure as typed terminal |
| Generic V0/V1 | parked behind M4 | bounded Generic V1 only after debt classification and winner proof | no mock, omission, or post-effect `None` conversion |

The matrix is a census and contract, not a second route registry. A source
operation outside the current portable vocabulary is a typed producer reject
and a new bounded vocabulary design stop; it is never made opaque merely to
make the row green.

## Ordered work

### M7-D0 — facade and closure boundary (this card)

Write the facade contract, representative matrix, producer caller-zero guard,
and exact reject reasons. Confirm the existing Accum direct/nested-`Always`
fixtures are the only accepted logical golden at this boundary.

### M7-S0 — Accum producer cohort

Expose the existing M5/M6 Accum semantic construction through the common
facade; do not create an `AccumProducerV2` or a second normalized recipe
builder. Add a Nested-`Always` semantic golden with explicit child/ancestor
carrier rows using the same neutral key/arena helpers.
Verify deterministic artifact roundtrip, verifier output, JoinSig output, and
normalized parity to the legacy oracle. Keep physical candidate/PHI evidence in
the existing P1/M5 harness; do not call `LoopPhiMaterializerV1`.

Before claiming Nested source-bound parity, close the source-authority gap:
`bind_resolved_loop_root_v1` currently issues a root-only source witness, while
the portable artifact requires a one-to-one binding for every nested loop. The
Nested producer must not hand-build child paths from AST or raw indices. If the
current resolver cannot issue an owned non-`Clone`
`VerifiedResolvedLoopSourceForestV1` (root plus exact child sites), Nested stays
logical caller-zero only and opens a bounded source-owner design stop.

### M7-S1 — negative closure fixtures

Pin typed rejection for nested predicates, unavailable values, missing carriers,
unreachable items, unsupported exits, and branch merge mismatch. These fixtures
prove the boundary without inventing an adapter-local workaround.

### M7-S2 — shared logical branch/merge closure (design stop first)

Extend only the JoinSig logical owner if needed so an `If` can carry deterministic
fallthrough and one or more exit paths. A branch with one fallthrough arm and
one `Break`/`Continue` arm must preserve both obligations; two fallthrough arms
must emit a binding-merge obligation when incoming values differ. This is logical
dataflow, not PHI emission. No Recipe schema change is implied.

### M7-S3 — LoopTrue and LoopCond cohorts

After S2 is green, add one representative adapter per family. Both use the same
artifact/verifier/JoinSig product path; no family-specific downstream switch or
PHI materializer is permitted.

### M7-S4 — Generic gate

This slice is explicitly parked until M4 closes Generic V0/V1 debt:
pre-effect disposition, V0/V1 precedence, legacy-winner equivalence, and zero
post-effect retry on the new path. If any remains unresolved, Generic stays a
typed `UnresolvedStop` and the other cohorts continue independently.

### M7-S5 — joint close

All five representatives produce the same recursive semantic envelope, with
normalized Recipe/JoinSig parity and caller-zero guards. Production
`route_loop`, scheduler retirement, physicalizer activation, and PHI/SSA
cutover remain M10b work.

## Gates and forbidden shortcuts

1. Exactly one producer facade definition; production callers remain zero.
2. The portable contract/facade imports no AST, Builder, CorePlan, PlanLowerer,
   RecipeTree, physical ID, PHI mutation, or Retry machinery. A migration
   adapter may read an existing analysis-only source/facts view once and
   convert it to owned typed recipe input; it may not carry AST/StmtRef into
   the portable product or reconstruct source paths.
3. No `Option`/Retry/suffix continuation, route matching, or family selection
   occurs downstream of the sealed demand.
4. Accepted semantic recipes pass verifier and JoinSig deterministically;
   source-bound artifacts are claimed only with an exact source-forest witness;
   rejected rows
   fail before any Builder effect with a named typed reason.
5. Legacy comparison is semantic normalized parity only. Raw MIR/ValueId/
   BasicBlockId equality is not a claim while legacy auxiliary artifacts remain.
6. `LoopPhiMaterializerV1` remains mechanical caller-zero evidence; the sole
   physical PHI/SSA owners remain the existing canonical CFG/Binding-SSA/PhiTxn
   services.
7. Every touched Rust file stays below 800 lines and each cohort is a focused
   fixture/gate slice.

## Dependencies and stop conditions

Hard dependencies are M2 schema/verifier, M3 frozen policy/typed outcomes, M5
Accum contract, M6-A logical JoinSig, the nested source-forest issuer, and the
existing canonical CFG/Binding-SSA/PhiTxn owner boundary. M6-B is evidence only.
Generic additionally depends on M4; it may not be silently treated as an
accepted representative.

If the shared JoinSig closure cannot express conditional exits or binding merges
without a new semantic vocabulary, stop at S2 and open a bounded design card.
Do not add route-local PHI repair, opaque operations, a second scheduler, or a
detached Builder candidate.
