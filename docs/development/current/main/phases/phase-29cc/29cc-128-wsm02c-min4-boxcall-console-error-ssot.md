---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: wasm lane `WSM-02c-min4` として BoxCall `error` を最小追加し、fail-fast 境界を維持する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-127-wsm02c-min3-boxcall-console-warn-ssot.md
  - docs/guides/wasm-guide/planning/unsupported_features.md
  - src/backend/wasm/codegen/builtins.rs
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29cc/README.md
---

# 29cc-128 WSM-02c-min4 BoxCall Console Error SSOT

## 0. Goal

`WSM-02c` の最小4件目として、BoxCall `error` を WASM backend で受理する。

1. `builtins.rs` の method dispatch に `error` を追加
2. `console_error` import 呼び出しへ接続
3. unsupported fallback は増やさず fail-fast 方針を維持

## 1. Boundary (fixed)

In scope:
1. `src/backend/wasm/codegen/builtins.rs` に `error` method routing を追加
2. 既存 console-call helper の共有を維持
3. unsupported inventory と phase pointer 同期

Out of scope:
1. BoxCall console family 以外の追加
2. ExternCall coverage 拡張（`WSM-02b`）
3. `Load` / `Store` 対応（`WSM-02d` 以降）

## 2. Contract Lock

1. BoxCall `error` は `call $console_error` で lower する
2. BoxCall unsupported は `Unsupported BoxCall method: ...` で fail-fast
3. 既存 `log/info/debug/warn` 経路の挙動は維持する

## 3. Evidence (2026-02-26)

1. `cargo check --bin hakorune` -> PASS
2. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` -> PASS
3. `bash tools/smokes/v2/profiles/integration/apps/archive/phase134_plugin_best_effort_init.sh` -> PASS
4. `bash tools/vm_plugin_smoke.sh` -> PASS

## 4. Decision

Decision: accepted

- `WSM-02c-min4` は完了。
- wasm lane active next は `WSM-02d-min1`（boundary gates）とする。
