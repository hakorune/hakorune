# JOINIR Loop Selected Recipe Demand Source D0

Status: Design stop; no implementation beyond the caller-zero facade.

Task: `JOINIR-LOOP-SELECTED-RECIPE-DEMAND0-D0`

## Evidence

The M7-S0 facade is green, but it intentionally consumes an already-owned
semantic `LoopRecipeV1`. The current upstream objects cannot yet issue that
input:

- `LoopQualifiedV1` contains only `SourceAvailable` candidate facts and a
  private seal. It does not own structural facts, source identity, or a recipe.
- `FrozenLoopRouteScheduleV1` preserves all 19 raw rows and typed observations;
  it is a policy/provenance snapshot, not a selected source payload.
- `CanonicalLoopFacts` still contains route-local AST expressions and is not a
  portable input. Passing it through the policy or portable contract would
  create a second source/recipe authority.
- `bind_resolved_loop_root_v1` issues one exact root witness. It cannot claim a
  nested source-bound artifact until a root+child source forest is issued by a
  sealed resolved-source owner.

Therefore the missing item is not PHI/SSA and not another route adapter. It is
one post-policy, pre-recipe owner that joins a sealed winner with owned
structural input exactly once.

## Decision boundary

The selected demand must be a non-`Clone`, consuming product with this shape:

```text
one non-Clone VerifiedLoopPolicyWinnerV1
  + one VerifiedLoopStructuralFactsV1
  + one exact resolved-source capability
  + opaque migration receipt for diagnostics
  -> VerifiedSelectedLoopRecipeDemandV1
  -> caller-zero producer facade
```

The winner, structural facts, and source capability each have one owner and
must be paired one-to-one. A missing winner is a typed `NoPolicyWinner`,
`PolicyBlocked`, or `PolicyExhausted` disposition; it never reaches the
producer. A facts/source mismatch is a typed handoff rejection.

The demand owner may carry typed analysis-only input needed by an adapter, but
the portable contract receives only owned recipe data. It must not carry:

```text
ASTNode / StmtRef / CondBlockView
MirBuilder / CorePlan / PlanLowerer
ValueId / BasicBlockId / PHI / Binding SSA state
RouteFn / Retry / suffix / candidate mutation
```

Route and family are opaque diagnostic receipt fields only. The demand owner
must not enumerate candidates or match `LoopRouteId` to select a producer.

## Options and selected direction

### Rejected: put structural facts in `LoopQualifiedV1`

This couples the pure M3 policy owner to AST/recipe input and makes policy
responsible for source ownership. It violates the current route-policy guard
and turns policy into a second producer.

### Rejected: let the facade read `CanonicalLoopFacts`

This bypasses the sealed winner and imports route-local AST/legacy authority
into the portable boundary. It also makes `None`/retry semantics implicit.

### Selected for the next design slice: neutral selected-demand issuer

Add one neutral bridge after policy evaluation. It consumes the non-`Clone`
qualified result and an independently sealed structural projection, checks the
pairing/owner invariant once, and emits `VerifiedSelectedLoopRecipeDemandV1`. The
policy remains data-only; the source owner remains the source authority; the
producer facade remains a one-way verifier/JoinSig terminal.

The exact Rust types and issuer placement are still a design question because
the current policy product has no source payload and the current source adapter
is root-only. Do not implement a guessed `LoopQualifiedV2` or a route-specific
bridge before this boundary is approved.

## Required source projections

### Direct Accum

The minimum accepted projection must include the condition/step/accumulator
operands as owned typed observations, not AST references. A direct root source
witness is sufficient for semantic M7, but source-bound artifact parity remains
unclaimed until the source issuer contract is promoted.

### Nested Always

The semantic recipe may be tested with the existing Nested-Always golden. A
source-bound claim requires a non-`Clone` root+child forest with exact lineage;
the child path may not be hand-built from raw indices or inferred from outer
provenance.

### LoopTrue / LoopCond

These remain blocked by the shared logical JoinSig branch/merge closure. The
demand issuer must preserve conditional exit/fallthrough obligations; it may
not normalize them to a direct exit.

### Generic V0/V1

No demand is issued while M4 Generic debt remains unresolved. `UnresolvedStop`
is a typed disposition, not an omitted row or a mock recipe.

## Gates for the next implementation card

1. One non-`Clone` selected-demand issuer definition and one consuming caller.
   The policy winner, structural facts, and exact source handle are each
   one-to-one and independently owner-checked.
2. Policy remains free of AST, `CanonicalLoopFacts`, `LoopRecipeV1`, and
   physical/PHI imports.
3. Source projection has one owner and a typed root/child lineage witness;
   no raw-index reconstruction.
4. Producer facade receives one demand, performs no selection/retry, and
   returns `Result<VerifiedLoopRecipeProductV1, Reject>`.
5. Direct Accum semantic parity remains green; nested source-bound parity is a
   non-claim until the forest issuer is present.
6. No production `route_loop`, scheduler, physicalizer, `LoopPhiMaterializerV1`,
   `CanonicalCfgSessionV1`, `BindingSsaBuilderV1`, or `PhiTxn` caller is added.
7. The existing logical-demand/shared guard is extended before the first
   implementation commit; touched Rust files remain below 800 lines.

## Stop conditions

Stop and reopen design if the issuer requires any of the following:

- route/family dispatch inside the recipe contract or JoinSig;
- AST reconstruction or opaque legacy statement payloads;
- source paths manufactured from raw indices;
- a second policy evaluator or retry scheduler;
- a new PHI/SSA/materializer owner;
- a source-bound nested claim without a sealed source forest.
