---
Status: Accepted design — canonical SSA owner/profile handoff is fixed; implementation may proceed
Date: 2026-08-03
Decision: keep one `CanonicalSsaFunctionLowererV2` owner; admit DirectAccum as an explicit whole-function profile, then borrow its existing CFG/Binding-SSA/PhiTxn for the bridge
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

## Why this was a design stop

The caller-zero M10a slice is complete. The real DirectAccum physicalizer
proves candidate abort/fresh reuse and shared semantic-core parity, but it has
no production caller. The missing decision was not a new PHI/SSA design: it
was how the resolved loop profile enters the already SSOT'd function owner.

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

Decision is now closed:

1. `CanonicalSsaFunctionLowererV2` is the only canonical function owner. It
   owns one `ResolvedIdentityLedgerV2`, `CanonicalCfgSessionV1`,
   `BindingSsaBuilderV1`, and `PhiTxn` for the whole function. `If`, Loop, and
   straight-line statements are profiles over this same owner.
2. `DirectAccum` is an explicit whole-function admission/profile variant, not
   a second lowerer state. `CanonicalFunctionLowererV1` remains the
   compatibility A+ facade; `CanonicalTrivialSsaLowererV1` remains the exact
   carrier-free profile. Neither is silently widened with a Loop arm.
3. The profile co-seals one `VerifiedDirectAccumRouteAdmissionV1` from the
   existing frozen policy winner/schedule, one `VerifiedResolvedLoopSourceV1`,
   one `VerifiedSelectedLoopRecipeDemandV1`, a binding-effect witness, and the
   function completion contract before Builder effects. Route names or AST
   rescans are not admission authority.
4. The non-Clone
   `VerifiedLoopBindingEffectWitnessV1` is execution-scoped and carries the
   same frame/owner brand, exact DirectAccum `BindingRefV1` roles, and all
   canonical source claims: condition/update/step variable-use sites plus the
   update/step assignment-target sites. It carries no AST clone, names,
   `ValueId`, `BasicBlockId`, route, Recipe, or PHI data. The identity ledger
   consumes it once before physicalization; it is not a second ledger.
5. The physicalizer core borrows the function owner's existing CFG/SSA/PhiTxn.
   Only the caller-zero adapter may create local owners and finish/abort them.
   A failed claim or physicalization drops the unpublished compile candidate;
   it never becomes `None`, retry, or fallback. `Unit` goes through the
   existing completion contract.

The first implementation is a behavior-neutral Refactor Series: extract the
shared SSA machinery behind thin A+/Trivial/Profile facades, then add the
DirectAccum admission/profile and caller-zero vertical. Every touched Rust
file remains below 800 lines. No `route_loop`, scheduler, handler, or PHI
writer edit is part of the refactor series.

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

## Implementation contract after D2

### 1. Exact issuer and co-sealed inputs

The canonical resolved ingress must issue all of the following from one
function execution frame:

```text
raw schedule == [AccumConstLoop]
VerifiedLoopPolicyWinnerV1
VerifiedLoopStructuralFactsV1::DirectAccum
VerifiedResolvedLoopSourceV1
VerifiedSelectedLoopRecipeDemandV1
```

The profile must prove singleton/disjointness before the first Builder effect.
`diagnostic_effective`, `LoopRouteContext::route_kind`, route names, and AST
rescans are not authority. The bridge must not add a second selector beside
`RecipeFirstRouteSelectionV1`.

### 2. Refactor-series order

The next code slice is behavior-neutral and does not widen acceptance:

```text
shared SSA/CFG/identity/completion core
  -> thin A+ and Trivial facades remain green
  -> explicit DirectAccum whole-function profile
  -> caller-zero physicalizer borrow adapter
  -> one resolved production caller
  -> selected old-edge retirement
```

Do not add a Loop arm to the Trivial analyzer, create a second SSA owner, or
wire `route_loop` during the extraction series.

### 3. Candidate-scope caller census

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

### 4. Production binding/input projection owner

The caller-zero physicalizer requires an owner-issued
`VerifiedLoopBindingProjectionV1` and an input projection that points to
already-defined current-function values. Production must not derive these from
names or invent `Const(0)` values. The bridge must identify the existing
resolved BindingRef/ValueId owner at the current function boundary and issue
the projection once. If this requires seeding a new SSA owner, stop: the
existing function-owned Binding SSA is the only authority.

### 5. Unit completion contract

The physicalizer currently returns an explicit `LoopResultDispositionV1::Unit`
receipt, while `route_loop` exposes `Result<Option<ValueId>, String>` to its
caller. The DirectAccum profile must select the existing function-completion/
value-carrier contract for a unit loop. It may not fabricate a `ValueId`, silently map Unit
to `None`, or widen the physicalizer into a scheduler. The selected mapping
must preserve legacy completion semantics and be covered by the parity gate.

### 6. Selected old-edge retirement

Determine from the full raw-schedule census whether `AccumConstLoop` has any
production overlap beyond the bounded singleton fixture. If none exists, the
the same production bridge commit may remove the selected
`route_accum_const_loop`/legacy PHI edge.
If an overlap remains, keep its old edge behind an explicitly named legacy
owner and prove that the singleton portable branch can never call it. Do not
delete a shared route entry merely because the direct fixture is singleton.

## Required consultation output (closed)

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

The design boundary is now closed. The next implementation boundary is the
behavior-neutral shared-SSA-core extraction; no production caller or old-edge
retirement is claimed until its focused gate and the DirectAccum profile gate
are green.

## Acceptance gates for D2 and the next refactor series

- the existing PHI/SSA SSOT remains the only physical owner;
- shared-core extraction preserves the Trivial profile's current acceptance
  and focused tests;
- DirectAccum admission is co-sealed with the policy winner, resolved source,
  recipe/JoinSig, binding-effect witness, and Unit completion contract;
- witness rejection covers wrong frame, foreign owner, wrong variable-use or
  target site, duplicate claim, and unclaimed claim;
- the physicalizer core has no production-local SSA/CFG/PhiTxn owner and has
  no `route_loop` caller;
- production caller switch and old-edge retirement remain a later atomic row;
- Generic, overlap schedules, all-route Retry deletion, M7-M9, selfhost
  authority, and M10b remain explicit non-claims.

## Explicit non-claims

This card does not authorize Generic V0/V1 classification, Retry deletion,
global PHI/materializer retirement, M7-M9 route coverage, `.hako` selfhost
physicalization, or M10b atomic scheduler cutover. PHI/SSA lifecycle and
reaching-definition authority are already SSOT'd; this stop concerns only the
production consumer wiring and legacy-edge retirement.
