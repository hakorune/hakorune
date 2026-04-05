# Phase 128x Task Board

## A. Inventory

- [x] `src/runner/stage1_bridge/plan.rs`
- [x] `src/runner/stage1_bridge/args.rs`
- [x] `src/runner/stage1_bridge/env/stage1_aliases.rs`
- [x] `src/config/env/stage1.rs`
- [x] `src/runner/stage1_bridge/direct_route/mod.rs`
- [x] `src/runner/stage1_bridge/route_exec/direct.rs`

## B. Softening

- [x] identify which helper still hard-requires `backend=vm`
- [x] keep compat fallback env explicit
- [x] default `stage1_cli_env.hako` child path stays backend-hint free
- [x] `BinaryOnlyRunDirect` plan no longer depends on backend CLI hint
- [ ] decide whether the binary-only direct-route vm gate should remain as an explicit legacy contract

## C. Proof

- [x] `bash tools/selfhost/mainline/stage1_mainline_smoke.sh`
- [x] `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_runtime_mode_parity_smoke_vm.sh`
- [x] `bash tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`

## D. Closeout

- [x] update `CURRENT_TASK.md`
- [x] update `05-Restart-Quick-Resume.md`
- [x] update `10-Now.md`
- [x] update `15-Workstream-Map.md`
