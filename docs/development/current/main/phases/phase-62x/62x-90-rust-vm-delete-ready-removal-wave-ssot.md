---
Status: SSOT
Date: 2026-04-04
Scope: execute a narrow removal wave only if delete-ready rust-vm surfaces are proven by phase-61x facts.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-61x/61x-90-residual-rust-vm-caller-zero-audit-rerun-ssot.md
---

# 62x-90 Rust-VM Delete-Ready Removal Wave SSOT

## Intent

- removal is allowed only for caller-zero or explicitly replaced rust-vm surfaces
- if no such surfaces exist, `62x` records a no-op proof instead of forcing deletion
- `vm-hako` remains out of scope as reference/conformance keep

## Entry Assumption

- inherited from `61x`:
  - `delete-ready`: none
  - `keep-now`: `vm.rs`, `vm_fallback.rs`, `stage_a_compat_bridge.rs`, `core.hako`, `run_stageb_compiler_vm.sh`, `dispatch.rs`, `route_orchestrator.rs`

## Candidate Confirmation

- `62xA1` rerun confirms:
  - `src/runner/modes/vm.rs` still called by `route_orchestrator`
  - `src/runner/modes/vm_fallback.rs` still called by `route_orchestrator`
  - `src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs` still called by `stage_a_route`
  - `lang/src/runner/stage1_cli/core.hako` still owns raw compat callers
  - `tools/selfhost/run_stageb_compiler_vm.sh` still has proof callers
- result:
  - `delete-ready`: none

## Removal / No-Op Decision

- `62xA2` decision: no-op
- rationale:
  - forcing deletion here would violate the `61x` source-backed audit result
  - `62x` therefore serves as proof that the current removal wave is empty, not as a forced delete lane

## Boundary

- do not delete broad rust-vm core just to satisfy the phase title
- delete only if source-backed caller-zero facts change inside `62x`
- if the shortlist stays empty, close `62x` as a no-op removal wave and hand off to `63x`
