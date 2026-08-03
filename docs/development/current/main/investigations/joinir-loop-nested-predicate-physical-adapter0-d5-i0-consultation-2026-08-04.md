# JOINIR-LOOP-NESTED-PREDICATE-PHYSICAL-ADAPTER0-D5-I0

Status: I0-B design closed — bounded resolved-source physicalizer is authorized.
Date: 2026-08-04
Design inputs:

- `joinir-loop-nested-predicate-physical-adapter0-d5-p0-task-2026-08-04.md`
- `joinir-loop-nested-predicate-physical-adapter0-d5-p1-task-2026-08-04.md`
- `joinir-loop-nested-predicate-d4-physical-emission-design-2026-08-04.md`

## Objective

Connect the first Nested physical adapter to the existing unpublished compile
candidate and canonical function session only after the production caller,
fallback authority, and winner-equivalence boundaries are explicitly closed.
This card is the design gate and bounded execution task for D5-I0. I0-A only
issues and tests the sealed plan; I0-B now owns one physicalizer implementation
over the existing canonical session.

## Landed prerequisites

```text
D5-P0  source handoff, Recipe/JoinSig, topology, and candidate block projection
D5-P1  resolver-issued i/sum prefix, declaration-only j, ordered effect claims,
       canonical initialized-readiness gate, and LoopBody retirement harness
```

The only PHI/SSA lifecycle owner remains:

```text
CanonicalSsaFunctionSessionV2
  = ResolvedSsaIdentityStateV2
  + BindingSsaBuilderV1
  + CanonicalCfgSessionV1
  + one PhiTxn
```

Nested must borrow this owner. It must not create a route-local PHI writer,
SSA map, readiness map, Builder transaction, or fallback scheduler.

## Candidate I0 boundary

The proposed production chain is:

```text
resolved source / one selected demand
  -> VerifiedNestedBindingExecutionClaimsV1
  -> VerifiedNestedPhysicalEmissionInputV1
  -> existing canonical session adapter
  -> one physicalizer result
  -> unpublished compile candidate
  -> existing external commit barrier
```

The physical adapter may mutate only the unpublished candidate. A physical
failure is a typed terminal freeze and drops the whole compile candidate. The
live `MirCompiler::builder` must remain unchanged. No `Option`, `Retry`, route
reselection, AST reread, or post-effect `None` projection may cross this seam.

## Consultation questions (must be answered before implementation)

1. Which named production caller owns the Nested ingress, and can its complete
   execution be proven inside `ModuleBuilderInvocationSessionV1` or the already
   established canonical resolved-function draft session?
2. Does the shared `JoinSig` already encode all conditional branch-transfer and
   merge obligations for LoopCond fallthrough/break/continue/return? If not,
   keep I0 parked and open the JoinSig closure task first; do not special-case
   the physicalizer.
3. Which exact legacy JoinIR/family caller is retired in the same cutover, and
   what is the selected Generic V0/V1 post-effect debt disposition? Generic is
   not silently included in Nested admission.
4. What parity evidence compares the legacy winner, Recipe/JoinSig topology,
   canonical CFG/PHI result, diagnostics, and candidate isolation for late
   failure?

## Consultation resolution

The exact existing production chain is now fixed:

```text
MirCompiler::compile_resolved
  -> compile_resolved_first_family
  -> CanonicalLoweringPreflightV1::verify
  -> CanonicalModuleLoweringSessionV1
  -> bind_canonical_source
  -> begin_canonical_invocation
  -> SourceBoundCanonicalPackageV1::consume_parts
```

The first-family sum will gain one bounded variant:

```text
CanonicalLoopFamilyPlanV1::NestedPredicate
  -> ExactCanonicalPreflightPlanV1::Loop
  -> existing BindingSsaTrivial invocation family
```

No new `ModuleInvocationFamilyV1` is needed. The sole production physical
owner is the existing `CanonicalSsaFunctionSessionV2`; Nested borrows its
`ResolvedSsaIdentityStateV2`, `CanonicalCfgSessionV1`, `ResolvedSemanticStackV1`,
and one `PhiTxn`. A route-local PHI/SSA session is forbidden.

The Nested probe must run before the existing DirectAccum probe. Both pilots
use the top-level `Local + Loop` envelope; allowing DirectAccum to inspect the
shape first would turn a Nested producer failure into a misleading DirectAccum
terminal. The probe must therefore classify the Nested sentinel first and,
once present, make every producer/effect/JoinSig failure a typed terminal. Only
a clearly non-Nested shape may continue to DirectAccum or the ordinary family.

The plan issuer consumes the already-sealed products in this order:

```text
projection -> Nested Recipe product
           -> P1 effect claims from product.source_handoff()
           -> physical emission input (Recipe consumed once)
           -> owner/frame/header seals
```

It must provide the same resolved-owner header seal as DirectAccum so the
existing source-bound package can bind it without a second lifecycle family.

I0 admission is limited to the currently sealed Nested fixture:

```text
Enter -> PredicateTrue / PredicateFalse -> Backedge
```

Mixed break/continue/return/fallthrough transfer remains a typed reject until
the shared JoinSig branch-transfer owner is closed. Generic V0/V1 debt and all
other Loop families remain on their existing lanes.

The implementation is explicitly scoped to `compile_resolved` and its
unpublished `CanonicalModuleLoweringSessionV1` candidate. It does not claim
that public `compile_with_source` or the `.hako` normal ingress consumes this
Nested plan.

## Minimum I0 implementation slice

```text
I0-A  add Nested plan/probe and first-family variant; reject unsupported shapes
I0-B  add canonical resolved Nested lowerer and one physicalizer core
I0-C  add the resolved-source cutover orchestrator, using DirectAccum's chain
I0-D  add one representative fixture, legacy parity, and late-failure reuse
I0-E  retire only the selected Nested legacy edge and prove caller counts
```

I0-A/B must remain one route/family only. Do not add a universal Loop
physicalizer, Generic fallback, route registry rewrite, or all-route cutover in
this slice.

## I0-A landed boundary

The first-family preflight now probes the Nested structural sentinel before
DirectAccum and issues `CanonicalNestedPredicatePlanV1` from the existing
projection, Recipe/JoinSig/topology, effect claims, completion, and owner/frame
seals. The source-bound package has typed placeholder arms for the still
unimplemented physicalizer; no Nested MIR writer or production cutover is
claimed by this milestone.

Next is I0-B: borrow the existing canonical SSA/CFG/PHI owner for one bounded
Nested physicalizer, with candidate-only mutation and terminal failure.

## I0-B design resolution

The worker API audit confirms that the existing DirectAccum lowerer is the
correct lifecycle template:

```text
CanonicalModuleLoweringSessionV1 (outer candidate)
  -> CanonicalFunctionLoweringSessionV1 (function draft)
  -> CanonicalSsaFunctionSessionV2
       identity + semantic stack + CanonicalCfgSessionV1 + one PhiTxn
  -> Nested role adapter + Nested physicalizer
  -> existing draft seal / external commit
```

Nested adds no module family, function session, SSA map, PHI transaction, or
route scheduler. The only new boxes are a sibling lowerer, nine-role identity
adapter, and topology-driven physicalizer. The adapter must publish `i` and
`sum`, activate declaration-only `j`, then consume the exact first assignment
before any `j` read. The semantic stack enters root and child LoopBody pairs;
the existing `close_scope_region_success` contract is sufficient because the
child pair has no declarations and the root pair retires `j` exactly at
`Root.After` (proven by the P1 scope test).

The physicalizer allocates the verified ten unique blocks plus ParentResume,
uses the existing topology alias for child preheader, and leaves Root.After
open for the existing completion/finalization path. It must emit only the
sealed `Enter -> PredicateTrue/False -> Backedge` shape; mixed transfer remains
a typed reject. Required I0-B tests are end-to-end MIR verification, owner/
frame/block alias checks, role ordering and read-before-init rejection,
predecessor seals, and candidate drop/fresh-request reuse after injected late
failure.

## Acceptance gates

```text
named production caller = exactly 1
Nested physicalizer production caller = exactly 1
caller is inside unpublished compile candidate = proven
live Builder direct caller = 0
selected old fallback caller = 0 after cutover
post-effect Option/Retry/route reselection = 0 on the new subtree
legacy/new winner and MIR parity = green for the selected fixture
late physical failure leaves live builder and fresh request unchanged
no route-local PHI/SSA/identity/scope authority
all touched Rust/test files < 800 lines
```

## Explicit non-claims

This card does not claim that all Loop routes have one winner, that Generic
debt is solved, that route-specific PHI materializers are retired, or that the
portable Recipe is already the production authority. Those remain later gates.

## Remembered convergence task

Keep the named post-cutover cleanup task visible:

```text
JOINIR-LOOP-RECURSIVE-FRAME-CONVERGENCE0-M12
```

It must close JoinSig branch transfer/merge obligations, unify LoopV0/True/Cond
as one recursive semantic frame, recurse Nested through that frame, classify
Generic debt separately, and prove family callers=0 / one physicalizer / no
Option or Retry before it is considered complete.
