---
Status: closed__NoSafeSlice__2026-09-22__NoPhysicalLoweringContextOwner
Task: MIR-PLATFORM-MACOS-CAPI-CLANG-D0
Date: 2026-09-22
Priority: owner audit only — macOS CAPI compilation stopped before lifetime execution
Parent: mir-platform-macos-capability-d0-2026-09-15.md
NextCard: none__frontier_pause__no_physical_lowering_context_owner
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

## Static owner inventory — 2026-09-22

Starting at the 38 indented includes inside
`compile_doc_compat_pure`, the recursive include closure contains 110
`*.inc` files. A static signature scan finds 795 `auto` helper declarations
(777 distinct names). The largest state-coupled groups are
`hako_llvmc_ffi_same_module_method_views.inc` (52),
`hako_llvmc_ffi_lowering_plan_metadata.inc` (47),
`hako_llvmc_ffi_compiler_state.inc` (37),
`hako_llvmc_ffi_same_module_function_emit.inc` (30), and
`hako_llvmc_ffi_pure_compile_generic_lowering.inc` (28).

This inventory rules out treating the failure as one isolated helper or one
Darwin spelling switch. The canonical compiled owner is still the single
`hako_llvmc_ffi.c` translation unit; the existing include partitions are
state-coupled implementation slices, not independent ABI owners. A portable
file-scope seam therefore needs an explicit state/issuer design before any
mechanical rewrite is authorized.

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

## Independent owner audit — 2026-09-22

The read-only owner audit found no existing bounded file-scope seam. The
single `compile_doc_compat_pure` owner reaches 38 direct include roots and a
recursive closure of 110 `.inc` files with 795 `auto` helper declarations
(777 distinct names). The helpers are state-coupled rather than isolated
Darwin spellings: the generic lowering state declares a local
`lowering_state` and macro aliases, the compiler-state family consumes those
aliases, and the prescan `EMIT` macro captures a local `FILE *f`.

The captured set also includes invocation and function handles, blocks and
counts, route and definition metadata, LLVM/object/error paths, and mutable
cursors. `HakoLlvmcInvocation` owns documents, options, and ledgers but does
not own that complete lowering stack or its output streams. Moving one helper
family would therefore leave the remaining nested definitions invalid; making
the owner file-scope requires a new physical lowering-context owner and an
explicit state/issuer contract.

Decision: `NoSafeSlice` for this card. Keep the existing single C owner and
the named `BackendCapabilityMissing` boundary. Do not substitute compilers,
add Darwin-only flags, skip CAPI, add a fallback, or create a parallel
semantic/shim owner. A later row may reopen only after an accepted physical
lowering-context owner is designed and mapped across the finite helper
families.

## Task order

1. Build a finite inventory of nested helper definitions reachable from the
   pure compile function body and group each by captured state and caller.
2. Check the existing C owner/module README for a file-scope helper seam that
   preserves one compiled owner and the current state authority.
3. **Closed as `NoSafeSlice`:** no existing physical owner contains the
   complete captured lowering context. Do not install GCC, add a Darwin-only
   flag, or hide the CAPI test.
4. Reopen only after a physical lowering-context owner and its source-to-owner
   mapping are accepted, with a negative compiler proof before implementation.
5. Only after a clang-compatible owner is selected, open the separate Darwin
   dylib-name/loading row and rerun the focused CAPI lifetime test.

## Acceptance and retirement boundary

- The owner audit names the full included nested-function inventory and one
  canonical C owner or an explicit `NoSafeSlice`.
- The missing owner is explicit: a file-scope physical lowering-context owner
  for the complete captured state and its issuer contract.
- No new compiler, fallback, or semantic receipt is introduced here.
- The macOS CAPI lifetime remains unproven until a later native run reaches
  the test body.
- No old edge is deleted by this card.
