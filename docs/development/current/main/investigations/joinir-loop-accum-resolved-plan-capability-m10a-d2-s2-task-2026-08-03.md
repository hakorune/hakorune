---
Status: Accepted task — resolved DirectAccum plan over the existing canonical session
Date: 2026-08-03
Decision: add one explicit DirectAccum whole-function plan; reuse the existing candidate and SSA/CFG/PHI owner
Related:
  - joinir-loop-accum-resolved-plan-capability-m10a-d2-s2-design-stop-2026-08-03.md
  - joinir-loop-accum-policy-admission-m10a-d2-s1-task-2026-08-03.md
  - ../design/phi-lifecycle-ssot.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
---

# Resolved DirectAccum plan capability: M10a/D2/S2

## Objective

Turn the caller-zero source observation and policy handoff into one resolved
whole-function capability. The capability must be issued before any Builder
effect and consumed once by a DirectAccum lowering facade inside the existing
unpublished compile candidate.

```text
ResolvedFunctionLoweringInputV1 + exact source/frame
  -> VerifiedDirectAccumSingletonObservationV1
  -> VerifiedDirectAccumPolicyHandoffV1
  -> CanonicalDirectAccumPlanV1
  -> existing CanonicalModuleLoweringSessionV1 candidate
  -> one CanonicalSsaFunctionSessionV2 owner
```

`CanonicalDirectAccumPlanV1` owns only source-side evidence: the policy
handoff, selected recipe/product, role-keyed effect plan, resolved input
projection, and function completion witness. It must not own
`BindingSsaBuilderV1`, `CanonicalCfgSessionV1`, `PhiTxn`, `ValueId`,
`BasicBlockId`, or a live `MirBuilder`.

## Implementation order

1. Add a source-owned `verify_direct_accum_function_v1` entry that does not
   call `route_loop`, the 19-route registry, legacy `CanonicalLoopFacts`, or
   an AST rescan. It consumes the already sealed singleton observation and S1
   policy handoff exactly once.
2. Add the smallest DirectAccum lowerer facade over the existing
   `CanonicalSsaFunctionSessionV2`. Prefix bindings, CFG, PHI, and completion
   all use that same session. The loop physicalizer receives only owner-issued
   input/role projections and returns a typed Unit/After continuation.
3. Use the existing `CanonicalModuleLoweringSessionV1` as the sole abort
   boundary. A physicalization or completion error drops the whole candidate;
   there is no next-route retry or live-Builder mutation.
4. Add focused success, late-failure, candidate-discard, and fresh-reuse
   tests before any production caller beyond the canonical resolved compile
   entry is enabled.
5. Only after the lowerer and candidate proofs are green, add the explicit
   `CanonicalFirstFamilyPlanV1::DirectAccum` variant and update every
   exhaustive match intentionally. Trivial and CurrentCanonicalAPlus keep
   their current contracts; no Loop arm is added to either lowerer. This
   ordering keeps the central enum/source-bound package out of the proof until
   the concrete consumer is known to be sound.

## Hard gates

- DirectAccum is issued only from the sealed source/frame/policy product.
- Exactly one `CanonicalSsaFunctionSessionV2` owns identity, SSA, CFG, PHI,
  semantic stack, and completion for the function.
- Inputs are issued by the canonical identity owner; name lookup, `Const(0)`,
  raw `ValueId` tables, and a second declaration publisher are forbidden.
- Unit completion uses the resolved completion contract; `Option`, fake
  `ValueId`, `None`, and `Retry` are absent from the new facade.
- The physicalizer has one production caller inside the unpublished candidate;
  live `compiler.builder` remains unchanged on failure and a fresh compile
  succeeds.
- Existing Trivial/A+ fixtures remain green and behaviorally unchanged.
- Every touched Rust source file remains below 800 lines.

## PHI/SSA boundary

SSA/CFG/PHI ownership is already SSOT'd by
`CanonicalSsaFunctionSessionV2` → `ResolvedSsaIdentityStateV2` /
`BindingSsaBuilderV1` + `CanonicalCfgSessionV1` + `PhiTxn`. This task consumes
that owner; it does not create another one.

This task does not retire route-specific PHI materializers, the legacy
`phi_input_materializer`, or JoinIR fallback. Making the Recipe-specific PHI
writer the sole production writer is the later M6 cutover.

## Explicit non-claims

This task does not wire `route_loop`, classify Generic V0/V1, remove legacy
Retry, alter the old scheduler, retire old Accum/PHI edges, or activate
selfhost. Those require separate gates after this capability has a named
resolved production caller.

## Implementation progress

The first S2 slice is caller-zero green. A new profile ingress consumes
`VerifiedDirectAccumPolicyHandoffV1` exactly once and builds the existing
Recipe/effect-plan product without reselecting a winner or reprojecting source
facts. The old winner-based profile helper is test-only parity evidence. The
focused DirectAccum/policy suites, binary check, diff check, and all touched
source line-count checks are green.

The resolved issuer now owns the full preflight sequence as
`issue_direct_accum_plan_v1`: exact source lookup → structural projection →
singleton observation → policy handoff → `CanonicalDirectAccumPlanV1`. This
keeps the one-shot boundary explicit while the plan is still caller-zero.

The next safe slice is now landed as a direct-only preflight module:
`verify_direct_accum_function_v1(function, exact_loop_stmt)`. It validates the
closed function prefix/suffix and completion contract, then calls the one-shot
issuer. The ordinary `verify(unit)` and source-bound package remain untouched;
no central family enum or production caller has been widened yet.

The plan now retains a non-Clone `VerifiedDirectAccumPolicyReceiptV1` after
consuming the handoff's winner for Recipe demand. This preserves policy
provenance without retaining a raw schedule cursor or adding a second policy
authority.

The plan now also owns an AST-free `VerifiedDirectAccumPrefixInputV1`. The
source projection seals the two local declaration sites, BindingRefs, kinds,
diagnostic names, and zero initializers once; the lowerer does not rescan the
prefix AST or reconstruct declaration identity.

The candidate-only canonical lowerer facade is now caller-zero green. It uses
the existing `CanonicalSsaFunctionSessionV2`, publishes the sealed prefix,
consumes the role-aware physicalizer, seals the open `After` block without
writing `Return`, and hands implicit completion to the existing draft seal.
Focused evidence covers successful candidate lowering, a late draft-seal
failure that leaves the live Builder untouched after candidate discard, and
fresh candidate reuse after the discard. The central family enum and
source-bound package were the next design/implementation slice; their
exhaustive matches and typed exclusion boundary are now landed without a
caller switch.

The CENTRAL-ENUM-SOURCE-BOUNDARY0-S0 slice is now implemented. The compiler
family sum owns `DirectAccum(CanonicalDirectAccumPlanV1)`, the exact
source-bound sum carries it without adding a module/header family, and the
source-bound consumer calls only the candidate DirectAccum draft lowerer. The
ordinary Trivial/A+ path has an explicit typed exclusion, while the existing
canonical SSA/CFG/PHI session remains the sole physical owner. Focused
DirectAccum, capability, source-binding, no-run, binary-check, pointer-guard,
and under-800-line checks are green. No `route_loop`, public compile switch,
Generic/Retry change, or PHI-writer retirement was made.

## Next design stop: central family-plan/source-bound boundary

The worker audit closed this boundary without authorizing a caller switch.
The selection authority remains in `src/mir/compiler`:
`CanonicalLoweringPreflightV1` issues one
`CanonicalFirstFamilyPlanV1::DirectAccum(CanonicalDirectAccumPlanV1)`. The
`resolved_lowering` layer remains only the concrete plan consumer and keeps
`CanonicalSsaFunctionSessionV2` as the sole SSA/CFG/PHI owner.

The source-bound package remains closed at the module-lifecycle level, but the
candidate pilot may carry a DirectAccum plan in its exact plan sum. It reuses
the existing `BindingSsaTrivial` module token/header/policy because that is the
external candidate/collector/finalization lifecycle, not because the loop is
semantically a Trivial body. No new `ModuleInvocationFamilyV1`,
`ResolvedOwnerHeaderFamilyV1`, or finish schedule is introduced merely for a
body profile. The mapping is documented and tested at the source-bound seam;
the compiler-layer `CanonicalFirstFamilyPlanV1` remains the one selection
authority.

The next implementation slice is limited to the compiler enum arm, its brand
and exhaustive typed exclusions, the exact source-bound plan arm, a
direct-only preflight constructor, and unchanged Trivial/A+ evidence.
`route_loop`, public caller wiring, Generic or Retry removal, PHI-writer
retirement, old-edge deletion, and selfhost remain explicit non-claims. A
later function-body integration must decompose the current whole-function
pilot into a body-owned DirectAccum plan that borrows the outer canonical
session; it must not nest a second completion/SSA owner.

## Accepted execution brief: CENTRAL-ENUM-SOURCE-BOUNDARY0-S0

Change: add `DirectAccum(CanonicalDirectAccumPlanV1)` to the compiler-owned
family sum and carry it through the exact source-bound plan sum. Reuse the
existing `BindingSsaTrivial` module/header lifecycle identity only at that
external boundary; do not add a module/header family or a public caller.

Contract: the direct-only issuer is the sole producer of the new arm. The plan
contains source/policy/Recipe/effect/completion evidence only; the existing
canonical session remains the sole identity/SSA/CFG/PHI owner. Existing
Trivial/A+ consumers reject DirectAccum explicitly and never fall through.

Done: central/source-bound exhaustive matches compile; DirectAccum header and
token mapping are tested; source-bound consume calls only the candidate
DirectAccum lowerer; existing Trivial/A+ and candidate abort/fresh-reuse gates
remain green; touched Rust stays below 800 lines.

Stop: stop and return to design if a match requires a new module/header family,
an AST rescan, a second owner, a live-Builder caller, or a fallback/retry
projection. Do not wire `route_loop` or public `compile_resolved` in this row.

## Next accepted slice: CANDIDATE-LOWER-CALLER0-S1

Worker design decision: exercise the existing named terminal
`MirCompiler::lower_canonical_source` with the DirectAccum source-bound
package. This is a candidate-only pilot, not a public compile cutover. The
terminal opens the existing `CanonicalModuleLoweringSessionV1`, consumes the
package once, and returns `CanonicalLoweringCandidateV1`; live Builder state,
module publication, and collector/finalization remain untouched.

Acceptance:

1. A DirectAccum source-bound package reaches
   `lower_canonical_source` and returns one `LoweredCanonicalPlanV1::Single`.
2. The candidate's draft is produced by the existing DirectAccum lowerer and
   the same `CanonicalSsaFunctionSessionV2`; no second SSA/CFG/PHI owner or
   source re-observation is introduced.
3. The compiler's live Builder remains unopened after candidate success and
   after an injected late failure; a fresh candidate can be opened afterward.
4. Existing Trivial/A+ source-bound tests remain green and every touched Rust
   file stays below 800 lines.

The broad logical-demand guard now has a precise exception for the named
compiler-owned `direct_accum_profile.rs` issuer and source projection, and
requires the issuer's two producer call sites while rejecting every other
production caller. This is guard evidence only; it does not activate a route.
`route_loop`, public `compile_resolved`, Generic/Retry classification,
PHI-writer retirement, and external commit remain explicit non-claims.

S1 is now green in the disconnected candidate lane. The focused test invokes
`MirCompiler::lower_canonical_source` with a DirectAccum package and proves
that the result is a single lowered draft while the live Builder remains
unopened. The shared in-place replacement guard now recognizes exactly the
compiler-owned DirectAccum issuer and source projection, requires its two
producer calls, and rejects any other production caller. No runtime route or
publication authority was added.

## Next design stop: RESOLVED-FIRST-FAMILY-CUTOVER-M10A-D2

Worker decision: the first real production caller must be the existing
`MirCompiler::compile_resolved_first_family`, reached through the public
`compile_resolved` owner. `route_loop` is rejected because it receives a live
`&mut MirBuilder` and raw AST-shaped inputs before source/frame/candidate
proof; wiring it would preserve effect-after-retry and dual SSA authority.
A permanent `compile_resolved_direct_accum` opt-in is also rejected because it
would create a second ingress; a canary may only be a temporary proof, never a
final owner.

The proposed I0/R0 boundary is one exact resolved singleton:

```text
CanonicalLoweringPreflightV1
  -> DirectAccum exact source adapter
  -> CanonicalFirstFamilyPlanV1::DirectAccum
  -> bind_canonical_source
  -> begin_canonical_invocation
  -> lower / collect / completion / drain / finalization / postprocess
  -> one prepared external commit
```

Required proof before implementation:

- exact `AccumConstLoop` winner is selected once before Builder effects;
  non-direct first-family fixtures preserve Trivial/A+ parity and other Loop
  shapes reject by typed error without legacy fallback;
- DirectAccum has exactly one production physicalizer caller and consumes the
  existing `CanonicalSsaFunctionSessionV2` (`BindingSsaBuilderV1`,
  `CanonicalCfgSessionV1`, `PhiTxn`) exactly once;
- every lower/collect/completion/final-verifier/finalization failure drops
  the unpublished candidate, leaves the live Builder unchanged, and performs
  zero external commit; success commits exactly once and fresh reuse works;
- the old `CapabilityNotActivated { boundary:
  "direct_accum_source_bound" }` edge is removed only in the same cutover;
  `route_loop`, registry, `CanonicalLoopFacts`, Generic/Retry, and old PHI
  writers are not fallback paths for this profile;
- existing resolved Trivial/A+ tests and all touched Rust files remain green
  and below 800 lines.

Non-claims: normal/default `compile_with_source`, all-loop support, Generic
V0/V1 debt, nested/Call/Record/Match, selfhost, and repository-wide PHI-writer
retirement remain separate tasks. PHI/SSA design and lifecycle SSOT is already
complete; this stop concerns only production caller convergence.
