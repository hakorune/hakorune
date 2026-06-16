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
- exact-front optimization and representation rows remain governed by
  `CURRENT_STATE.toml`; the latest row is
  `OBJECT-STORAGE-PLAN-MODULE-SPLIT-001`, and the next blocker is
  `OBJECT-STORAGE-PLAN-VOCAB-AUDIT-001`
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
2. `docs/development/current/main/phases/phase-296x/296x-989-OBJECT-STORAGE-PLAN-MODULE-SPLIT-001.md`
3. `docs/development/current/main/design/perf-owner-first-optimization-ssot.md`
4. `docs/development/current/main/phases/phase-296x/296x-738-SIMPLE-BOX-EXACT-OBJECT-CANDIDATE-001.md`
5. `docs/development/current/main/phases/phase-296x/296x-661-BOOL-SCALAR-LOWERING-001.md`
6. `docs/development/current/main/phases/phase-293x/293x-1040-COMPILER-FOUNDATION-CHECKPOINT-001.md`
7. `docs/development/current/main/workstreams/compiler-foundation-current.md`
8. `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Proof Bundle

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
