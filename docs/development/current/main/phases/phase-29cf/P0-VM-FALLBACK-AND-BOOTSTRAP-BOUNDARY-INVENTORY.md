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

## Do Not

- mainline route を `compat-fallback` へ戻さない
- closeout 済みの `phase-29cc` checklist を reopen しない
- speculative code retire を docs inventory より先に進めない
