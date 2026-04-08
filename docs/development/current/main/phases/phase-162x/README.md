# Phase 162x: vm fallback lane separation cleanup

- Status: Landed
- 目的: optimization の前に `vm fallback` の owner split を固定し、runner compat fallback / kernel Rust fallback / `vm-hako` reference lane を別 surface として読めるようにする。
- 対象:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/design/vm-fallback-lane-separation-ssot.md`
  - `src/config/env/vm_backend_flags.rs`
  - `src/runner/route_orchestrator.rs`
  - `src/runner/keep/vm_fallback.rs`
  - `crates/nyash_kernel/src/hako_forward_bridge.rs`

## Decision Lock

- this is a cleanup wave, not a removal wave
- behavior stays the same
- current implementation lane is now `phase-163x`
- `phase-137x` remains the sibling string guardrail lane

## Fixed Owner Split

1. runner compat fallback
   - explicit `vm-compat-fallback` lane
   - owner: route orchestrator + runner keep path
2. kernel Rust fallback
   - `.hako` hook miss policy inside the Rust microkernel
   - owner: `hako_forward_bridge.rs` and hookable kernel callers
3. `vm-hako` reference lane
   - explicit reference/conformance lane
   - not a fallback lane

## Non-Goals

- removing `vm-compat-fallback`
- promoting `vm-hako`
- changing `NYASH_VM_USE_FALLBACK` behavior
- reopening `substring_hii` leaf tuning inside this cleanup wave

## Exit Criteria

1. docs/current pointers no longer blur the three surfaces
2. code comments/names match that split
3. current pointers can move to `phase-163x` without blurring fallback ownership
