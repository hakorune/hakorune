---
Status: Accepted design stop — source-capability handoff is required before bridge wiring
Date: 2026-08-03
Decision: direct `route_loop` bridge is `NoSafe`; next row is the resolved-source capability handoff
Scope: one disjoint DirectAccum singleton bridge through the existing compile candidate
Related:
  - joinir-loop-scoped-nongeneric-cutover-ssot.md
  - joinir-loop-selfhost-recipe-pipeline-ssot.md
  - joinir-loop-accum-physicalizer-candidate0-m10a-d1-design-stop-2026-08-03.md
  - joinir-loop-pre-effect-product-ssot.md
  - ../design/phi-lifecycle-ssot.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
---

# DirectAccum singleton production bridge: M10a/N2 design stop

## Why this is a design stop

The caller-zero M10a slice is now complete. The real DirectAccum physicalizer
uses the existing `CanonicalCfgSessionV1`, function-owned
`BindingSsaBuilderV1`, and `PhiTxn`; candidate abort/fresh reuse and shared
semantic-core parity are green. No production caller exists yet.

N2 is the first behavior-changing wiring boundary. It must prove that the
portable path can own one genuinely disjoint singleton source without turning
the existing route scheduler into a second authority or falling back after a
Builder effect. This card therefore freezes the production boundary before any
router/handler edit.

The permitted scope is exactly:

```text
one raw schedule: [AccumConstLoop]
one policy winner
one verified DirectAccum Recipe/JoinSig pair
one candidate physicalizer
one external candidate commit later in the normal compile transaction
```

Generic, overlapping schedules, all-route Retry removal, and final M10b remain
outside this row.

## Consultation result and decision

The worker audit closed the design question without authorizing router wiring:

1. Exact source topology belongs to `FunctionSourceViewV1` plus
   `LocatedStmtV1`/`LocatedExprV1` child roles. A production bridge must not
   rebuild paths, names, or AST identity at `route_loop`.
2. Binding/source identity belongs to `VerifiedResolvedFunctionV1` and the
   separately sealed `VerifiedResolvedLoopSourceV1`. `CanonicalLoopFacts` and
   `AccumConstLoopFacts` remain observation/parity products, not source
   authority.
3. The current `route_loop` production entry receives only
   `LoopRouteContext` and `&mut MirBuilder`. It cannot issue a matching
   `LoopExecutionFrameKeyV1`, and the resolved lowerer does not currently
   consume this route entry. Directly wiring the physicalizer there would
   create an unsound second source/frame authority.
4. The physicalizer still has exactly one permitted mutation owner: an
   unpublished `ModuleBuilderInvocationSessionV1` candidate using the existing
   `CanonicalCfgSessionV1`, `BindingSsaBuilderV1`, and `PhiTxn`. No live-builder
   caller or route fallback is permitted.
5. `LoopResultDispositionV1::Unit` must be mapped by the existing function
   completion/value-carrier contract. It must not become `None`, a fabricated
   `ValueId`, or a scheduler retry.
6. The old Accum composer/PHI edge cannot be deleted from the bounded fixture
   alone. A full production raw-schedule census must prove that the portable
   singleton branch is disjoint before same-commit retirement.

Decision: do not edit `route_loop`, handlers, scheduler, or PHI writers for
N2. The next executable row is a small, non-Builder **resolved-source
capability handoff** that carries one source-issued `LoopExecutionFrameKeyV1`
and exact located loop identity to the future singleton bridge. Until that
handoff exists, the correct outcome is typed `NoSafe`, not AST/name lookup.

The production issuer candidate is now fixed more precisely: the
`CanonicalFunctionLowererV1`/`CanonicalTrivialSsaLowererV1` located-source
boundary, never `route_loop`. The latter already owns the function's
`CanonicalCfgSessionV1`, `BindingSsaBuilderV1`, and `PhiTxn`. The physicalizer
core must borrow those owners; only the caller-zero test wrapper may create
local owners and commit/abort them. Promoting the current standalone
physicalizer unchanged would create a second SSA/CFG authority and is
`NoSafe`.

The audit then found one further unresolved handoff: the portable
Recipe/JoinSig intentionally carries no source sites, while the canonical
identity ledger must still claim the DirectAccum update/step assignments
before the physicalizer emits their writes. The production bridge therefore
needs a separate, execution-scoped **binding-effect witness** (exact
`BindingRefV1` roles plus sealed source assignment sites) alongside the
portable physical input. It must be issued from the same resolved frame,
consumed once by the canonical lowerer, and must not become a second Recipe,
SSA, or PHI authority. Until this witness and its claim/update contract are
specified, external-SSA injection remains `NoSafe`.

## Current production observations

The current `route_loop` path is still:

```text
try_build_outcome
  -> LivePreflightFrameV1
  -> observe_all_route_preflight_v1
  -> legacy continuation / RouteExecutionWitnessV1
  -> route_accum_const_loop
  -> RecipeComposer -> CorePlan -> PlanVerifier -> PlanLowerer
```

The scoped census already identifies the direct Accum fixture as a singleton
`[AccumConstLoop]`, while simple-while and nested-loop fixtures retain overlap
or Generic suffixes. That census is test evidence only until a production
caller consumes the same frame-owned schedule.

The portable producer currently consumes
`VerifiedSelectedLoopRecipeDemandV1`, which requires one matching execution
frame, one resolved source identity, and one structural-facts payload. The
existing AST-to-structural projector consumes
`ResolvedFunctionLoweringInputV1` plus `LocatedStmtV1`; `route_loop` currently
receives only `LoopRouteContext` and a mutable `MirBuilder`. A bridge must not
re-resolve the AST, reconstruct names, or manufacture a frame key at the
router. If the resolved source capability is not available at the caller, the
correct result is a typed design rejection and a caller-side capability seam,
not a new AST matcher.

## Design questions that must be closed

### 1. Exact production issuer and frame identity

Choose the one owner that issues all of the following from the same execution
frame:

```text
raw schedule == [AccumConstLoop]
VerifiedLoopPolicyWinnerV1
VerifiedLoopStructuralFactsV1::DirectAccum
VerifiedResolvedLoopSourceV1
VerifiedSelectedLoopRecipeDemandV1
```

The issuer must prove singleton/disjointness before the first Builder effect.
`diagnostic_effective`, `LoopRouteContext::route_kind`, and route-name checks
are not authority. The bridge must not add a second selector beside
`RecipeFirstRouteSelectionV1`.

### 2. Candidate-scope caller census

Prove that the one production physicalizer caller is inside the existing
`ModuleBuilderInvocationSessionV1` candidate and that no live external
`MirBuilder` caller reaches it. The physicalizer may mutate that candidate but
never commits it. A failed physicalization drops the whole compile candidate;
it never calls `route_accum_const_loop`, `RouteExecutionWitnessV1`, or Generic.

Required static result:

```text
physicalize_direct_accum_v1 production callers = exactly 1
portable singleton branch -> legacy handlers = 0
portable singleton branch -> Retry/Option/fallback = 0
live Builder direct callers = 0
```

### 3. Production binding/input projection owner

The caller-zero physicalizer requires an owner-issued
`VerifiedLoopBindingProjectionV1` and an input projection that points to
already-defined current-function values. Production must not derive these from
names or invent `Const(0)` values. The bridge must identify the existing
resolved BindingRef/ValueId owner at the current function boundary and issue
the projection once. If this requires seeding a new SSA owner, stop: the
existing function-owned Binding SSA is the only authority.

### 4. Unit completion contract

The physicalizer currently returns an explicit `LoopResultDispositionV1::Unit`
receipt, while `route_loop` exposes `Result<Option<ValueId>, String>` to its
caller. N2 must select the existing function-completion/value-carrier
contract for a unit loop. It may not fabricate a `ValueId`, silently map Unit
to `None`, or widen the physicalizer into a scheduler. The selected mapping
must preserve legacy completion semantics and be covered by the parity gate.

### 5. Selected old-edge retirement

Determine from the full raw-schedule census whether `AccumConstLoop` has any
production overlap beyond the bounded singleton fixture. If none exists, the
same N2 commit may remove the selected `route_accum_const_loop`/legacy PHI edge.
If an overlap remains, keep its old edge behind an explicitly named legacy
owner and prove that the singleton portable branch can never call it. Do not
delete a shared route entry merely because the direct fixture is singleton.

## Required worker consultation output (closed)

The consultation returned:

1. the exact current production caller (`route_loop`) and the missing
   resolved-source capability path;
2. the singleton/disjointness evidence and known overlapping Accum schedules;
3. the candidate-scope and external-commit boundary, including the Unit result
   mapping;
4. the precise old Accum/PHI retirement precondition (full production census);
5. the smallest next guard/test additions, with every touched Rust file below 800
   lines;
6. explicit non-claims for Generic/D2, M7-M9, all-route scheduler deletion,
   selfhost authority, and final M10b.

The missing capability path is now the next implementation boundary; no
production caller or old-edge retirement is claimed by this card.

## Acceptance gates for N2 design

- no code changes to `route_loop`, handlers, scheduler, or PHI writers for this
  design stop;
- current M10a caller-zero gates remain green;
- the design names exactly one future source/policy/demand issuer and one
  future physicalizer caller, while recording that the current route entry is
  not a valid issuer;
- unresolved resolved-source, candidate-scope, Unit-completion, or old-edge
  questions remain typed `NoSafe` rather than being bridged by AST/name lookup;
- the resulting implementation slice is one scoped singleton bridge, not a
  partial all-route cutover.

## Explicit non-claims

This card does not authorize Generic V0/V1 classification, Retry deletion,
global PHI/materializer retirement, M7-M9 route coverage, `.hako` selfhost
physicalization, or M10b atomic scheduler cutover. PHI/SSA lifecycle and
reaching-definition authority are already SSOT'd; this stop concerns only the
production consumer wiring and legacy-edge retirement.
