---
Status: Design stop — resolved plan capability before production wiring
Date: 2026-08-03
Decision: add an explicit DirectAccum whole-function plan over the existing canonical session
Outcome: implementation is not authorized until the candidate/completion boundary is fixed
Related:
  - joinir-loop-accum-policy-admission-m10a-d2-s1-task-2026-08-03.md
  - joinir-loop-accum-production-bridge-m10a-n2-design-stop-2026-08-03.md
  - ../design/phi-lifecycle-ssot.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
---

# Resolved DirectAccum plan capability: M10a/D2/S2 design stop

## Purpose

The source-owned singleton observation and policy admission are now
caller-zero green. The next boundary is the resolved whole-function plan. It
must enter the existing `CanonicalFirstFamilyPlanV1` without widening the
carrier-free Trivial profile, reviving A+ fallback, or creating a second SSA
owner.

The intended shape is:

```text
CanonicalLoweringPreflightV1
  -> CanonicalFirstFamilyPlanV1::DirectAccum
  -> existing unpublished module candidate
  -> CanonicalSsaFunctionSessionV2::lower_direct_accum facade
  -> existing Unit completion contract
```

The DirectAccum plan is a whole-function profile. It owns the sealed policy
handoff, selected Recipe/JoinSig, effect plan, resolved input/source
projection, and Unit completion witness. It does not own `BindingSsaBuilderV1`,
`CanonicalCfgSessionV1`, `PhiTxn`, `ValueId`, or a live `MirBuilder`.

## Design gates before code

1. **Preflight boundary**: identify the resolved source entry that can issue
   the policy handoff without calling `route_loop`, rescanning AST, or
   manufacturing a frame key. If the current `verify_body` Loop rejection
   cannot be bypassed by a source-owned profile branch, stop and split that
   ingress design first.
2. **Plan ownership**: add one explicit `DirectAccum` variant and update every
   exhaustive match intentionally. Trivial/A+ plans remain unchanged and must
   not gain a Loop arm.
3. **Canonical session factory**: DirectAccum uses the same
   `CanonicalSsaFunctionSessionV2` owner. An empty/loop-specific If-control
   witness must be a named profile input; creating another CFG/SSA/PHI owner
   is forbidden.
4. **Input projection**: current-function `BindingRef`/`ValueId` inputs must be
   issued by the existing identity owner. `Const(0)`, name lookup, raw ValueId
   tables, or a second declaration publisher are NoSafe.
5. **Completion**: Unit must flow through the existing resolved completion
   contract. `None` and fabricated `ValueId` are not valid Loop completion.
6. **Candidate scope**: the sole production physicalizer caller must remain
   inside the unpublished compile candidate; failed lowering drops the whole
   candidate and never retries another route.

## Required proof and tests

- DirectAccum preflight and all existing Trivial/A+ preflight fixtures remain
  behaviorally unchanged.
- `CanonicalFirstFamilyPlanV1::DirectAccum` is issued only from the sealed
  policy handoff and matching function/source/frame owner.
- One canonical session owns all identity/CFG/SSA/PHI state for the function.
- Unit completion seals without `Option`/Retry projection.
- A late physicalization failure leaves the live Builder unchanged and a fresh
  compile succeeds.
- All touched Rust files remain below 800 lines.

## Explicit non-claims

This design stop does not authorize `route_loop`, legacy handler changes,
Generic classification, Retry removal, old Accum/PHI edge retirement, or
final selfhost activation. Those remain later gates after a named resolved
production caller exists.
