# Phase 121x: vm backend retirement gate decision

- 目的: `--backend vm` を public explicit gate のまま残すか、internal-only gate へ狭めるかを blocker ベースで判定する。
- 対象:
  - `src/cli/args.rs`
  - `src/runner/dispatch.rs`
  - `src/runner/route_orchestrator.rs`
  - `tools/selfhost/run.sh`
  - `tools/selfhost/lib/selfhost_run_routes.sh`
  - `tools/selfhost/proof/*`
  - `README.md`
  - `README.ja.md`
  - `tools/selfhost/README.md`
  - `docs/development/runtime/cli-hakorune-stage1.md`
  - `docs/development/selfhosting/quickstart.md`
- success:
  - `keep public for now / could demote internal-only / blockers` の3 buckets が current docs で読める
  - `phase-122x vm compat route exit plan` に渡す gate decision が固定される

## First-pass decision buckets

- keep public for now
  - `src/cli/args.rs`
  - `src/runner/dispatch.rs`
  - `tools/selfhost/proof/run_stageb_compiler_vm.sh`
  - `tools/selfhost/proof/selfhost_vm_smoke.sh`
- could demote internal-only
  - `tools/dev/bug_origin_triage.sh`
  - `tools/checks/route_env_probe.sh`
  - `docs/tools/README.md`
- blockers
  - `src/cli/args.rs` still defaults `--backend` to `vm`
  - `tools/selfhost/lib/selfhost_run_routes.sh` compat branch still shells into raw `--backend vm`
  - `src/runner/route_orchestrator.rs` still has `emit-mode-force-rust-vm-keep`
  - public docs still carry explicit `--backend vm` examples for proof/debug
