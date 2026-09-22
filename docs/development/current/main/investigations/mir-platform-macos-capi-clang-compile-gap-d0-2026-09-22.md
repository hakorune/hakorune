---
Status: design_stop__owner_audit__2026-09-22__ClangNestedFunctionBoundary
Task: MIR-PLATFORM-MACOS-CAPI-CLANG-D0
Date: 2026-09-22
Priority: owner audit only — macOS CAPI compilation stopped before lifetime execution
Parent: mir-platform-macos-capability-d0-2026-09-15.md
NextCard: per-owner bounded source row after compiler-boundary decision
Implementation permission: false; no compiler substitution, fallback, or C rewrite is authorized by this card
---

# macOS CAPI clang compile-gap D0

## Six-line brief

Decision: the macOS CAPI surface is blocked at a compiler portability
boundary. The hosted `macos-26-arm64` runner uses Apple clang, while the
published C shim currently relies on GNU C nested functions.

Source authority + canonical issuer: the single compiled owner
`lang/c-abi/shims/hako_llvmc_ffi.c` and its function-body include graph,
especially `hako_llvmc_ffi_pure_compile.inc` and
`hako_llvmc_ffi_generic_pure_lowering_state.inc`. The build script is only
the invocation owner; it does not define C semantics.

Non-authority: a test-only compiler probe, a different compiler installed in
CI, a platform fallback, a skipped CAPI test, or a macOS-only semantic shim.

Fail-fast boundary: a compiler that cannot compile the selected C owner must
remain a named `BackendCapabilityMissing` result before dylib loading or
temporary-input claims.

Smallest next slice: enumerate the included nested-function definitions and
their captured state, then decide whether the existing C owner has a bounded
file-scope translation-unit seam. Do not begin a broad rewrite from this D0.

Non-claims: no CAPI lifetime evidence on macOS, no x86_64/older-macOS claim,
no compiler installation policy, no production switch, and no fallback.

## Evidence from the native census

Run `35681768498` at SHA `d4168473d5b57178068d9574a48fd9758bbb9328`
ran on hosted `macos-26-arm64`. Provider lifetime passed. The CAPI test
invoked `tools/build_hako_llvmc_ffi.sh`, reached clang compilation, and
failed before `dlopen` or the temporary-input path. The first diagnostics
were `function definition is not allowed here` at
`hako_llvmc_ffi_generic_pure_lowering_state.inc:103`, followed by the same
form in `hako_llvmc_ffi_pure_compile.inc` and
`hako_llvmc_ffi_boxed_sum_abi_plan.inc`.

The existing Windows proof names this boundary explicitly: its compiler
probe selects a compiler that accepts GNU C nested functions and comments
that native MSVC-targeting clang rejects the extension
(`tools/checks/windows_harness_c_export_smoke.sh`). That is corroborating
static evidence, not macOS execution evidence.

## Boundary inventory

Includes: the C compiler -> `hako_llvmc_ffi.c` -> pure compile include graph
and the first compiler rejection. Excludes: dylib naming/loading,
LLVM TargetMachine, child process behavior, temporary-file ownership,
Windows changes, and MirBuilder semantics.

The CAPI integration test currently constructs
`target/release/libhako_llvmc_ffi.so` in its Unix test body, while the build
script emits `.dylib` on Darwin. That downstream path is unmeasured because
clang fails first; it is recorded as a later bounded row, not folded into
this compiler-boundary decision.

## Task order

1. Build a finite inventory of nested helper definitions reachable from the
   pure compile function body and group each by captured state and caller.
2. Check the existing C owner/module README for a file-scope helper seam that
   preserves one compiled owner and the current state authority.
3. If no bounded seam exists, record `NoSafeSlice` with the exact missing
   owner; do not install GCC, add a Darwin-only flag, or hide the CAPI test.
4. If a bounded seam exists, write its source-to-owner mapping and negative
   compiler proof before opening an implementation row.
5. Only after a clang-compatible owner is selected, open the separate Darwin
   dylib-name/loading row and rerun the focused CAPI lifetime test.

## Acceptance and retirement boundary

- The owner audit names the full included nested-function inventory and one
  canonical C owner or an explicit `NoSafeSlice`.
- No new compiler, fallback, or semantic receipt is introduced here.
- The macOS CAPI lifetime remains unproven until a later native run reaches
  the test body.
- No old edge is deleted by this card.
