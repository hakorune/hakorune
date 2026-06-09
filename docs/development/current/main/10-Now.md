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
- inventory note: `docs/development/current/main/investigations/docs-pointer-inventory-2026-06-09.md`
- typed-object exact slot ABI SSOT: `docs/development/current/main/design/typed-object-exact-slot-abi-ssot.md`
- adjacent array-text session route SSOT: `docs/development/current/main/design/array-text-session-route-ssot.md`
- inspect scope dump SSOT: `docs/development/current/main/design/hako-inspect-scope-dump-ssot.md`
- comparison note: `docs/development/current/main/investigations/hako-vs-c-mimalloc-direct-exact-comparison-2026-06-09.md`
- active lane: read `active_lane` in `CURRENT_STATE.toml`
- active phase: read `active_phase` in `CURRENT_STATE.toml`
- phase status: read `phase_status` in `CURRENT_STATE.toml`
- latest card: read `latest_card_path` in `CURRENT_STATE.toml`
- blocker token: read `current_blocker_token` in `CURRENT_STATE.toml`
- implementation gaps: none; open work is typed-object exact slot ABI split, and adjacent
  array-text session route work is landing through the selected-route boundary slice

## Next

- continue the active phase from `current_blocker_token`, `phase_status`, and
  `latest_card_path` in `CURRENT_STATE.toml`
- current day-to-day tasks live in `latest_workstream_card` from
  `CURRENT_STATE.toml`
- if string hot-path work resumes, use the array-text session route SSOT
  instead of extending `nyash.array.string_indexof_hisi` as the semantic owner
  and continue through the selected-route cache-backed session boundary slice
- if MIR / LLVM IR / assembly slices are needed, use the inspect scope dump
  SSOT: source anchors are selectors, while dumps are `hako_check` artifacts
- use the inventory note first when pointer hunting is noisy
- keep allocator-provider activation, hooks, host allocator replacement, and `#[global_allocator]` out of scope
- further user-box counter-heavy optimization must use selected exact slot routes, not compat `field_get_hii`

## Rules

- keep BoxShape and BoxCount separate
- do not grow the restart mirrors with landed history
- update `CURRENT_STATE.toml` and the active card first

## Read Next

1. `docs/development/current/main/CURRENT_STATE.toml`
2. `docs/development/current/main/investigations/docs-pointer-inventory-2026-06-09.md`
3. `docs/development/current/main/design/typed-object-exact-slot-abi-ssot.md`
4. `docs/development/current/main/design/array-text-session-route-ssot.md`
5. `docs/development/current/main/design/hako-inspect-scope-dump-ssot.md`
6. `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Proof Bundle

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
