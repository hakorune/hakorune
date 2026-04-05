# Phase 120x: vm route retirement decision refresh

- 目的: `compat / proof / debug-observability` だけに残った vm-family route を、retire-ready / explicit keep の境界で再判定する。
- 対象:
  - `tools/selfhost/run.sh`
  - `tools/selfhost/lib/selfhost_run_routes.sh`
  - `tools/selfhost/proof/*`
  - `tools/smokes/v2/profiles/integration/phase29x/observability/*`
  - `tools/dev/bug_origin_triage.sh`
  - `src/runner/dispatch.rs`
  - `src/runner/route_orchestrator.rs`
- success:
  - `vm-family` の残存 surface が `compat / proof / debug-observability` の3 bucketsで再確認される
  - 次に retire するなら何を先に切るかが current docs から読める
  - current pointers が `phase-120x` に揃う

## First-pass buckets

- keep-now debug-observability
  - `tools/smokes/v2/profiles/integration/phase29x/observability/phase29x_vm_route_observability_vm.sh`
  - `tools/smokes/v2/profiles/integration/phase29x/observability/phase29x_vm_route_strict_dev_priority_vm.sh`
  - `tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`
  - `tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_strict_dev_replay_vm.sh`
  - `tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_derust_strict_default_route_vm.sh`
  - `tools/smokes/v2/profiles/integration/phase29x/observability/phase29x_vm_route_compat_bypass_guard_vm.sh`
  - `tools/smokes/v2/profiles/integration/phase29x/observability/phase29x_vm_route_pin_guard_vm.sh`
- candidate-thin
  - `tools/dev/bug_origin_triage.sh`
  - `tools/checks/route_env_probe.sh`
  - `tools/checks/vm_route_bypass_guard.sh`
  - `tools/checks/phase29x_vm_route_pin_guard.sh`
  - `tools/checks/phase29x_vm_route_pin_allowlist.txt`
  - `tools/smokes/v2/suites/integration/phase29x-vm-route.txt`
- retire-later pressure
  - `tools/selfhost/proof/run_stageb_compiler_vm.sh`
  - `tools/selfhost/proof/selfhost_vm_smoke.sh`
  - `README.md`
  - `README.ja.md`
  - `tools/selfhost/README.md`
  - `docs/development/runtime/cli-hakorune-stage1.md`
  - `docs/development/selfhosting/quickstart.md`
