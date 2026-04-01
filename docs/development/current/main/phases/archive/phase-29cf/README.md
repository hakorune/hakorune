---
Status: Accepted (monitor-only)
Decision: accepted
Date: 2026-03-09
Scope: 脱Rust selfhost closeout 後の `VM fallback compat lane` と `bootstrap boundary reduction` を独立管理する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - docs/development/current/main/phases/phase-29cc/29cc-260-derust-task-checklist.md
  - docs/development/current/main/phases/phase-29cf/P0-VM-FALLBACK-AND-BOOTSTRAP-BOUNDARY-INVENTORY.md
  - docs/development/current/main/phases/phase-29cf/29cf-10-vm-fallback-bootstrap-retirement-checklist.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/de-rust-lane-map-ssot.md
  - src/runner/route_orchestrator.rs
  - src/runner/modes/vm_fallback.rs
  - tools/selfhost/lib/identity_routes.sh
  - tools/selfhost/build_stage1.sh
---

# Phase 29cf: De-Rust Selfhost Follow-up

## Goal

`phase-29cc` の top-level closeout を再オープンせず、残っている

1. `VM fallback compat lane`
2. `bootstrap boundary reduction`

を docs-first で棚卸しし、`keep` / `future retire target` / `monitor-only` を固定する。

## Closeout Decision

1. `compat-fallback` は current mainline route ではなく、explicit compatibility keep として固定する
2. Stage0 / Stage1 / Stage2 の bootstrap boundary は inventory-fixed とし、実削減は future-wave へ分離する
3. 本 phase は `monitor-only` で維持し、route authority か bootstrap dependency に具体的な削減候補が出た時だけ再オープンする

## Master Pointer

- `phase-29cc` は closeout 済みの orchestration lane として維持する
- この phase は closeout 後の follow-up を独立管理する
- checkbox 正本:
  - `docs/development/current/main/phases/phase-29cf/29cf-10-vm-fallback-bootstrap-retirement-checklist.md`

## Non-goals

- `phase-29cc` の done judgment を巻き戻すこと
- silent fallback を増やすこと
- current mainline route を `compat-fallback` へ戻すこと

## Fixed Workstreams

1. `VM fallback compat lane`
   - `vm` / `vm-hako` / `compat-fallback` の current contract を固定する
   - `NYASH_VM_USE_FALLBACK=1` を explicit compat keep として扱う
2. `bootstrap boundary reduction`
   - Stage0 / Stage1 / Stage2 の keep boundary を明示する
   - `stage1-cli` と default bootstrap lane の dependency を分けて読む

## Current Snapshot

- current mainline route:
  - `vm`
  - `vm-hako`
- explicit compat keep:
  - `compat-fallback`
  - `stage0` recovery in `identity_routes.sh`
- bootstrap boundary:
  - `stage1` route is current selfhost identity route
  - `stage0` / `auto` are compat-only
  - Stage2 still depends on default bootstrap lane and remains a future reduction target
- `docs/private` drift is out of scope here

## Live Caller Matrix

| Bucket | Current owner / caller | Meaning |
| --- | --- | --- |
| `current keep` | [route_orchestrator.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/route_orchestrator.rs), [vm_fallback.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/modes/vm_fallback.rs), [vm_backend_flags.rs](/home/tomoaki/git/hakorune-selfhost/src/config/env/vm_backend_flags.rs) | explicit compat lane implementation remains in top-level runtime; not a mainline route |
| `route observability keep` | [phase29x_vm_route_observability_vm.sh](/home/tomoaki/git/hakorune-selfhost/tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_observability_vm.sh) | observes `[vm-route/pre-dispatch]` and `[vm-route/select]` tags across `vm` / `compat-fallback` / `vm-hako` |
| `strict-dev priority keep` | [phase29x_vm_route_strict_dev_priority_vm.sh](/home/tomoaki/git/hakorune-selfhost/tools/smokes/v2/profiles/integration/apps/phase29x_vm_route_strict_dev_priority_vm.sh) | proves `strict/dev -> vm-hako`, while compat fallback remains explicit-only |
| `non-strict compat boundary keep` | [phase29x_vm_route_non_strict_compat_boundary_vm.sh](/home/tomoaki/git/hakorune-selfhost/tools/smokes/v2/profiles/integration/selfhost/phase29x_vm_route_non_strict_compat_boundary_vm.sh) | proves non-strict stage-a rejects compat unless `NYASH_VM_USE_FALLBACK=1` is explicit |
| `done-sync keep` | [phase29x_derust_strict_default_route_vm.sh](/home/tomoaki/git/hakorune-selfhost/tools/smokes/v2/profiles/integration/apps/phase29x_derust_strict_default_route_vm.sh) | current de-rust done-sync evidence consumed by the done matrix and lane map |
| `current diagnostics keep` | [route_env_probe.sh](/home/tomoaki/git/hakorune-selfhost/tools/checks/route_env_probe.sh) | active diagnostics helper used by route guard/check scripts and tools docs |
| `plugin test keep` | [plugin_loader_v2 route_resolver tests](/home/tomoaki/git/hakorune-selfhost/src/runtime/plugin_loader_v2/enabled/route_resolver.rs) | plugin route-resolver tests still exercise explicit fallback coverage |
| `compat-only recovery keep` | [identity_routes.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost/lib/identity_routes.sh) | `auto -> stage0` recovery and `[identity/compat-fallback]` evidence line |
| `historical canary keep` | `tools/smokes/v2/profiles/integration/core/phase2043/lower_{load_store_local,typeop_cast,typeop_check}_direct_struct_canary_vm.sh` | old direct-struct canaries still exercise explicit fallback, but are no longer part of current mainline authority |
| `historical / archive` | `phase-29x` historical SSOTs and archived handoff docs | evidence only; not a current route owner |

## Bootstrap Boundary Matrix

| Boundary | Current owner | Bucket | Note |
| --- | --- | --- | --- |
| `stage1` identity route | [selfhost_identity_check.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost_identity_check.sh), [run_stage1_cli.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost/run_stage1_cli.sh) | `current keep` | mainline selfhost identity route |
| `stage0` / `auto` identity route | [selfhost_identity_check.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost_identity_check.sh), [identity_routes.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost/lib/identity_routes.sh) | `compat keep` | compatibility-only recovery; not full-mode evidence |
| `launcher-exe` default artifact | [build_stage1.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost/build_stage1.sh) | `bootstrap keep` | run-oriented default artifact; does not satisfy G1 emit contract by itself |
| `stage1-cli` artifact | [build_stage1.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost/build_stage1.sh) | `current keep` | explicit emit-capable artifact for G1/full mode |
| Stage2 default-bootstrap dependency | [selfhost_identity_check.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost_identity_check.sh) | `future retire target` | when artifact-kind=`stage1-cli`, Stage2 build prints `stage1-cli artifact is emit-route entry only; using default bootstrap for Stage2 build`; removing this dependency is the next dedicated reduction target |

## Exit Criteria

- `VM fallback compat lane` の keep/reject/future-retire bucket が checklist で固定されている
- `bootstrap boundary reduction` の Stage0/1/2 keep matrix が checklist で固定されている
- current docs / comments が `compat keep` と `mainline keep` を混同しない
