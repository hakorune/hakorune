# MIRBUILDER-HAKO-MIMALLOC-PROMOTION-GATE0

Status: `Parked; required after the first production cutover`
Date: `2026-08-10`
Parent: `docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md`
Related:
  - `docs/development/current/main/workstreams/mimalloc-current.md`
  - `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`
  - `docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md`

## Purpose

Before enabling the `.hako` selfhost MirBuilder/parser migration, build the
pinned `.hako` mimalloc workload through the new MIRBuilder and perform a
compiler-promotion check. This is a bounded compiler readiness gate, not
provider activation, process-allocator replacement, or a new allocator
lowering lane.

This task is deliberately parked until:

```text
MIRBUILDER-FIRST-PRODUCTION-CUTOVER
```

is the milestone closed by the Loop production cutover
`M10b-I0-R0` (`6e88441c0b`, receipt:
`docs/development/current/main/investigations/joinir-loop-portable-recipe-cutover0-m10b-i0-r0-2026-09-24.md`),
not a second switch task. That row had a named production caller, deleted the
selected old route, and left retry/fallback at zero. The gate still stays
parked behind unified resume order gates 1–2 (Call/R7 residual and the
language-v1 conformance matrix); the milestone name alone does not unblock it.
It must close before `HAKO-CALLABLE-RESULT-ISSUER-CUTOVER-I0` or any broader
`.hako` selfhost producer activation.

## Execution contract

1. Pin the exact `.hako` mimalloc entrypoint and matched workload from the
   existing mimalloc workstream. Do not shrink the workload after seeing a
   failure.
2. Build it through the canonical new-MIRBuilder ingress with legacy retry,
   profile reselection, and fallback disabled.
3. Run allocator correctness and lifecycle smokes before performance runs.
4. Compare three distinct artifacts: the post-cutover `.hako` executable,
   the accepted pre-cutover compiler artifact, and the matched C mimalloc
   reference. The C comparison is not a substitute for the compiler
   regression baseline.
5. Use the owner-first perf method. Record repeated timing distribution,
   peak memory, generated MIR, and hot assembly for the exact measured front.
   Whole-program timing alone is not sufficient evidence.
6. If the gate fails, return to the first failing compiler owner. Do not add a
   mimalloc-specific lowering branch, `.hako` workaround, legacy route, or
   silent fallback.

## Admission criteria

```text
canonical new MirBuilder selected                         = yes
legacy / retry / fallback observed                       = 0
allocator correctness and lifecycle gates                = green
active exact-front performance regression                = none unexplained
generated MIR / assembly regression                      = none unexplained
evidence recorded in the existing mimalloc benchmark SSOT = yes
```

The result permits resuming the `.hako` selfhost migration. It does not claim
whole-program selfhosting, provider selection, global allocator installation,
or final C-mimalloc performance parity.

## Required evidence

```text
source revision and pinned entrypoint
new-MIRBuilder build command and mode
backend / target / runtime configuration
correctness and lifecycle gate results
pre-cutover compiler baseline
matched C mimalloc reference
repeated timing and peak-memory observations
MIR / assembly owner finding
legacy/retry/fallback trace = zero
```

The closeout belongs in the existing mimalloc benchmark ledger and the
selfhost migration checklist. No new optimizer or provider authority is
created by this card.
