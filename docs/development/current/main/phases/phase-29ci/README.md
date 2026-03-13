---
Status: Accepted (queued)
Decision: accepted
Date: 2026-03-13
Scope: `phase-29ch` closeout 後に `Program(JSON v0)` bootstrap boundary 自体を retire するための separate phase pointer。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/selfhost-compiler-structure-ssot.md
  - docs/development/current/main/phases/phase-29ch/README.md
  - docs/development/current/main/phases/phase-29ch/29ch-10-mir-direct-bootstrap-unification-checklist.md
  - docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md
  - docs/development/current/main/phases/phase-29ci/P1-FUTURE-RETIRE-BRIDGE-DELETE-ORDER.md
  - docs/development/current/main/phases/phase-29ci/P2-LIVE-CALLER-DELETE-ORDER.md
  - docs/development/current/main/phases/phase-29ci/P3-SHARED-SHELL-HELPER-AUDIT.md
  - src/stage1/program_json_v0/README.md
  - src/runner/stage1_bridge/README.md
---

# Phase 29ci: Program JSON v0 Bootstrap Boundary Retirement

## Goal

`phase-29ch` で `temporary bootstrap boundary` に縮退した

- `src/stage1/program_json_v0.rs` cluster
- `src/runner/stage1_bridge/**` future-retire lane

を、authority migration と混ぜずに separate phase として retire する。

この phase は `MIR-direct bootstrap unification` ではない。
`phase-29ch` が固定した authority を前提に、bootstrap-only JSON v0 boundary の caller / owner / delete order を扱う。

## Entry Conditions

1. `phase-29ch` の done judgment が green
   - reduced bootstrap proof can be explained without JSON v0 route authority
   - bridge is documented as `temporary bootstrap boundary` only
2. proof bundle is green on the current authority contract
   - Stage1/Stage2 rebuild
   - `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh`
   - `tools/selfhost_identity_check.sh --mode {smoke,full} --skip-build`
3. `Program(JSON v0)` retirement work is not mixed back into `phase-29ch`

## Fixed Order

1. inventory the remaining bootstrap-only JSON v0 consumers
2. choose one owner-local retirement slice at a time
3. keep proof bundle green after each retirement slice
4. only after caller inventory is empty, consider deleting the boundary itself

## P0 Inventory

- exact caller / owner matrix:
  - `docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md`
- future-retire bridge delete order:
  - `docs/development/current/main/phases/phase-29ci/P1-FUTURE-RETIRE-BRIDGE-DELETE-ORDER.md`
- live/bootstrap + shell caller delete order:
  - `docs/development/current/main/phases/phase-29ci/P2-LIVE-CALLER-DELETE-ORDER.md`
- shared shell helper audit:
  - `docs/development/current/main/phases/phase-29ci/P3-SHARED-SHELL-HELPER-AUDIT.md`
- current preferred first bucket:
  - Rust-owned `build surrogate keep`
  - then `future-retire bridge`
- retreat note:
  - compiled-stage1 build surrogate is not deletable yet, but it is now intended to shrink behind a single owner-local module; route registration, handler ownership, and build-box/launcher MIR handoff regression coverage moved there too, so retirement no longer needs shared route-table or root-test edits
  - future-retire bridge entry is also close to owner-local: direct emit-flag reads now stay inside `src/runner/stage1_bridge/**`, the entry facade lives in `program_json_entry/mod.rs`, request building/source-path precedence lives in `program_json_entry/request.rs`, exact success/error process-exit formatting lives in `program_json_entry/exit.rs`, and outer root-runner files remain only as thin caller contracts (`src/runner/mod.rs`, `src/runner/emit.rs`)
  - bridge-local Program(JSON v0) read->emit->write orchestration is now owner-local to `src/runner/stage1_bridge/program_json/pipeline.rs`, while read policy / payload emission / writeback stay in `read_input.rs` / `emit_payload.rs` / `writeback.rs`, so `program_json/mod.rs` keeps shrinking toward a pure facade
  - future-retire bridge delete order is now explicit: next Rust-only slices stay inside `program_json_entry/` and `program_json/`, while `src/runner/mod.rs` and `src/runner/emit.rs` are `must-stay thin callers`
  - outer caller audit is also explicit now: after Rust-only buckets, next exact buckets are `.hako` owner 4 files, shared shell helper 3 files, then test-only smoke tail 43 files
  - shared shell helper audit order is now explicit too: `tools/hakorune_emit_mir.sh` first, `tools/selfhost/selfhost_build.sh` second, and `tools/smokes/v2/lib/test_runner.sh` last because it bridges into the 43-file smoke tail
  - `tools/hakorune_emit_mir.sh` now keeps its provider-first Program→MIR delegate chain behind `emit_mir_json_from_program_json_delegate_chain()`, with `try_legacy_program_json_delegate()` isolating the old CLI fallback, so the next helper-local tail is the direct-emit fallback lane only
  - `MirBuilderBox.emit_from_source_v0(...)` remains a live keep and must not be demoted into the diagnostics/probe bucket
  - shell/helper delete order still has a wider test-only shell/apps tail beyond the three shared helper scripts; keep that caller audit separate from the first Rust-only delete slices

## Current Retirement Targets

- `src/stage1/program_json_v0.rs` cluster
- `src/runner/stage1_bridge/program_json/mod.rs`
- `src/runner/stage1_bridge/program_json/`
- `src/runner/stage1_bridge/program_json_entry/`
- `src/runner/mod.rs` bridge-entry caller contract
- `src/runner/stage1_bridge/**` future-retire bridge lane
- compiled-stage1 / shell callers that still need the bootstrap-only JSON v0 boundary

## Non-goals

- reopening `phase-29cg` solved reduction buckets
- re-arguing `phase-29ch` authority migration
- widening compat keep or raw direct `stage1-cli` authority

## Acceptance

- retirement work can be explained without mixing authority migration back into `phase-29ch`
- remaining JSON v0 consumers are inventoried with exact owners
- next delete/reduction slice can be chosen from this phase alone
