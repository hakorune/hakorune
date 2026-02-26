---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: wasm lane `WSM-01` として unsupported inventory を現行実装へ同期し、BoxCall/ExternCall fail-fast 診断を固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-105-post-wave1-route-lock-ssot.md
  - docs/guides/wasm-guide/planning/unsupported_features.md
  - src/backend/wasm/codegen/builtins.rs
  - src/backend/wasm/codegen/instructions.rs
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29cc/README.md
---

# 29cc-117 WSM-01 WASM Unsupported Inventory Sync SSOT

## 0. Goal

`WSM-01`（non-blocking parallel）の最小修正として、WASM backend の unsupported inventory を実体コードへ同期する。

1. inventory 文書を `executor.rs` 前提から現行実装へ修正
2. `BoxCall` と `ExternCall` の unsupported 診断を fail-fast かつ具体化
3. plugin lane mainline を壊さない

## 1. Boundary (fixed)

In scope:
1. `docs/guides/wasm-guide/planning/unsupported_features.md` の全面同期
2. `src/backend/wasm/codegen/builtins.rs` の unsupported BoxCall 診断改善
3. `src/backend/wasm/codegen/instructions.rs` の unsupported ExternCall 診断改善
4. `phase-29cc` pointer 同期（plugin lane next / wasm lane next）

Out of scope:
1. 新規 ExternCall 実装追加
2. 新規 BoxCall メソッド実装追加
3. wasm executor 再導入

## 2. Contract Lock

1. unsupported path は fallback しない
2. unsupported message は supported list を含む
3. inventory docs は実装ファイルパスを正本として示す

## 3. Evidence (2026-02-26)

1. `cargo check --bin hakorune` -> PASS
2. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` -> PASS
3. `bash tools/smokes/v2/profiles/integration/apps/archive/phase134_plugin_best_effort_init.sh` -> PASS
4. `bash tools/vm_plugin_smoke.sh` -> PASS

## 4. Decision

Decision: accepted

- `WSM-01`（inventory sync + fail-fast diagnostics）は完了。
- wasm lane active next は `WSM-02b`（ExternCall coverage expansion）。
