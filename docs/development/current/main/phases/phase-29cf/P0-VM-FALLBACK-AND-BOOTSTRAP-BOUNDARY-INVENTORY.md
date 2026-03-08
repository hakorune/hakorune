---
Status: Accepted
Decision: accepted
Date: 2026-03-09
Scope: `phase-29cf` の初手として、VM fallback compat lane と bootstrap boundary の inventory を固定する。
Related:
  - docs/development/current/main/phases/phase-29cf/README.md
  - docs/development/current/main/phases/phase-29cf/29cf-10-vm-fallback-bootstrap-retirement-checklist.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - src/runner/route_orchestrator.rs
  - src/runner/modes/vm_fallback.rs
  - tools/selfhost/lib/identity_routes.sh
  - tools/selfhost/build_stage1.sh
---

# P0: VM Fallback And Bootstrap Boundary Inventory

## Purpose

- `compat-fallback` を current mainline と混同しない
- Stage0 / Stage1 / Stage2 の keep boundary を 1 枚で確認できるようにする
- 後続の retire / reduction を docs-first で進める

## Fixed Order

1. `VM fallback compat lane` の current keep / compat keep / future retire target を分類する
2. Stage0 / Stage1 / Stage2 の current bootstrap dependency を分類する
3. `phase-29cf` checklist へ bucket を同期する

## Acceptance

- `route_orchestrator.rs` の `compat-fallback` が explicit compat keep と読める
- `identity_routes.sh` の `stage0` recovery が compatibility-only と読める
- checklist に `keep / future retire target / monitor-only` が反映される

## Outcome

1. `compat-fallback` は current mainline ではなく `explicit compat keep`
2. Stage0 / Stage1 / Stage2 boundary は `inventory-fixed`
3. actual reduction is a future-wave target and does not reopen `phase-29cc`

## VM Fallback Live Caller Inventory

| Bucket | Exact current files | Reason |
| --- | --- | --- |
| `implementation keep` | [src/runner/route_orchestrator.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/route_orchestrator.rs), [src/runner/modes/vm_fallback.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/modes/vm_fallback.rs), [src/config/env/vm_backend_flags.rs](/home/tomoaki/git/hakorune-selfhost/src/config/env/vm_backend_flags.rs) | runtime owns explicit opt-in compat fallback and bypass guard |
| `stage-a compat keep` | [src/runner/modes/common_util/selfhost/runtime_route_contract.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/modes/common_util/selfhost/runtime_route_contract.rs), [src/runner/modes/common_util/selfhost/stage_a_policy.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/modes/common_util/selfhost/stage_a_policy.rs), [src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs) | selfhost non-strict compat route remains explicit opt-in |
| `route authority probes` | [tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh](/home/tomoaki/git/hakorune-selfhost/tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh), [tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_observability_vm.sh](/home/tomoaki/git/hakorune-selfhost/tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_observability_vm.sh), [tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_strict_dev_priority_vm.sh](/home/tomoaki/git/hakorune-selfhost/tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_strict_dev_priority_vm.sh) | verify route authority and fail-fast guard without making compat-fallback mainline |
| `done-sync evidence` | [tools/smokes/v2/profiles/integration/apps/phase29x_derust_strict_default_route_vm.sh](/home/tomoaki/git/hakorune-selfhost/tools/smokes/v2/profiles/integration/apps/phase29x_derust_strict_default_route_vm.sh) | current de-rust done-matrix evidence for strict default route |
| `current diagnostics keep` | [tools/checks/route_env_probe.sh](/home/tomoaki/git/hakorune-selfhost/tools/checks/route_env_probe.sh) | active route environment probe consumed by guard/check scripts |
| `plugin test keep` | [src/runtime/plugin_loader_v2/enabled/route_resolver.rs](/home/tomoaki/git/hakorune-selfhost/src/runtime/plugin_loader_v2/enabled/route_resolver.rs) | plugin route-resolver tests still exercise explicit fallback contract |
| `historical canary keep` | [tools/smokes/v2/profiles/integration/core/phase2043/lower_load_store_local_direct_struct_canary_vm.sh](/home/tomoaki/git/hakorune-selfhost/tools/smokes/v2/profiles/integration/core/phase2043/lower_load_store_local_direct_struct_canary_vm.sh), [tools/smokes/v2/profiles/integration/core/phase2043/lower_typeop_cast_direct_struct_canary_vm.sh](/home/tomoaki/git/hakorune-selfhost/tools/smokes/v2/profiles/integration/core/phase2043/lower_typeop_cast_direct_struct_canary_vm.sh), [tools/smokes/v2/profiles/integration/core/phase2043/lower_typeop_check_direct_struct_canary_vm.sh](/home/tomoaki/git/hakorune-selfhost/tools/smokes/v2/profiles/integration/core/phase2043/lower_typeop_check_direct_struct_canary_vm.sh) | retained only as old explicit-fallback evidence for direct-struct bring-up; not a current route authority |
| `historical / evidence only` | `docs/development/current/main/phases/phase-29x/**`, archived `CURRENT_TASK` investigations | preserve route-history evidence only |

## Bootstrap Boundary Inventory

| Bucket | Exact current files | Reason |
| --- | --- | --- |
| `current keep` | [tools/selfhost/run_stage1_cli.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost/run_stage1_cli.sh), [tools/selfhost_identity_check.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost_identity_check.sh) | Stage1 CLI is the current selfhost identity path |
| `compat keep` | [tools/selfhost/lib/identity_routes.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost/lib/identity_routes.sh), [tools/selfhost_identity_check.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost_identity_check.sh) (`--cli-mode auto|stage0`) | stage0/auto remain recovery-only and are not accepted as main-route evidence |
| `bootstrap keep` | [tools/selfhost/build_stage1.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost/build_stage1.sh), [tools/selfhost/README.md](/home/tomoaki/git/hakorune-selfhost/tools/selfhost/README.md) | `launcher-exe` default artifact and current build route are still needed for bootstrap |
| `future retire target` | [tools/selfhost_identity_check.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost_identity_check.sh) Stage2 build path, [docs/development/current/main/design/selfhost-bootstrap-route-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/selfhost-bootstrap-route-ssot.md) | Stage2 still depends on default bootstrap lane: when artifact-kind=`stage1-cli`, Stage2 build intentionally omits `NYASH_BIN=<stage1>` and falls back to default bootstrap because the stage1-cli artifact is emit-only |

## Do Not

- mainline route を `compat-fallback` へ戻さない
- closeout 済みの `phase-29cc` checklist を reopen しない
- speculative code retire を docs inventory より先に進めない
