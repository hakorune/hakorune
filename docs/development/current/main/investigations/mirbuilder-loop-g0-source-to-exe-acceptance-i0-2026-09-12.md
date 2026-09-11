Status: closed__Implementation__GenericG0SourceToExeAcceptance__2026-09-12
Task: LOOP-G0-SOURCE-TO-EXE-ACCEPTANCE-I0
Date: 2026-09-12
Priority: prove the existing normal-package Generic G0 publication reaches the existing EXE emitter
Parent: mirbuilder-loop-g0-source-to-exe-publication-d0-2026-09-11.md
NextCard: none__GenericG0SourceToExeAcceptance__PendingCloseout
---

# Generic G0 source-to-EXE acceptance I0

## Six-line brief

```text
Decision: extend the existing normal-package loop test with one opt-in source-to-EXE acceptance witness; do not add a source fixture, semantic receipt, or Call route.
Source authority + canonical issuer: VerifiedFinalCallableProgramSourceV1 -> NormalRootExecutionConsumerV1 -> VerifiedNormalCallableSemanticPackageV1 -> existing Generic G0 function consumer and Single publication lifecycle.
Non-authority: package.source_ast() re-resolution, route_loop, MIR/backend metadata, test-only emitter sessions, executable runtime result of the helper body, fallback, retry, and a second issuer.
Fail-fast boundary: selected package key, source owner/forest/header, G0 policy mode, published verification, and existing EXE emitter admission must fail before a partial artifact is retained.
Smallest next slice: reuse the existing in-test source program, assert the published Generic G0 function remains present, and invoke the existing emit_published_view_exe path when its explicit runtime archive is available.
Non-claims: no G0 helper runtime result, Main-to-G0 Call support, new fixture, backend parity, real-app improvement, all-family Loop selection, legacy retirement, or whole-MIRBuilder completion.
```

## Bounded census

The boundary is the already accepted normal source package through the existing
published-MIR EXE emitter. It includes the source-backed top-level Generic G0
function, the ordinary `Main.main/0` executable root, publication callback, and
the existing `emit_published_view_exe` admission. It excludes Call resolution,
new source files under `apps/`, new semantic products, backend route changes,
and runtime execution of the non-root G0 helper.

The exact source is the test string plus the existing
`apps/typed-object-birth-min/main.hako` fixture in
`src/mir/compiler/normal_default_pipeline_loop_tests.rs`. Its existing Pair
lifecycle `Main.main/0` root exits with 30 and does not call `generic_g0`.
Therefore successful EXE execution proves the root/publication boundary only;
it is not evidence that the helper body was executed.

## Authority and failure order

The normal package remains the only source-backed function loan. The test may
observe the published module and pass it to the existing backend emitter, but it
must not reconstruct a source unit or select a route from the emitted MIR. The
emitter owns only backend admission and artifact construction; it does not issue
Generic G0 meaning.

Missing runtime archive, `ny-llvmc`, selected FFI, or LLVM18 tools is an explicit
environment boundary. The acceptance witness is opt-in/ignored under that
condition rather than weakening the normal focused Rust evidence or adding a
fallback.

## Required implementation

1. Reuse the existing source-backed helper and `typed-object-birth-min` root
   fixture without adding a new source fixture or Call edge.
2. Add one ignored EXE acceptance test beside the existing loop consumer test.
3. Assert the callback runs once, verification succeeds, `generic_g0/2` is
   published, and the existing EXE emitter owns the output when the runtime
   archive is present.
4. Remove the temporary executable on both success and error paths.

No new production function, route, receipt, fixture, or source authority is
allowed in this row. Any attempt to make Main call the helper reopens Call/R7
and is outside this card.

## Acceptance evidence

- Positive Rust path: existing `generic_g0` focused test remains green.
- Positive EXE path: the ignored acceptance witness reaches the existing emitter
  and executes `Main.main/0` with exit code 30 when the explicit runtime and
  LLVM18 toolchain are available. On this host it passed as an environment-gated
  skip because `llvm-config-18`, `llc-18`, and `opt-18` are unavailable.
- Guard: no source AST re-resolution, route-loop entry, test-only emitter
  session, fallback, or helper-body runtime claim is added.
- Environment: the lifecycle archive and FFI were prepared locally; the missing
  LLVM18 tools made the witness environment-unavailable. This is not a compiler
  regression claim.

## Closeout

Closeout evidence: the focused `generic_g0` test passed; the ignored witness
passed with the explicit environment-unavailable skip; the Generic G0 source
guard, physical-transfer guard, current-state/pointer guard, and
`git diff --check` passed. The witness command was
`CARGO_BUILD_JOBS=1 cargo test --profile quick -j1
normal_package_generic_g0_reaches_existing_exe_emitter -- --ignored --nocapture`.
The synchronized card, test, SSOT mirrors, guard, and `CURRENT_STATE.toml` are
ready for commit and push.
