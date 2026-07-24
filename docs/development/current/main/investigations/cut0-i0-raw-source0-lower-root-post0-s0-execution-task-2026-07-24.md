# RAW-SOURCE0 LOWER ROOT — POST0-S0 execution task

Status: **Active implementation row — POST-CARRIER-prime-r1**
Date: 2026-07-24
Decision source: `cut0-i0-raw-source0-lower-root-post0-drained-carrier-consultation-2026-07-24.md`

## Decision lock

```text
RawFinalizedInvocationV1
  -> prepare_postprocess(self)
  -> RawPostprocessReadyInvocationV1::{Script, App}

ModulePostprocessOwnerV1::run_raw_ready(self, ready)
  -> RawPostprocessedInvocationV1::{Script, App}
  -> RejectedRawPostprocessInvocationV1
```

`RawFinalizedInvocationV1::prepare_postprocess(self)` is the only FINAL0
consumer. The existing `ModulePostprocessOwnerV1` remains the only stage
execution owner. The old `run_raw(RawFinalizedModuleInvocationV1)` path stays
legacy/disconnected and receives no new caller.

## S0a — carrier handoff

Add one consuming `RawFinalizedPhysicalV1::begin_postprocess(self)` terminal.
It converts the opaque finalized module into a private, non-Clone
`RawPostprocessModuleLoanV1` and retains the Builder readiness product,
`RawDrainWitnessV1`, and FINAL0 parity. No bare `MirModule`, `DerefMut`,
`AsMut`, `into_module`, or caller mutation closure is exposed.

## S0b — route-ready owner

Consume Script/App FINAL0 owners into route-specific
`RawPostprocessReadyInvocationV1`. Continuation, runtime snapshot, module
name, pre-root completion, helper receipts, App callable-Main outcome, and
physical evidence remain paired. No route is inferred from module symbols.

## S0c — shared stage kernel

Keep the existing stage order in one private kernel used by canonical/legacy
paths and the new Raw loan. Raw retains the existing schedule:

```text
RC = Run
verification = ReportPreTransformOnly
```

Pre-transform verifier errors remain reportable evidence. Raw final-verifier
rejection remains zero. Currently infallible RC/refresh/canonicalization
failure retention is not claimed.

## S0d — evidence and failure

Success retains the complete Raw route evidence, `RawDrainWitnessV1`, FINAL0
parity, POST0 parity, schedule, and Raw verification evidence until the later
external-commit preparation. Failure retains the mutated-but-unpublished loan
at the exact stage with typed cause and exposes inspection plus discard only.

## Acceptance

```text
new RawFinalizedInvocationV1 POST0 consumer = 1
old RawFinalizedModuleInvocationV1 new consumer = 0
stage-order authority = 1
RawPostprocessModuleLoanV1 mutable/bare-module escape = 0
source/catalog/current_module re-observation = 0
RawDirect route evidence retained = 1
Raw FinalVerification rejection producer = 0
retry/resume/fallback/rollback/catch_unwind = 0
external commit/public ingress/executor/CUT0 = 0
all modified/new source/check files < 800 lines
```

## Fixtures and gates

```text
empty Script
App callable-Main NotSelected
App callable-Main Selected
App with helpers
natural optimizer diagnostics
natural contract-refresh failure
reportable Raw pre-transform verifier Err
carrier parity drift with exact mutated owner retention
```

Required evidence:

```bash
RUSTFLAGS='-Awarnings' cargo check -q --lib
RUSTFLAGS='-Awarnings' cargo test -q raw_root_postprocess_p0 --lib -- --test-threads=1
python3 tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_post0_guard.py
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Non-claims

```text
external commit activation
public Raw ingress
AST-JSON / Program(JSON v0) behavior
old Raw-finalizer retirement
RC fault injection or typed panic retention
production executor and atomic CUT0
```
