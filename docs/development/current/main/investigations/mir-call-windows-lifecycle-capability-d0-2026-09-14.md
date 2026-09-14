---
Status: accepted__design_stop__2026-09-14
Task: MIR-CALL-WINDOWS-LIFECYCLE-CAPABILITY-D0
Date: 2026-09-14
Priority: define the native Windows lifecycle capability boundary before any implementation
Parent: mir-call-compatibility-retire-r7-d0-2026-09-11.md
NextCard: MIR-CALL-WINDOWS-LIFECYCLE-OWNERSHIP-I0
Implementation permission: false until the target, loader, ABI, and temporary-file ownership matrix is accepted
---

# Windows lifecycle capability D0

## Six-line brief

Decision: native Windows lifecycle remains `BackendCapabilityMissing` until a
target-built runtime, loader, ABI, and V4 temporary-file path are all proven.
Linux lifecycle remains unchanged, and the existing named Windows terminal is
kept. No fake DLL, retry, or compatibility fallback is introduced.

Source authority + canonical issuer: the target-built lifecycle runtime
artifact and its ABI descriptor/entry record are admitted by
`LifecycleRuntimeSessionV1::select`; the C lifecycle owner issues the native
session, LLVM emission, and published object result through the existing
`PublishedLifecyclePhysicalAbiInputV1`.

Non-authority: generic provider/CAPI Windows receipts, a Linux archive, a fake
DLL, a hand-made session, name/arity/path text, environment flags, and
TempDir-only tests.

Fail-fast boundary: reject a missing or wrong target, archive, descriptor,
ABI, loader, symbol, session, or temporary-file capability before child
processes, IR mutation, or object publication. Preserve the existing named
Windows platform/session terminals and do not retry through another backend.

Smallest next slice: inventory the `x86_64-pc-windows-msvc` `.lib`/COFF
descriptor, LLVM DLL loader, target session, and temporary input/output
ownership; then define one owner-local I0 seam without changing Rust semantic
selection.

Non-claims: this D0 does not claim Windows lifecycle execution, LLVM 18 DLL
compatibility, mixed-CRT allocation safety, native close/reopen, provider/CAPI
equivalence, backend parity, Call production changes, or R7 schema deletion.

## Current authority and evidence

Rust `LifecycleRuntimeSessionV1::select` currently reads an ELF archive and
accepts only `x86_64-unknown-linux-gnu` (`src/host_providers/llvm_codegen/runtime_abi_descriptor.rs`).
The descriptor reader has no Windows `.lib`/COFF path. The C lifecycle session
open and V4 compile owner return named platform terminals under `_WIN32`
(`lang/c-abi/shims/published_mir/hako_llvmc_ffi_lifecycle_target_session_v1.inc`,
`hako_llvmc_ffi_lifecycle_v4_compile.inc`). The existing V4 path also owns
POSIX `mkstemp`, `close`, `unlink`, and `rename` operations.

The native Windows provider and generic CAPI temporary-input close/reopen
receipts are separate evidence. They do not issue or prove the lifecycle
runtime artifact, LLVM session symbols, V4 publication, or lifecycle
close/reopen. The Linux lifecycle execution test is likewise limited to the
POSIX archive and file path.

## Finite capability state table

| State | Canonical owner | Terminal or result |
| --- | --- | --- |
| lifecycle not selected | `PublishedMirBackendView` | ordinary route; outside this D0 |
| runtime directory absent | `published_mir_object.rs` | explicit runtime-directory error |
| archive absent or unreadable | `LifecycleRuntimeSessionV1::select` | archive-read error |
| descriptor absent, duplicated, or malformed | target artifact reader | descriptor rejection |
| entry ABI absent, duplicated, or malformed | target artifact reader | entry-ABI rejection |
| target triple mismatched | Rust session selector | `BackendCapabilityMissing` |
| LLVM DLL absent | native session loader | lifecycle-session library terminal |
| required symbol absent | `LoadLibraryA`/`GetProcAddress` owner | lifecycle-session symbol terminal |
| ABI or layout mismatched | session admission | runtime/layout terminal |
| JSON/V2/V4 admission fails | V4 parser/admission | named pre-artifact rejection |
| temporary `.ll`/`.obj` open, close, or move fails | V4 temporary owner | cleanup, then failure |
| LLVM emit or publish fails | C lifecycle owner | cleanup, then failure |
| native success | V4 owner | object publish, session close, temporary cleanup |

## D0 task order

1. Inventory the target triple, descriptor format, archive/container owner, and
   COFF inspection tools for `x86_64-pc-windows-msvc`.
2. Specify a Windows dynamic-loader owner using `LoadLibraryA` and
   `GetProcAddress`, with explicit session disposal. Do not copy the POSIX
   `dlopen`/`dlsym` path or hide missing symbols behind a fallback.
3. Specify Windows temporary lifecycle operations (`_open`/`_close`/
   `_unlink` plus an atomic publish operation such as `MoveFileEx`, or an
   equivalent owner) while preserving the current cleanup and no-gap
   publication contract.
4. Define focused native Windows direct-C and Rust-wrapper tests for positive
   V4 publication, target/symbol/temp failures, cleanup, and close/reopen. A
   native Windows runner and the target-built runtime are required for this
   evidence; CI dispatch happens only after the code slice is accepted.
5. Update the owner README/reference and pointer with the accepted I0 boundary
   and the resulting Windows receipt. Keep each implementation owner below
   the 760-line split trigger and 800-line hard stop.

## Replacement and retirement boundary

The future I0 may replace only the `_WIN32` lifecycle platform/session
terminals, the Linux-only target predicate, the ELF-only artifact admission
branch, and the POSIX-only V4 temporary operations once equivalent Windows
owners and evidence exist. Linux lifecycle behavior, `BackendInputJsonFile`,
generic provider/CAPI evidence, the existing physical ABI, and semantic
receipts remain. No old edge is deleted until the Windows owner, result, and
cleanup terminals are selected and observed.

## Acceptance

- target, archive/COFF descriptor, loader, symbol, ABI, session, and temporary
  ownership authorities are named with a finite state table;
- positive native Windows lifecycle V4 evidence publishes an object and proves
  close/reopen plus returned-input cleanup;
- negative evidence covers target mismatch, missing symbols, ABI/layout
  mismatch, and temporary open/close/publish failures without partial output;
- Linux lifecycle and existing provider/CAPI paths remain unchanged;
- source-size, pointer, focused guard, and Windows CI receipt (run, SHA, and
  time) are recorded before implementation closeout;
- no generic fallback, synthetic authority, new semantic receipt, or R7 schema
  deletion is used to bridge the capability gap.

## D0 disposition

This is a capability design stop, not a current Rust implementation failure.
The next bounded card is
`MIR-CALL-WINDOWS-LIFECYCLE-OWNERSHIP-I0`; it may be selected only after the
target/loader/ABI/temporary-owner matrix above is accepted and the required
Windows toolchain inputs are available. Until then, retain
`BackendCapabilityMissing` and continue with another inventoried family rather
than weakening the lifecycle boundary.

