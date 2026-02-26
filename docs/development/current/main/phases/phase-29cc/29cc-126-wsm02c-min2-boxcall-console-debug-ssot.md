---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: wasm lane `WSM-02c-min2` として BoxCall `debug` を最小追加し、fail-fast 境界を維持する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-125-wsm02c-min1-boxcall-console-info-ssot.md
  - docs/guides/wasm-guide/planning/unsupported_features.md
  - src/backend/wasm/codegen/builtins.rs
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29cc/README.md
---

# 29cc-126 WSM-02c-min2 BoxCall Console Debug SSOT

## 0. Goal

`WSM-02c` の最小2件目として、BoxCall `debug` を WASM backend で受理する。

1. `builtins.rs` の method dispatch に `debug` を追加
2. `console_debug` import 呼び出しへ接続
3. unsupported fallback は増やさず fail-fast 方針を維持

## 1. Boundary (fixed)

In scope:
1. `src/backend/wasm/codegen/builtins.rs` に `debug` method routing を追加
2. 既存 console-call helper の共有を維持
3. unsupported inventory と phase pointer 同期

Out of scope:
1. BoxCall `warn` 拡張（`WSM-02c-min3+`）
2. ExternCall coverage 拡張（`WSM-02b`）
3. `Load` / `Store` 対応（`WSM-02d` 以降）

## 2. Contract Lock

1. BoxCall `debug` は `call $console_debug` で lower する
2. BoxCall unsupported は `Unsupported BoxCall method: ...` で fail-fast
3. 既存 `log/info` 経路の挙動は維持する

## 3. Evidence (2026-02-26)

1. `cargo check --bin hakorune` -> PASS
2. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` -> PASS
3. `bash tools/smokes/v2/profiles/integration/apps/archive/phase134_plugin_best_effort_init.sh` -> PASS
4. `bash tools/vm_plugin_smoke.sh` -> PASS

## 4. Decision

Decision: accepted

- `WSM-02c-min2` は完了。
- wasm lane active next は `WSM-02c-min3`（BoxCall console family の次1件）とする。
