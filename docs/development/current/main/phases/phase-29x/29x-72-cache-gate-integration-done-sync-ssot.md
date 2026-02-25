---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X46 cache gate integration + done sync（CB-5）の契約を固定する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-71-l3-link-cache-ssot.md
  - docs/development/current/main/design/hako-module-cache-build-ssot.md
  - tools/checks/phase29x_cache_gate_integration_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_cache_lane_gate_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh
---

# Phase 29x X46: Cache Gate Integration + Done Sync SSOT

## 0. Goal

X42-X45 cache lane を daily/milestone 入口へ統合し、
cache hit/miss 観測を運用契約として再現可能に固定する。

## 1. Contract

`phase29x_llvm_only_daily_gate.sh` は次を順序固定で実行する。

1. `tools/checks/abi_lane_guard.sh`
2. `tools/smokes/v2/profiles/integration/apps/phase29x_llvm_cabi_link_min.sh`
3. `tools/checks/phase29x_cache_gate_integration_guard.sh`
4. `tools/smokes/v2/profiles/integration/apps/phase29x_cache_lane_gate_vm.sh`

`phase29x_cache_lane_gate_vm.sh` は X42-X45 smoke を 1 コマンドで再生する。

1. `phase29x_cache_key_determinism_vm.sh`
2. `phase29x_l1_mir_cache_vm.sh`
3. `phase29x_l2_object_cache_vm.sh`
4. `phase29x_l3_link_cache_vm.sh`

`phase29x_cache_gate_integration_guard.sh` は次を fail-fast で検証する。

1. daily gate が `phase29x_cache_lane_gate_vm.sh` callsite を持つ。
2. cache lane gate が X42-X45 smoke を全て含む。

## 2. Acceptance

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_cache_lane_gate_vm.sh` が PASS。
2. `bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh` が PASS。
3. `README / 29x-90 / 29x-91 / CURRENT_TASK` が X46 完了状態に同期。
4. rollback 条件と fail-fast 契約が docs で同期される。

## 3. Evidence (X46)

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_cache_lane_gate_vm.sh`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29x_llvm_only_daily_gate.sh`

## 4. Next Step

X41-X46（post-29x cache lane）は完了。以後は daily/milestone で統合 gate を運用し、
新規 lane を開始する場合は SSOT 入口（README/29x-90/29x-91/CURRENT_TASK）を先に同期する。
