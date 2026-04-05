# Phase 132x Task Board

## A. Inventory

- [ ] `src/cli/args.rs`
- [ ] `src/runner/dispatch.rs`
- [ ] `src/runner/stage1_bridge/direct_route/mod.rs`
- [ ] `tools/stage1_minimal.sh`
- [ ] `tools/stage1_debug.sh`
- [ ] `tools/smokes/v2/profiles/integration/apps/phase21_5_perf_direct_emit_dominance_block_vm.sh`

## B. Decision

- [ ] omitted-backend caller inventory complete
- [ ] determine whether default `vm` is still required
- [ ] if retained, align help/docs with explicit legacy keep/debug wording
- [ ] if changed, update code and callers together

## C. Proof

- [ ] `cargo test --lib route_orchestrator::tests --quiet`
- [ ] `cargo test --lib stage1_bridge::env::tests --quiet`
- [ ] `cargo test --lib stage1_bridge::plan::tests --quiet`
- [ ] `cargo test --lib stage1_bridge::stub_child::tests --quiet`
- [ ] `bash tools/selfhost/mainline/stage1_mainline_smoke.sh`
- [ ] `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_direct_emit_dominance_block_vm.sh`

## D. Closeout

- [ ] update `CURRENT_TASK.md`
- [ ] update `05-Restart-Quick-Resume.md`
- [ ] update `10-Now.md`
- [ ] update `15-Workstream-Map.md`
