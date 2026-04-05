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
  - `src/runner/stage1_bridge/direct_route/mod.rs` still requires `backend == "vm"` for binary-only direct run
  - public docs still carry explicit `--backend vm` examples for proof/debug

## Decision

- decision-now
  - keep `--backend vm` public as an explicit gate for now
- why
  - CLI contract still defaults `--backend` to `vm`
  - dispatch still exposes `backend=vm` as a first-class branch
  - compat route and Stage1 direct bridge still rely on raw `--backend vm`
- demote-first
  - docs/manual surfaces and engineering triage helpers can shrink earlier than the backend gate itself
- next lane
  - `phase-122x vm compat route exit plan`
