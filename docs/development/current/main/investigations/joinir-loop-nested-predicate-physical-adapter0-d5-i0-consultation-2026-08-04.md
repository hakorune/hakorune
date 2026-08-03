# JOINIR-LOOP-NESTED-PREDICATE-PHYSICAL-ADAPTER0-D5-I0

Status: design consultation stop — production cutover is not yet authorized.
Date: 2026-08-04
Design inputs:

- `joinir-loop-nested-predicate-physical-adapter0-d5-p0-task-2026-08-04.md`
- `joinir-loop-nested-predicate-physical-adapter0-d5-p1-task-2026-08-04.md`
- `joinir-loop-nested-predicate-d4-physical-emission-design-2026-08-04.md`

## Objective

Connect the first Nested physical adapter to the existing unpublished compile
candidate and canonical function session only after the production caller,
fallback authority, and winner-equivalence boundaries are explicitly closed.
This card is the design gate for D5-I0; it is not permission to wire a route.

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

## Minimum I0 implementation slice after approval

```text
I0-A  caller census and named unpublished-candidate owner
I0-B  one Nested physicalizer consumer of sealed P0/P1 products
I0-C  one representative production fixture with legacy parity
I0-D  late-failure candidate discard + fresh-request reuse
I0-E  atomic caller switch and selected legacy-edge deletion
```

I0-A/B must remain one route/family only. Do not add a universal Loop
physicalizer, Generic fallback, route registry rewrite, or all-route cutover in
this slice.

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
