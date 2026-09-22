---
Status: design_stop__census_complete__2026-09-22__MacArmCapiCompileGap
Task: MIR-PLATFORM-MACOS-CAPABILITY-D0
Date: 2026-09-15
Priority: census only — classify the first macOS failures before any implementation; selected after the Map red was cleared and the semantic lane reached a frontier pause
Parent: mir-call-compatibility-retire-r7-d0-2026-09-11.md
NextCard: macos-capi-clang-compile-gap-d0 (design audit before any fix)
Implementation permission: false; reopen only after the census classifies every first failure and the Map lane's red is cleared or explicitly paused
---

# macOS platform capability D0

## Six-line brief

Decision: macOS is a supported target by owner decision
(`user_selected_include__2026-09-15`, paired with the Windows lifecycle
`deferred__user_selected_later__2026-09-14` record). The first row is a
census, not implementation: run the focused suite on `macos-latest` and
classify the first failure. Unknown gaps are not fixed blind.

Source authority + canonical issuer: the focused suite result on a real
`macos-latest` runner (or an equivalent native macOS host), recorded per
failing surface with run ID, SHA, and time. Each confirmed gap then names
its existing owner before any fix.

Non-authority: Linux-only receipts, the existing macOS build-only CI job,
plugin `cargo check` results, local macOS assumptions, simulator output,
and any "should work" reasoning.

Fail-fast boundary: a missing capability is classified
`BackendCapabilityMissing` — no fake dylib, no POSIX-only shim retry, no
compatibility fallback, no skipping the failing suite.

Smallest next slice: dispatch `portability-ci.yml` `scope=full` (which
already runs `macos-build` and the plugin-recovery matrix on
`macos-latest`), then run the focused suite surface-by-surface and record
the first failure per surface.

Non-claims: this D0 does not claim macOS test execution, lifecycle
execution, LLVM C API linking, provider loading, temporary-file behavior,
AOT/EXE output, or any fix.

## Verified current state (2026-09-15)

Already in-tree — these are scaffolding, not capability proof:

- `tools/build_hako_llvmc_ffi.sh` has a `Darwin` branch emitting
  `libhako_llvmc_ffi.dylib` with `-dynamiclib` and
  `-Wl,-install_name,@rpath/...` — written, but never executed on a real
  macOS host in recorded evidence.
- `portability-ci.yml` already has `macos-build` (release build only) and
  the plugin-recovery `cargo check` matrix on `macos-latest`, gated by
  `scope=full` dispatch or non-draft PR — **no test or suite execution
  exists on macOS today**.
- Provider package code already names `.dylib` for `macos`/`darwin`
  (`src/cli/provider_package_*_build/support.rs`, `*.rs:97/246`).
- The CI-tax rules Claude cited are already implemented: dispatch `scope`
  input, `dorny/paths-filter` gate for non-dispatch runs, and
  `cancel-in-progress` on pull_request only.

## Focused manual runner entry — 2026-09-22

`portability-ci.yml` now exposes a manual-only `scope=macos` job.  It runs
the provider and Unix CAPI lifetime tests sequentially on one
`macos-latest` runner with the quick profile and `CARGO_BUILD_JOBS=4`, so the
test binary can be reused and the Windows lanes are not started.  This is an
evidence entry only: a failure is recorded as a first-surface census result,
not repaired or skipped here.  The existing `scope=full` path remains the
separate build/plugin baseline and was not duplicated by this job.

## Census dispatch receipt — complete

Run `35681768498` at SHA `d4168473d5b57178068d9574a48fd9758bbb9328`
was dispatched with `scope=macos` at `2026-09-22T03:03:51Z`
(`12:03:51 JST`) and completed at `2026-09-22T03:20:16Z`
(`12:20:16 JST`) on the hosted `macos-26-arm64` image.

- Provider lifetime: **PASS**.  The focused test passed (`1 passed`) after
  the uncached quick-profile build (`15m44s`); the test body took `0.30s`.
- CAPI lifetime: **FAIL / BackendCapabilityMissing**.  The test reached the
  CAPI build and then clang rejected the generated C include graph before the
  lifetime assertion ran.  The first compiler diagnostics were
  `function definition is not allowed here` in
  `hako_llvmc_ffi_generic_pure_lowering_state.inc:103`, followed by the same
  nested-function form in `hako_llvmc_ffi_pure_compile.inc` and
  `hako_llvmc_ffi_boxed_sum_abi_plan.inc`.  The test ended with exit 101 after
  `8.62s` of test execution.

This is a macOS 26 arm64/clang capability result, not evidence for every
macOS version or x86_64 host.  It is also not a temporary-input lifetime
result: the CAPI compiler failed before that path could run.  No code or
fallback change is authorized by this census.

The follow-up owner-audit row is
[`MIR-PLATFORM-MACOS-CAPI-CLANG-D0`](mir-platform-macos-capi-clang-compile-gap-d0-2026-09-22.md).
The integration test's later Darwin library-name path is intentionally kept
separate: it cannot be classified until the clang compile boundary is passed.

## Census boundary

このcensusが覆う境界: `macos-latest` runner checkout -> focused suite
terminal; includes Rust build/test, the hako_llvmc_ffi build script,
dylib/provider loading, temporary-file paths, and LLVM C API linking;
excludes Windows lane changes, MirBuilder semantic work, performance
measurement, and any fix.

## Suspect surfaces to classify (order by likely first failure)

1. `cargo test` focused suite on `macos-latest` — does the Rust test
   binary itself run green?
2. `tools/build_hako_llvmc_ffi.sh` on macOS — `cc` is clang; `flock` is
   absent (script already degrades: `command -v flock` guard exists, but
   the unlocked path is unproven).
3. dylib loading — `dlopen` of `libhako_llvmc_ffi.dylib`, `@rpath`
   resolution, SIP/unsigned-library loader behavior on the runner.
4. LLVM C API linkage — whether the lifecycle/provider path can link or
   load LLVM on macOS at all (compare: Windows D0's loader inventory).
5. Temporary-file ownership — `mkstemp`/`rename` semantics are POSIX on
   macOS but sandbox/TMPDIR behavior may differ.

## Task order (after the Map lane frees this row)

1. Keep the focused manual job as the reusable macOS census entry; do not
   dispatch another run until the CAPI compile-gap owner is selected.
2. Continue in the design-only owner-audit row
   `MIR-PLATFORM-MACOS-CAPI-CLANG-D0`.  Its first decision is whether an
   existing C owner can provide a portable translation unit; nested-function
   syntax must not be papered over with a platform fallback or a silent skip.
3. Each confirmed gap becomes a bounded row following the Windows
   capability-D0 type: `BackendCapabilityMissing` until the per-capability
   owner is named and proven; fake fallback prohibited.
4. Re-run the focused CAPI lifetime test only after that row names an owner
   and a clang-compatible source boundary.  Until then, the macOS tier is
   provider-tested / CAPI-unverified.

## Replacement and retirement boundary

Nothing is replaced or retired by this D0. Any follow-up row may only
extend the existing `Darwin`/`macos` branches or add owner-named macOS
surfaces; Linux and Windows behavior must remain unchanged, and no old
edge is deleted until the macOS owner and evidence exist.

## Acceptance

- census receipt names run ID, SHA, and time per surface;
- every first failure is classified to a named owner surface or recorded
  as `BackendCapabilityMissing` with an observable reopen trigger;
- no implementation, fallback, or silent skip was introduced;
- CI changes, if any, obey the existing scope/paths/cancel rules.

## D0 disposition

Owner decision recorded 2026-09-15: macOS is a supported target. The native
census is complete on `macos-26-arm64`: provider lifetime passed, while the
CAPI surface stopped at the clang compile boundary and is classified
`BackendCapabilityMissing`. The card remains design-only and does not
authorize compiler changes, fallback, or backend parity. Windows lifecycle
remains `deferred__user_selected_later__2026-09-14` — unchanged.
