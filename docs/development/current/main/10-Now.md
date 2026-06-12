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
- substring-concat closed-form lowering SSOT: `docs/development/current/main/design/substring-concat-len-closed-form-lowering-ssot.md`
- comparison note: `docs/development/current/main/investigations/hako-vs-c-mimalloc-direct-exact-comparison-2026-06-09.md`
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
- if string hot-path work resumes, use the array-text session route SSOT
  instead of extending `nyash.array.string_indexof_hisi` as the semantic owner
  and continue through the selected-route cache-backed session boundary slice
- if MIR / LLVM IR / assembly slices are needed, use the inspect scope dump
  SSOT: source anchors are selectors, while dumps are `hako_check` artifacts
  with explicit mapping quality
- if `kilo_micro_substring_concat` resumes, continue as lowering/codegen
  residual work: consume the existing StableLengthScalar route and emit
  closed-form scalar IR instead of adding source or MIRBuilder witnesses
- use the inventory note first when pointer hunting is noisy
- keep allocator-provider activation, hooks, host allocator replacement, and `#[global_allocator]` out of scope
- further optimization must use the active method anchor from
  `CURRENT_STATE.toml` instead of stale historical lane notes

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
6. `docs/development/current/main/design/substring-concat-len-closed-form-lowering-ssot.md`
7. `docs/development/current/main/design/current-docs-update-policy-ssot.md`

## Proof Bundle

```bash
git status -sb
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
