---
Status: Implementation task — source-backed G0 helper backend reach
Date: 2026-09-12
Decision: LOOP-G0-HELPER-BACKEND-REACH-I1
Parent: mirbuilder-loop-g0-helper-backend-reach-d0-2026-09-12.md
NextCard: none__G0HelperBackendReach__PendingCloseout
---

# Generic G0 helper backend reach I1

## Execution brief

Change:
  Make the focused normal-package G0 witness call `generic_g0(0, 0)` from
  `Main.main`, and assert the existing source-backed Global Call selects
  `generic_g0/2` in the lifecycle physical program and ABI input.

Contract:
  Reuse `VerifiedNormalCallableSemanticPackageV1`, the existing G0 consumer,
  `PublishedMirBackendView::try_new`, `collect_ordinary_calls`, and the Single
  lifecycle. The Call relation and canonical definition table are the only
  helper membership authority; no module-name scan, AST re-resolution,
  synthetic Call, new receipt, second issuer, fallback, or retry.

Done:
  Focused normal-package positive proves root plus `generic_g0/2` physical
  membership and exact two-argument integer ABI; the existing ignored EXE
  witness runs to exit `3` when selected FFI, lifecycle kernel, ny-llvmc, and
  LLVM18 are available. Missing/foreign definition or Call relation rejects
  before artifact publication. Run the reusable current-state guard and
  focused tests with one Cargo process.

Stop:
  Return to design if the existing Global Call cannot issue the exact
  `free_function("generic_g0", 2)` relation, if physical ABI membership needs
  a new owner, or if the runtime witness requires Pair/entry behavior unrelated
  to the helper Call.

## Exact acceptance

- Positive: the source-backed package contains the G0 loop and
  `Main.main { return generic_g0(0, 0) }`; the physical program has exactly the
  selected root and the called `generic_g0/2` helper, with two helper lanes and
  integer result; the lifecycle ABI input retains the same membership.
- Negative: the existing physical membership/definition or Call-to-definition
  relation mutation rejects before LLVM/C artifact output; no module presence
  assertion may substitute for this check.
- EXE: when the already-required environment is present, the existing typed
  emitter runs the witness and the executable exits `3`. LLVM18 unavailable is
  an explicit skip, never a success claim.
- Guard: no `source_ast()` re-resolution, module-name selection, synthetic
  Call, route-loop entry, test-only emitter, fallback/retry, or second issuer.

## Scope boundary

This row proves only source-backed G0 helper reach through the existing
ordinary Global Call. It does not widen G0 to cataloged methods, all-family
Loop selection, backend parity, real-app coverage, Call/R7 completion, or
legacy retirement.
