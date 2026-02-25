---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X44 L2 object cache（CB-3）の契約を固定する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-69-l1-mir-cache-ssot.md
  - docs/development/current/main/design/hako-module-cache-build-ssot.md
  - tools/cache/phase29x_l2_object_cache.sh
  - tools/checks/phase29x_l2_object_cache_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_l2_object_cache_vm.sh
---

# Phase 29x X44: L2 Object Cache SSOT

## 0. Goal

object artifact の module 単位再利用（L2 cache）を導入し、
`miss -> hit` と ABI 差分 miss を機械検証できる状態を固定する。

## 1. Contract

`phase29x_l2_object_cache_vm.sh`（内部で `phase29x_l2_object_cache_guard.sh`）は次を検証する。

1. 初回実行は `cache_status=miss` で object artifact を materialize する。
2. 2回目実行は `cache_status=hit` で同じ `object_key/object_path` を再利用する。
3. `abi_boundary_digest` 差分時は `cache_status=miss` となり、`object_key/object_path` が変化する。
4. L2 実行前提として L1 cache が走り、2回目以降は `l1_cache_status=hit` が観測される。

実装:

- key derivation: `tools/cache/phase29x_cache_keys.sh`（X42）
- L1 cache prerequisite: `tools/cache/phase29x_l1_mir_cache.sh`（X43）
- L2 cache: `tools/cache/phase29x_l2_object_cache.sh`

## 2. Acceptance

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_l2_object_cache_vm.sh` が PASS。
2. `README / 29x-90 / 29x-91 / CURRENT_TASK` が X44 完了状態に同期。
3. 次タスク `X45`（L3 link cache）へ進める。

## 3. Evidence (X44)

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_l2_object_cache_vm.sh`

## 4. Next Step

X45 で L3 link cache を導入し、
entry + ordered object set 不変時の link 再利用を契約化する。
