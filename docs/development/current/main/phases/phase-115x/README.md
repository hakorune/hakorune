# Phase 115x: vm route retirement planning

- 目的: `--backend vm` を通常実行経路ではなく compat/proof/debug override として凍結した上で、将来の route retirement 順を固定する。
- 対象:
  - `src/runner/route_orchestrator.rs`
  - `src/runner/dispatch.rs`
  - `tools/selfhost/run.sh`
  - `tools/selfhost/lib/selfhost_run_routes.sh`
  - `tools/selfhost/proof/*`
  - active vm-family observability / compat boundary smokes
- success:
  - `--backend vm` が keep/debug family でしか使われない current shape を inventory 化できる
  - compat / proof / debug の dependency を分けて retirement order を書ける
  - next lane が `alias pruning` か `explicit env hardening` のどちらかに絞れる

## Current Inventory

- compat route
  - `tools/selfhost/run.sh --runtime --runtime-route compat`
  - `tools/selfhost/lib/selfhost_run_routes.sh`
  - current body still resolves `runtime-route compat` / `stage-a-compat` to raw `--backend vm`
  - explicit guard remains `NYASH_VM_USE_FALLBACK=1`
- proof wrappers
  - `tools/selfhost/proof/run_stageb_compiler_vm.sh`
  - `tools/selfhost/proof/bootstrap_selfhost_smoke.sh`
  - `tools/selfhost/proof/selfhost_smoke.sh`
  - `tools/selfhost/proof/selfhost_vm_smoke.sh`
  - `tools/selfhost/proof/selfhost_stage3_accept_smoke.sh`
  - all still call raw `--backend vm` or `--ny-parser-pipe --backend vm`
- active debug / observability
  - `tools/smokes/v2/profiles/integration/phase29x/observability/*`
  - `tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`
  - these pin `rust-vm-keep` / `vm-compat-fallback` / `vm-hako-reference` tags as current contract

## Retirement Order

1. execution surface alias pruning
   - shrink `stage-a` / `stage-a-compat` and route/mode alias pressure first
2. compat env hardening
   - keep `runtime-route compat` explicit and narrow
   - avoid widening raw `--backend vm` beyond guarded compat
3. proof wrapper repoint/review
   - decide which proof wrappers stay public proof surfaces and which move behind thinner helpers
4. debug / observability review
   - keep only the vm-family traces still needed after alias/env hardening
5. backend retirement decision
   - only after compat / proof / debug callers are reduced
