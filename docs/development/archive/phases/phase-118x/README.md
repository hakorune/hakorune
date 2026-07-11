# Phase 118x: proof wrapper surface review

- 目的: `tools/selfhost/proof/*` を public proof surface と internal proof helper に分け、front-door で広く見せすぎない。
- 対象:
  - `tools/selfhost/README.md`
  - `README.md`
  - `README.ja.md`
  - `Makefile`
  - `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
  - `docs/development/selfhosting/quickstart.md`
  - `docs/development/runtime/cli-hakorune-stage1.md`
- success:
  - public proof surface が `run_stageb_compiler_vm.sh` / `selfhost_vm_smoke.sh` 中心に見える
  - `bootstrap_selfhost_smoke.sh` / `selfhost_smoke.sh` / `selfhost_stage3_accept_smoke.sh` は internal proof helper として読める
  - current pointers が `phase-118x` に揃う
