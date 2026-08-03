# JOINIR-LOOP-NESTED-PREDICATE-PHYSICAL-ADAPTER0-D5-I0

Status: I0-B/C implemented — bounded resolved-source physicalizer and candidate isolation are green; I0-D is now a parity design stop and I0-E is a scoped retirement proof.
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

The first-family sum now carries one bounded variant:

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

I0-B/C are now landed for the resolved source-bound fixture: the sibling
lowerer, nine-role identity adapter, topology-driven physicalizer, and cutover
orchestrator all borrow the existing canonical candidate/function/SSA/CFG/PHI
owners. End-to-end MIR verification, same-compiler reuse, and a test-only
prepared-commit failure that drops the unpublished product while preserving the
live Builder are green. The remaining I0-D/E work is legacy/new winner parity
and selected legacy-edge retirement; no mid-physicalizer production fault hook
is authorized.

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

The topology's `ChildUpdate` destination is intentionally `Child.Step`: the
physical adapter splits the child-body source order into ancestor `sum` update
in `Child.Body`, then recurrence `j` update in `Child.Step`, before the
`Step -> Header` backedge. This is a semantic-to-physical placement rule, not a
second source claim or a silent role drop; I0-B must assert it explicitly.

## D5-I0-D design stop: winner parity and retirement boundary

The new production authority is deliberately narrow:

```text
compile_resolved
  -> CanonicalNestedPredicatePlanV1
  -> Nested canonical physicalizer
  -> unpublished compile candidate
```

The old `route_loop` / `route_nested_loop_minimal` path is not its parity
oracle. Caller census shows that it remains reachable from the normal/raw
ingress (`try_cf_loop_joinir`, `recursive_child_lowering`, and
`raw_loop_child_entry`). Deleting that route globally would break the still
legacy normal path and is outside this card.

The exact Nested fixture also cannot be compared through the public legacy
entrypoints: `compile_with_source` stops at callable semantic
incomplete-consumption, while `compile_raw_with_source` stops at typed raw
eligibility rejection, before a comparable old MIR is produced. Therefore
raw `MirPrinter` bytes are not a valid D5-I0 parity oracle.

Recommended next slice is a test-only legacy route oracle (effective winner
and semantic digest from the existing facts/registry boundary), followed by
alpha-normalized CFG/terminator/PHI invariants, MIR verification, and the
existing runtime result `9`. The new subtree must still have one winner and
zero post-effect `Option`/`Retry`. A broader normal-ingress cutover is a
separate task.

I0-E is consequently split into two proofs:

1. prove that the selected resolved ingress has zero old fallback callers
   (already structurally true in `source_bound_package.rs`, to be guard-
   recorded); and
2. retire `route_nested_loop_minimal` only in a later normal-ingress cutover
   after its remaining callers reach zero.

Stop and return to design if the effective legacy winner cannot be isolated,
or if the semantic digest/CFG proof shows that Nested is not the same winner
as `NestedPredicate`.

## D5-I0-D execution brief (design closed)

Change:
  Add a `cfg(test)` legacy effective-winner oracle at the existing
  facts/registry boundary.  The selection projection remains read-only; a
  separate test-only receipt may execute the selected legacy route against a
  disposable seeded `MirBuilder` so that “effective winner” means the route
  that actually returns `Succeeded`, not merely the first registry candidate.
  Neither helper is a production selector or ingress.

Contract:
  The oracle may call the existing loop-facts canonicalizer, legacy shadow
  observer, and (only inside the disposable test receipt) the existing ordered
  route executor. It must not become a production selector, physicalizer,
  fallback, or public ingress. The old normal/raw route remains authoritative
  outside the resolved-source pilot, and no live compiler Builder is mutated
  by the oracle.

Done:
  The focused profile and registry fixture prove `NestedLoopMinimal` is the
  legacy effective winner (the only attempted route and the successful route),
  `NestedPredicate` is the resolved winner, the sealed topology and role digest
  agree, verification/runtime remain green, and the new subtree still has no
  post-effect retry surface. The selected resolved ingress has a recorded
  guard target; the guard itself is the next I0-E slice and is not claimed
  closed by this oracle.

The first I0-E runtime guard is now green: resolved Nested compilation records
zero legacy loop-physicalizer effects. A structural caller census/guard is
still required before the old resolved-ingress edge can be called caller-zero;
this test does not authorize deleting the normal/raw `route_nested_loop_minimal`
path.

Stop:
  If the helper requires Builder mutation, public-ingress widening, or a
  second route selector, return to design. If the winners or semantic digest
  disagree, do not retire any legacy edge.

## Post-D5 convergence queue (ordered, do not start during this design stop)

The following cleanup is intentionally recorded here so it is not lost, but
does not widen D5-I0 or claim that the canonical owner is already universal.

```text
JOINIR-LOCATED-LEGACY-RETIRE0-S0
  caller census -> move the test oracle -> delete only caller-zero
  `located_legacy_*` session/adapters. Do not delete live `located_if.rs` or
  `IfCfgSessionV1`; they belong to the later resolved A+ adoption lane.

JOINIR-IF-RECIPE-CANONICAL-SSA-D0
  audit the four actual If surfaces (raw IfForm, CorePlan::If/JoinIR,
  resolved trivial canonical SSA, and resolved A+ IfCfgSessionV1); define one
  portable IfRecipeV1/IfJoinSig boundary, name each current authority and
  fail-fast seam, and explicitly do not claim a universal PHI writer yet

JOINIR-IF-RECIPE-PHYSICAL-ADAPTER0-I0
  pilot one bounded explicit-else scalar-rebind If family. The producer is
  Builder-free; the consumer borrows the existing unpublished candidate and
  CanonicalSsaFunctionSessionV2/CanonicalCfgSessionV1/one PhiTxn. Prove
  semantic CFG/PHI parity before retiring one old If edge

JOINIR-IF-RECIPE-ADOPTION0-A1
  retire only the selected resolved-canonical-trivial If writer after the
  pilot proves producer=1, physicalizer=1, Option/Retry=0, MIR/CFG/PHI and
  candidate-isolation parity. Do not use this as production-wide SSA adoption.

JOINIR-IF-RECIPE-ADOPTION0-B1
  separately migrate resolved A+ `located_if.rs` / `IfCfgSessionV1` branch
  transactions after JoinSig transfer/cleanup obligations are typed and
  closed; do not mix this owner with the trivial pilot.

JOINIR-IF-RECIPE-ADOPTION0-D1
  separately classify `CorePlan::If` / `apply_if_joins` / direct
  `phi_input_materializer` as a mechanical planner adapter, then close one
  selected planner-family caller at a time. CorePlan is not the portable
  semantic SSOT.

JOINIR-SSA-PHI-CANONICAL-ADOPTION0-M10
  tracking umbrella, not one giant cutover. Its execution rows are A1
  (resolved-trivial If), B1 (located A+ If), D1 (CorePlan/JoinIR), and D2
  (loop-variant PHI materializers), each with its own selected-writer
  caller-zero gate. Only after all rows are green may the umbrella claim
  Binding SSA + Canonical CFG + one PhiTxn as the production-wide PHI owner.

JOINIR-RAW-DESCENT-PARITY-RETIRE0-S0
  after caller-zero proof, collapse only the six raw/descent/parity facades
  that have actually become dead. Keep raw `IfForm` as the normal/default
  production authority until its own ingress cutover; the resolved If pilot
  does not make the raw caller zero.

JOINIR-LOOP-RECURSIVE-FRAME-CONVERGENCE0-M12
  unify LoopV0/LoopTrue/LoopCond as one recursive semantic frame, recurse
  Nested through that frame, and keep Generic post-effect debt separate
  until its own winner classification is closed
```

The If and PHI rows are opportunities, not current claims. `BindingSsaBuilderV1`,
`CanonicalCfgSessionV1`, and `PhiTxn` are already the SSA/PHI design SSOT, but
route-specific writers and legacy JoinIR remain production edges until the
M10 adoption gates close.

Feedback audit (2026-08-04): the “If is the next Loop” direction is valid, but
the implementation inventory is not a simple two-way split. The four surfaces
above have different callers and PHI owners, so the D0 census must precede any
adapter or deletion. `IfRecipeV1` is the semantic boundary; it does not by
itself make `IfCfgSessionV1`, legacy `IfForm`, CorePlan/JoinIR, or canonical SSA
the production authority. The cheap `located_legacy_*` cleanup is ordered first
only as a caller-zero retirement proof, not as an unconditional deletion. The
Loop frame convergence task stays post-cutover and keeps Generic debt separate.

## Acceptance gates

```text
named production caller = exactly 1
Nested physicalizer production caller = exactly 1
caller is inside unpublished compile candidate = proven
live Builder direct caller = 0
selected resolved-ingress old fallback caller = 0 (guard proof)
  runtime legacy-loop effect guard = 0 (green); structural caller-zero proof pending
post-effect Option/Retry/route reselection = 0 on the new subtree
  legacy/new effective winner and semantic CFG/PHI parity = green for the selected fixture
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
