Status: SSOT
Date: 2026-06-09
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
- compiler foundation taskboard: `docs/development/current/main/workstreams/compiler-foundation-current.md`
- active selection card: `docs/development/current/main/phases/phase-293x/293x-1004-COMPILER-FOUNDATION-SELECTION-001.md`
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
- next BoxCallable task is reconciliation/proof, not first implementation:
  reconcile older TypeAbiBoxDomain rows with landed BoxCallable rows and name
  narrow proof commands
- CorePlan C1 planner_required route-exhaustion, D1 normalizer AST-boundary,
  E1 active-v0 inventory, E1-002 first retire, and E1-003 collect_using_entries
  retire guards are landed; next implementation is `COREPLAN-E1-004`
  bundle_resolver v0 retire
- exact-front optimization is paused; resume later through
  `MIMALLOC-AOT-KERNEL-FRONT-SELECT-002`, not from historical perf notes
- keep allocator-provider activation, hooks, host allocator replacement, and `#[global_allocator]` out of scope
- use the active method anchor from `CURRENT_STATE.toml` instead of stale
  historical lane notes

## Rules

- keep BoxShape and BoxCount separate
- do not grow the restart mirrors with landed history
- update `CURRENT_STATE.toml` and the active card first

## Read Next

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/workstreams/compiler-foundation-current.md`
3. `docs/development/current/main/phases/phase-293x/293x-1004-COMPILER-FOUNDATION-SELECTION-001.md`
4. `docs/development/current/main/design/box-callable-registry-ssot.md`
5. `docs/development/current/main/design/type-abi-catalog-planning-spine-ssot.md`
6. `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`
7. `docs/development/current/main/phases/phase-293x/293x-1006-COREPLAN-FOUND-002-REMAINING-FAMILY-INVENTORY.md`
8. `docs/development/current/main/phases/phase-293x/293x-1011-COREPLAN-E1-003-COLLECT-USING-ENTRIES-V0-RETIRE.md`
9. `docs/development/current/main/design/coreplan-compat-normalizer-legoization-ssot.md`
10. `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Proof Bundle

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
