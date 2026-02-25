---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X45 L3 link cache（CB-4）の契約を固定する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-70-l2-object-cache-ssot.md
  - docs/development/current/main/design/hako-module-cache-build-ssot.md
  - tools/cache/phase29x_l3_link_cache.sh
  - tools/checks/phase29x_l3_link_cache_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_l3_link_cache_vm.sh
---

# Phase 29x X45: L3 Link Cache SSOT

## 0. Goal

entry + object input が不変なときに link 実行を再利用し、
`miss -> hit` と runtime ABI 差分 miss を機械検証できる状態を固定する。

## 1. Contract

`phase29x_l3_link_cache_vm.sh`（内部で `phase29x_l3_link_cache_guard.sh`）は次を検証する。

1. 初回実行は `cache_status=miss` で link artifact（manifest + binary）を materialize する。
2. 2回目実行は `cache_status=hit` で同じ `link_key/manifest_path/binary_path` を再利用する。
3. `runtime_abi_digest` 差分時は `cache_status=miss` となり、`link_key` が変化する。
4. runtime ABI 差分 run でも L2 は `l2_cache_status=hit` を維持し、object 再生成は不要である。

実装:

- key derivation: `tools/cache/phase29x_cache_keys.sh`（X42）
- L2 prerequisite: `tools/cache/phase29x_l2_object_cache.sh`（X44）
- L3 cache: `tools/cache/phase29x_l3_link_cache.sh`

## 2. Acceptance

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_l3_link_cache_vm.sh` が PASS。
2. `README / 29x-90 / 29x-91 / CURRENT_TASK` が X45 完了状態に同期。
3. 次タスク `X46`（cache gate integration + done sync）へ進める。

## 3. Evidence (X45)

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_l3_link_cache_vm.sh`

## 4. Next Step

X46 で cache lane の統合 gate を追加し、
daily/milestone で L1/L2/L3 cache hit/miss 観測を固定する。
