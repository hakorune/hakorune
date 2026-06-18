Status: SSOT
Date: 2026-06-15
Scope: current lane / blocker / next pointer only.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/investigations/docs-pointer-inventory-2026-06-09.md
  - docs/development/current/main/design/current-docs-update-policy-ssot.md

# Self Current Task - Now (main)

## Current

- current-state SSOT: `docs/development/current/main/CURRENT_STATE.toml`
- active task card: read `latest_card_path` in `CURRENT_STATE.toml`
- compiler foundation checkpoint: `docs/development/current/main/phases/phase-293x/293x-1040-COMPILER-FOUNDATION-CHECKPOINT-001.md`
- compiler foundation taskboard: `docs/development/current/main/workstreams/compiler-foundation-current.md`
- BoxCallable registry SSOT: `docs/development/current/main/design/box-callable-registry-ssot.md`
- TypeAbiCatalog planning spine SSOT: `docs/development/current/main/design/type-abi-catalog-planning-spine-ssot.md`
- CorePlan migration roadmap SSOT: `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
- compiler expressivity policy: `docs/development/current/main/design/compiler-expressivity-first-policy.md`
- active lane: read `active_lane` in `CURRENT_STATE.toml`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- phase status: read `phase_status` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- blocker token: read `current_blocker_token` in `CURRENT_STATE.toml`
- implementation gaps: none; read `active_lane_status` in `CURRENT_STATE.toml`

## Next

- continue the active phase from `current_blocker_token`, `phase_status`, and
  `latest_card_path` in `CURRENT_STATE.toml`
- current day-to-day tasks live in `latest_workstream_card` from
  `CURRENT_STATE.toml`
- compiler foundation is paused at `COMPILER-FOUNDATION-CHECKPOINT-001`
- exact-front optimization is paused by
  `EXACT-AOT-FASTPATH-PAUSE-CHECKPOINT-001`
- VM product-route app validation is retired by
  `VM-ACTIVE-LANE-RETIRE-001`; EXE/AOT is the primary route for app/selfhost
  validation
- compiler construction now includes build-time reduction planning from
  `BUILD-CRATE-SPLIT-PLAN-001`; the first `mir_core` growth slice moved
  control-flow ID newtypes into `hakorune-mir-core`; the first
  `hakorune-mir-plans` split moved `object_storage_plan` behind a compatibility
  facade; the first cold release build baseline is recorded; the second
  passive split moved `aggregate_storage_plan`; the third split moved
  `map_repr_plan` pure data while leaving refresh logic in the main crate; the
  fourth split moved `local_fastpath_fact` pure aggregation while leaving
  `MirFunction` metadata assignment in the main crate; the fifth split moved
  `TypedObjectFieldStorage` while preserving the existing
  `crate::mir::function` import path; the sixth split moved record-layout /
  ArrayRecord / PackedArray passive metadata rows while leaving producer logic
  in the main crate; the seventh split moved typed-object / direct-state /
  record-state passive rows while leaving declaration inventory and producers
  in the main crate; the eighth split moved loop/range/direct-array/span
  function fact vocabulary while leaving producers and refresh logic in the
  main crate; Stage 1 is now closed. The post-stage1 cold build measured
  real=158.95s, so this was structural rather than a build-time winner. The
  backend preflight rejected wholesale `src/backend` split and selected
  `runner/mir_json_emit` as the next boundary. MIR JSON emitter preflight then
  rejected direct extraction because it still has 372 direct `crate::mir`
  references. The MIR JSON emit boundary now keeps projection in the main crate
  and reserves future crate extraction for serialization only. The export model
  now has passive root and function summary wiring. The export-model seam is
  closed, direct `mir_json_emit` extraction is still blocked by direct MIR
  references, and the DTO boundary now has passive JSON-ready construction. The
  optional AOT passive split is closed. The default-compiled `mir_interpreter`
  surface was audited at 12,944 lines across 66 files; VMValue / VMError are
  live outside the interpreter, so immediate deletion/gating is rejected. The
  next row selected a default-on `vm-reference` feature ladder. The scaffold is
  now in place: VMValue / VMError stay always available, while mir_interpreter
  and backend VM aliases are feature-gated. Default-off is not claimed yet. The
  runner callers are now classified. Explicit VM, REPL, keep/vm, and JoinIR VM
  bridge remain `vm-reference` routes, while product and bridge paths still
  share `execute_mir_module_quiet_exit` as a VM terminal. The next blocker is
  `BUILD-VM-TERMINAL-EXECUTION-ROUTE-DESIGN-001`
- current manual entry points now route through current record/box,
  concurrency/thread, and object-storage SSOTs instead of stale historical
  Box-only or thread-spawn readings
- keep allocator-provider activation, hooks, host allocator replacement, and `#[global_allocator]` out of scope
- use the active method anchor from `CURRENT_STATE.toml` instead of stale
  historical lane notes

## Rules

- keep BoxShape and BoxCount separate
- do not grow the restart mirrors with landed history
- update `CURRENT_STATE.toml` and the active card first

## Read Next

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/phases/phase-296x/296x-1127-BUILD-VM-RUNNER-CALLER-CLASSIFICATION-001.md`
3. `docs/development/current/main/phases/phase-296x/296x-1126-BUILD-VM-REFERENCE-FEATURE-SCAFFOLD-001.md`
4. `docs/development/current/main/phases/phase-296x/296x-1125-BUILD-VM-MIR-INTERPRETER-FEATURE-GATE-DESIGN-001.md`
5. `docs/development/current/main/design/build-crate-split-plan-ssot.md`
6. `docs/development/current/main/design/vm-active-lane-retirement-ssot.md`
7. `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Proof Bundle

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
