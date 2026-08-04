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
   `VerifiedDirectAccumBindingEffectPlanV1` is a builder-free source-effect
   plan and carries the same frame/owner brand, exact DirectAccum `BindingRefV1`
   roles, and all canonical source claims: condition/update/step variable-use
   sites plus the update/step assignment-target sites. It carries no AST clone,
   names, `ValueId`, `BasicBlockId`, route, Recipe, or PHI data. A resolved
   identity adapter will consume it once before physicalization; it is not a
   second ledger.
5. The physicalizer core borrows the function owner's existing CFG/SSA/PhiTxn.
   Only the caller-zero adapter may create local owners and finish/abort them.
   A failed claim or physicalization drops the unpublished compile candidate;
   it never becomes `None`, retry, or fallback. `Unit` goes through the
   existing completion contract.

The behavior-neutral Refactor Series extracted the shared SSA machinery behind
the canonical profile boundary in commit `240d76d1bf`; the Trivial facade and
its 116 focused tests remain green. The resolved DirectAccum admission/profile
landed in `fc12f21834`, and commit `3e315a4d4e` now makes the physicalizer core
borrow the caller-owned CFG/SSA/PhiTxn services. Only the caller-zero wrapper
creates local owners and performs finish/commit; the borrowing seam leaves
those decisions to its caller. Every touched Rust file remains below 800
lines. No `route_loop`, scheduler, handler, or PHI writer edit is part of the
refactor series or profile preflight.

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

The design boundary is closed, the shared-core extraction is landed, and the
resolved DirectAccum profile is caller-zero in commit `fc12f21834`. The
physicalizer borrow adapter is also caller-zero green in `3e315a4d4e`, with
abort/fresh-reuse evidence and the owner finish/commit boundary explicit. The
next implementation boundary is the resolved production caller capability and
Unit-completion contract; no production caller or old-edge retirement is
claimed until that boundary and the full old-edge census are closed.

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

## S3 — `JOINIR-LOOP-ACCUM-PRODUCTION-CALLER-CONTRACT-AUDIT-M10A-D2-S3`

Change:
: Audit the exact resolved ingress before adding a caller. The audit must name
  the typed plan variant that admits one DirectAccum profile, the one-way
  source/frame-to-profile handoff, the existing function-owned Binding SSA
  input projection, the physical role issuer, and the Unit completion consumer.
  Recount production callers and the selected old Accum/PHI edge at the same
  boundary. This is a design stop; it does not wire `route_loop` or add a new
  scheduler.

Contract:
: The caller must run inside `CanonicalModuleLoweringSessionV1`'s unpublished
  candidate and consume the co-sealed profile once. `LocatedStmtV1` and
  `ResolvedFunctionLoweringInputV1` may be used only at the issuer boundary;
  the physicalizer receives Recipe/JoinSig plus owner-issued projections and
  never re-resolves AST, names, or routes. Binding/input/role projections must
  come from the same resolved function owner and existing
  `CanonicalCfgSessionV1`/`BindingSsaBuilderV1`/`PhiTxn` chain. Unit completion
  must use the existing function-exit/value-carrier contract; it may not become
  `None`, a fabricated `ValueId`, or retry.

Done:
: A compact design result identifies the exact production caller seam, proves
  whether the current canonical plan/lowerer can carry it without a second
  owner, and fixes the smallest typed capability product for any missing
  projection or completion handoff. Static census shows the caller count and
  old-edge disposition; focused reject tests cover foreign owner/frame,
  missing projection, and unsupported Unit mapping before Builder effects.

Stop:
: If the only available caller is `route_loop`, a raw AST path, a fresh SSA/CFG/
  PHI owner, a fabricated input value, or an unowned Unit-to-value conversion,
  do not implement. Return to design with that missing capability named. No
  DirectAccum production switch or old-edge deletion is claimed by S3 alone.

S3 audit result:
: The static census found zero production callers for the DirectAccum profile,
  physical input, and borrow physicalizer; the canonical resolved ingress still
  has only `TrivialBindingSsa` and `CurrentCanonicalAPlus` plans, both of which
  reject Loop. The borrow seam is therefore mechanically correct but not yet a
  production capability: the profile effect plan is not consumed by the identity
  ledger, the raw `ValueId` input projection has no owner provenance, and the
  current physicalizer closes the loop with `Return(None)` instead of leaving an
  inline continuation for the outer completion owner.

Recommended next order:
: Use the existing `VerifiedDirectAccumBindingEffectPlanV1` as a role-keyed
  execution claim (without adding a second plan), add an exact
  source-site/BindingRef claim plus non-claim entry-read adapter on the existing
  `ResolvedSsaIdentityStateV2`, split the physicalizer's inline continuation from
  its caller-zero finish/commit wrapper, and only then add one resolved
  DirectAccum plan/caller inside the unpublished candidate. Recipe/JoinSig stay
  portable and source-free; PHI/SSA remains the existing canonical owner.

## S4 — `JOINIR-LOOP-ACCUM-WITNESS-CONTINUATION-ADAPTER-M10A-D2-S4`

Change:
: Reuse the existing `VerifiedDirectAccumBindingEffectPlanV1` as the execution
  claim boundary. Replace positional effect arrays with named DirectAccum roles
  for the three binding reads and two assignment writes; exclude literal RHS
  sites from identity claims. Add a thin resolved identity adapter that consumes
  each role exactly once and delegates to the existing function-owned SSA/PHI
  services. Split the physicalizer's function-shaped `After -> Return(None)`
  behavior into an inline continuation receipt; only the outer lowerer owns
  function completion and final CFG/SSA/PhiTxn finish.

Contract:
: The portable Recipe/JoinSig remains source-free and physical-ID-free. The
  physicalizer sees only a small binding-effect port, canonical CFG, and the
  caller-owned `PhiTxn`; it cannot inspect `ResolvedSsaIdentityStateV2`, names,
  AST, or raw `ValueId` provenance. Exact source-site/BindingRef claims and
  active-binding checks happen atomically in the resolved adapter. Caller-zero
  tests may retain the local raw-binding wrapper, but production may not create
  a second SSA/CFG/PHI owner or commit/abort inside the physicalizer core.

Done:
: Role claims reject wrong frame/owner/site, duplicate or missing roles, and
  RHS-literal misclassification. A trailing-statement fixture proves the
  continuation block remains open and is lowered by the outer owner; no
  physicalizer `Return(None)` or Unit-to-`None` projection remains. Late PHI or
  write failure still aborts the shared transaction and the enclosing candidate
  can be discarded and reused.

Stop:
: If role claims require name lookup, the physicalizer must read the ledger or
  `variable_map` directly, the adapter cannot prove active bindings, or the
  outer completion owner cannot consume Unit without fabricating a value, stop
  before adding a resolved Loop plan. Do not broaden the Trivial profile or
  touch `route_loop` in S4.

Implementation progress (caller-zero only):
: The role-keyed effect plan now lives in the neutral `loop_structural_facts`
  layer, so the canonical adapter does not import compiler-profile ownership.
  `ResolvedSsaIdentityStateV2` exposes exact claim, non-claim entry-read, and
  exact assignment APIs. The physicalizer borrows a named binding port, and
  its production-shaped entrypoint returns an open `After` continuation while
  caller-zero wrappers retain the old finish/commit behavior. Focused
  physicalizer, DirectAccum, resolved-lowering, and binary checks are green.
  This paragraph is historical as of the caller-zero adapter commit.

## Reconciliation after the resolved caller connection (2026-08-04)

The subsequent canonical-candidate connection changed the caller census. The
resolved ingress now reaches `CanonicalDirectAccumSsaLowererV1`, which reaches
`physicalize_direct_accum_v1_with_port` through
`src/mir/builder/resolved_lowering/direct_accum_lowerer.rs`. The structural
guard requires exactly one non-test physicalizer caller and marks the
canonical-resolved-DirectAccum candidate reachable. Therefore the old S3
statement that DirectAccum and the physicalizer have zero production callers
is retained only as historical audit evidence, not as the current state.

The current boundary is narrower than an all-route cutover:

```text
resolved DirectAccum candidate
  -> one canonical CFG / Binding SSA / PhiTxn session
  -> one physicalizer caller
  -> test-only immutable physical snapshot (P4-S1)
```

`route_loop`, the legacy scheduler/registry, Retry/fallback, Generic policy,
old Accum-edge retirement, and default/selfhost authority remain unchanged.
P4-S1 owns the next observer/parity evidence and must not add another lowerer,
writer, or selector. Its implementation closeout must synchronize the P4
cards, Loop pipeline SSOT, PHI/SSA design SSOTs, MIR reference pages,
`src/mir/builder/README.md`, and current pointer mirrors; those reference
updates are acceptance work, not optional cleanup.

## Reconciliation after the P4 candidate audit (2026-08-04)

The first actual candidate audit found that the resolved lowerer closes the
five-block topology with an `After` Unit return but does not yet read/publish
the final `i`/`sum` carriers. This violates the already accepted P1/D1
binding-publication contract; it is not a snapshot-observer discrepancy.

The next implementation authority is the separate task
`JOINIR-LOOP-ACCUM-FINAL-CARRIER-PROJECTION-M10A-D2-S5`. It keeps the caller's
canonical CFG/Binding-SSA/PhiTxn session, seals `After`, reads the verified
carrier keys through `CanonicalDirectAccumBindingPort`, stores a typed
caller-owned receipt, and only then finishes effect claims and the existing
commit transaction. No header-PHI synthesis, second writer, route/retry,
Generic, fallback, grammar, or IR change is allowed. P4-S1 remains paused
until D2-S5 is green.
