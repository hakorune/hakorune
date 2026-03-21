---
Status: Active
Decision: provisional
Date: 2026-03-21
Scope: backend-zero portability slice to centralize FFI library candidate resolution in `src/host_providers/llvm_codegen.rs`, keep `transport.rs` thin, and restore `tools/checks/dev_gate.sh portability` to green without changing the daily backend route.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/design/de-rust-backend-zero-fixed-order-and-buildability-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - tools/checks/macos_portability_guard.sh
  - tools/checks/dev_gate.sh
  - src/host_providers/llvm_codegen.rs
  - src/host_providers/llvm_codegen/transport.rs
  - tools/build_hako_llvmc_ffi.sh
---

# P6: macOS Portability FFI Candidate Resolution Lock

## Goal

- `llvm_codegen.rs` owns the FFI library candidate resolution contract.
- `transport.rs` consumes that helper instead of inventing its own candidate list.
- macOS support stays explicit via `.dylib` candidate coverage.

## Non-Goals

- change the backend-zero daily owner order
- reopen p7/p8 smoke split work
- add a new ABI surface
- relax the portability guard into a noop

## Fixed Order

1. Move FFI candidate resolution ownership into `src/host_providers/llvm_codegen.rs`.
2. Make `src/host_providers/llvm_codegen/transport.rs` call the shared helper.
3. Verify `tools/checks/macos_portability_guard.sh` and `tools/checks/dev_gate.sh portability`.
4. Return to the active smoke split lane only after the portability guard is green.

## Acceptance

1. `bash tools/checks/macos_portability_guard.sh`
2. `bash tools/checks/dev_gate.sh portability`
3. `git diff --check`

## Next

1. Resume `phase29cc_wsm/p8` only after the portability slice is green.
