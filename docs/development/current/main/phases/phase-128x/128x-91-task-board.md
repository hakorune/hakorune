# Phase 128x Task Board

## A. Inventory

- [ ] `src/runner/stage1_bridge/plan.rs`
- [ ] `src/runner/stage1_bridge/args.rs`
- [ ] `src/runner/stage1_bridge/env/stage1_aliases.rs`
- [ ] `src/config/env/stage1.rs`
- [ ] `src/runner/stage1_bridge/direct_route/mod.rs`
- [ ] `src/runner/stage1_bridge/route_exec/direct.rs`

## B. Softening

- [ ] identify which helper still hard-requires `backend=vm`
- [ ] decide whether direct-route temp-MIR handoff can be route-first only
- [ ] keep compat fallback env explicit

## C. Proof

- [ ] `bash tools/selfhost/mainline/stage1_mainline_smoke.sh`
- [ ] `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_mode_parity_smoke_vm.sh`
- [ ] `bash tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`

## D. Closeout

- [ ] update `CURRENT_TASK.md`
- [ ] update `05-Restart-Quick-Resume.md`
- [ ] update `10-Now.md`
- [ ] update `15-Workstream-Map.md`
