---
Status: implementation_landed__EXEAcceptanceOpen__2026-09-12
Date: 2026-09-12
Decision: LOOP-G0-HELPER-BACKEND-REACH-I1
Parent: mirbuilder-loop-g0-helper-backend-reach-d0-2026-09-12.md
NextCard: existing physical JSON and V4 scalar CFG consumer; see R7 current card
---

# Generic G0 helper backend reach I1

## Execution brief

Change:
  Make the focused normal-package G0 witness call `generic_g0(0, 0)` from
  `Main.main`, and assert the existing source-backed Global Call selects
  `generic_g0/2` in the lifecycle physical program and helper body.

Contract:
  Reuse `VerifiedNormalCallableSemanticPackageV1`, the existing G0 consumer,
  `PublishedMirBackendView::try_new`, `collect_ordinary_calls`, the existing
  physical-program issuer, and the Single G0 lowering lifecycle. The Call
  relation and canonical definition table are the only helper membership
  authority; no module-name scan, AST re-resolution, synthetic Call, new
  receipt, second issuer, fallback, or retry.

Done:
  Focused normal-package positive proves exactly root plus the source-called
  `generic_g0/2` in the physical program and exact two-argument integer ABI;
  it also observes the helper body `Compare` and `Add` operations. The
  existing typed EXE witness must emit and run the helper with exit code 3.
  This acceptance remains open at the physical JSON instruction-vocabulary boundary.
  Missing/foreign definition or Call relation rejects before artifact
  publication. The selected production Loop paths reuse the one preflight
  `BuilderInvocationConfigV1` snapshot when opening their physical session;
  no second environment/import/plugin-signature snapshot is taken there.

Stop:
  Return to design if the existing Global Call cannot issue the exact
  `free_function("generic_g0", 2)` relation, if physical-program membership
  needs a new owner, or if the runtime witness requires Pair/entry behavior
  unrelated to the helper Call.

## Exact acceptance

- Positive: the source-backed package contains the G0 loop and
  `Main.main { return generic_g0(0, 0) }`; the physical program has exactly the
  selected root and the called `generic_g0/2` helper, with two helper lanes and
  integer result. The ignored EXE witness uses the same selected view and
  existing static emitter when its external toolchain is available.
- Negative: the existing physical membership/definition or Call-to-definition
  relation mutation rejects before LLVM/C artifact output; no module presence
  assertion may substitute for this check.
- EXE: when the already-required environment is present, the existing typed
  static emitter runs the witness and the executable exits `3`. LLVM18
  unavailable is an explicit skip, never a success claim.
- Guard: no `source_ast()` re-resolution, module-name selection, synthetic
  Call, route-loop entry, test-only emitter, fallback/retry, or second issuer.
- Configuration: `compile_resolved_first_family` reads the canonical config
  once for G0 policy and passes that same value through the selected physical
  invocation; the compatibility wrapper snapshots only for older direct test
  callers.

## Implementation evidence and classification (2026-09-12)

- `CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 cargo check --profile quick --lib`:
  pass. The workspace's existing warning inventory remains informational.
- `CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 cargo test --profile quick --lib
  --no-run`: pass; the test binary was built with one Cargo/rustc process.
- The focused normal-package physical-reach test: pass. It observes exactly
  one root `Invoke(Call)` to the canonical `generic_g0/2`, its two integer
  lanes, and helper `Compare`/`Add` body operations.
- The same-module free-function `Invoke(I64)` verifier regression: pass.
- Before installation the ignored EXE witness skipped for missing LLVM18.
  At `3d3b118ccf`, after LLVM18.1.8 installation, the corrected named test
  `normal_package_generic_g0_helper_reaches_existing_exe_emitter` ran once
  and rejected at `published-lifecycle-physical-abi/site-missing` before
  object emission. The existing archive was temporarily exposed from
  `target/lifecycle-kernel/release` at the test's hardcoded `target/release`
  path; the link was removed afterwards. Physical membership assertions
  passed before this rejection; LLVM generation and exit code 3 are unproven.
  This is acceptance debt, not evidence of a regression caused by the later
  documentation-only commit. See the LLVM18 installation task for commands.
- `published_consumer_runs_once_and_propagates_failure_without_retry`: known
  baseline red; the same `calls = 0` versus `1` failure reproduces with the
  pre-change test binary, so it is not evidence against this slice.
- `tools/checks/rust_mirbuilder_generic_g0_normal_package_consumer_i0_guard.sh`,
  `tools/checks/current_state_pointer_guard.sh`, and `git diff --check`: pass.

The config handoff is behavior-preserving: selected first-family production
cutovers receive the already-captured invocation config, while old direct test
helpers retain their compatibility snapshot wrapper. The obsolete pure
operation expected loop/block/role fields and unreachable `PreheaderSeed`
entry branch remain as a later behavior-neutral Loop T0 cleanup candidate;
current segment receipt/target validation remains. This cleanup is outside the
landed G0 I1 implementation; its EXE acceptance is still open.

## Scope boundary

Latest dependency check (2026-09-12): source-declared parameter and return
metadata, canonical carriers and full Rust lifecycle capability now pass.
G0 source/mutation2, capability4 and retained physical ABI/JSON10 pass (16 total).
The unchanged ignored EXE witness runs under LLVM18 and now rejects at
`published-lifecycle-physical-json/instruction-unsupported`, before object/link.
The temporary runtime archive alias was removed. This supersedes the earlier
site-missing and parameter-count observations; exit3 remains open.

This row proves only source-backed G0 helper reach through the existing
ordinary Global Call. It does not widen G0 to cataloged methods, all-family
Loop selection, backend parity, real-app coverage, Call/R7 completion, or
legacy retirement.
