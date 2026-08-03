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
2. Add an explicit `CanonicalFirstFamilyPlanV1::DirectAccum` variant and update
   every exhaustive match intentionally. Trivial and CurrentCanonicalAPlus
   keep their current contracts; no Loop arm is added to either lowerer.
3. Add the smallest DirectAccum lowerer facade over the existing
   `CanonicalSsaFunctionSessionV2`. Prefix bindings, CFG, PHI, and completion
   all use that same session. The loop physicalizer receives only owner-issued
   input/role projections and returns a typed Unit/After continuation.
4. Use the existing `CanonicalModuleLoweringSessionV1` as the sole abort
   boundary. A physicalization or completion error drops the whole candidate;
   there is no next-route retry or live-Builder mutation.
5. Add focused success, late-failure, candidate-discard, and fresh-reuse
   tests before any production caller beyond the canonical resolved compile
   entry is enabled.

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

The remaining work is the candidate-only canonical lowerer facade and its
success/failure/fresh-reuse proof. The central family enum and source-bound
package stay parked until that lowerer is green.
