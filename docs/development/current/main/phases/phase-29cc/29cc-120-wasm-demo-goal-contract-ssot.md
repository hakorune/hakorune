---
Status: Active
Decision: accepted
Date: 2026-02-26
Scope: `projects/nyash-wasm` を WASM lane の段階ゴール指標として固定し、`WSM-02+` の受け入れ判定に接続する。
Related:
  - projects/nyash-wasm/README.md
  - docs/development/current/main/phases/phase-29cc/29cc-118-wasm-grammar-compat-map-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-119-wsm02a-assignment-local-unblock-ssot.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
---

# 29cc-120 WASM Demo Goal Contract SSOT

## 0. Goal

`projects/nyash-wasm`（archived prototype）を、WASM lane の「再到達ゴールの目安」として正式に採用する。

- 目的は「過去デモの完全復刻」ではなく、現行 mainline の実装能力で段階再現すること。
- plugin/mainline lane を止めない non-blocking goal として扱う。

## 1. Goal Levels

### G0 (contract lock)
- `projects/nyash-wasm` を目安として採用すること自体を SSOT で固定。

### G1 (compiler/runtime minimum)
- `hakorune` で WASM compile path が通る（MIR -> WAT -> wasm）
- `WSM-02a..02d` の最低ゲートが緑である

### G2 (browser demo minimum)
- `projects/nyash-wasm/nyash_playground.html` 相当の最小デモで、
  - console 系の基本動作
  - コード実行ループ（Runボタン相当）が動作

### G3 (historical parity subset)
- `projects/nyash-wasm` の代表デモ（1～2本）を現行経路で再実行できる
- 非対象（scope外）は fail-fast で明示

## 2. Non-goals

1. `projects/nyash-wasm` 全HTMLの一括復活
2. 古い wasm-bindgen/旧ランタイム構成の完全互換
3. thread/worker 前提の並行デモ（現時点 scope 外）

## 3. WSM-02+ Integration

- `WSM-02a`: assignment/local path unblock（done）
- `WSM-02b`: ExternCall coverage expansion
- `WSM-02c`: BoxCall coverage expansion
- `WSM-02d`: boundary gates + demo-min fixture lock

受け入れ原則:
- 1 blocker = 1受理形 = 1コミット
- unsupported は silent fallback せず fail-fast

## 4. Acceptance Commands

Daily:
1. `cargo check --bin hakorune`
2. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
3. `bash tools/smokes/v2/profiles/integration/apps/archive/phase134_plugin_best_effort_init.sh`

Milestone (WASM demo goal):
1. `bash tools/vm_plugin_smoke.sh`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm02d_demo_min_boundary_vm.sh`

## 5. Decision

Decision: accepted

- `projects/nyash-wasm` は WASM lane の段階ゴール指標として採用。
- 詳細仕様はこの文書を正本とし、`CURRENT_TASK.md` はポインタのみ保持する。
