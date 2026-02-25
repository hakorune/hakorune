---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X42 cache key determinism（CB-1）の契約を固定する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-67-post29x-cache-lane-sequencing-ssot.md
  - docs/development/current/main/design/hako-module-cache-build-ssot.md
  - tools/cache/phase29x_cache_keys.sh
  - tools/checks/phase29x_cache_key_determinism_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_cache_key_determinism_vm.sh
---

# Phase 29x X42: Cache Key Determinism SSOT

## 0. Goal

cache lane の入口として、`ModuleCompileKey` / `ObjectKey` / `LinkKey` の
決定規則を固定し、同一入力で再現可能な key 生成を契約化する。

## 1. Contract

`phase29x_cache_key_determinism_vm.sh`（内部で `phase29x_cache_key_determinism_guard.sh`）は次を検証する。

1. 同一入力 2 回で `module_compile_key/object_key/link_key` が一致する。
2. `profile` 差分で `module_compile_key` が変化する。
3. `abi_boundary_digest` 差分で `object_key/link_key` が変化する。

Key 生成実装:

- `tools/cache/phase29x_cache_keys.sh`
- 出力は component digest を含む key-value 形式で、差分理由の観測を可能にする。

## 2. Acceptance

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_cache_key_determinism_vm.sh` が PASS。
2. `README / 29x-90 / 29x-91 / CURRENT_TASK` が X42 完了状態に同期。
3. 次タスク `X43`（L1 MIR cache）へ進める。

## 3. Evidence (X42)

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_cache_key_determinism_vm.sh`

## 4. Next Step

X43（L1 MIR cache）と X44（L2 object cache）は完了。次は X45 で L3 link cache を導入し、
X42 key 契約を link 再利用判定へ接続する。
