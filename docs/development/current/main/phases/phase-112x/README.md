# Phase 112x: vm-family lane naming hardening

- 目的: `backend=vm` family と internal lane を同じ語で混ぜない。
- canonical internal lane names:
  - `rust-vm-keep`
  - `vm-hako-reference`
  - `vm-compat-fallback`
- 対象:
  - `src/runner/route_orchestrator.rs`
  - `src/runner/dispatch.rs`
  - `tools/smokes/v2/profiles/integration/phase29x/**`
  - `tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_strict_dev_replay_vm.sh`
  - `tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh`
  - `docs/development/architecture/selfhost_execution_ssot.md`
- success:
  - route trace / derust trace が canonical lane 名を出す
  - active observability smokes が canonical lane 名で green
  - `backend override` と `lane` の語彙が current docs で分離される
