---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X43 L1 MIR cache（CB-2）の契約を固定する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-68-cache-key-determinism-ssot.md
  - docs/development/current/main/design/hako-module-cache-build-ssot.md
  - tools/cache/phase29x_l1_mir_cache.sh
  - tools/checks/phase29x_l1_mir_cache_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_l1_mir_cache_vm.sh
---

# Phase 29x X43: L1 MIR Cache SSOT

## 0. Goal

module 単位の MIR/ABI artifact 保存・再利用（L1 cache）を導入し、
`miss -> hit` が機械検証できる状態を固定する。

## 1. Contract

`phase29x_l1_mir_cache_vm.sh`（内部で `phase29x_l1_mir_cache_guard.sh`）は次を検証する。

1. 初回実行は `cache_status=miss` で MIR/ABI artifact を materialize する。
2. 2回目実行は `cache_status=hit` で同じ `module_compile_key`/artifact path を再利用する。
3. artifact は `target/hako-cache/v1/<profile>/<target>/mir|abi/<module-id>/...` に保存される。

実装:

- key derivation: `tools/cache/phase29x_cache_keys.sh`（X42）
- L1 cache: `tools/cache/phase29x_l1_mir_cache.sh`
- MIR emit helper: `tools/hakorune_emit_mir.sh`

## 2. Acceptance

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_l1_mir_cache_vm.sh` が PASS。
2. `README / 29x-90 / 29x-91 / CURRENT_TASK` が X43 完了状態に同期。
3. 次タスク `X44`（L2 object cache）へ進める。

## 3. Evidence (X43)

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_l1_mir_cache_vm.sh`

## 4. Next Step

X44（L2 object cache）は完了。次は X45 で L3 link cache を導入し、
entry + ordered object set 不変時の link 再利用を契約化する。
