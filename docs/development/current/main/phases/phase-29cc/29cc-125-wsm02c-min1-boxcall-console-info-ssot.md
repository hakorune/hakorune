---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: wasm lane `WSM-02c-min1` として BoxCall `info` を最小追加し、fail-fast 境界を維持する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-124-wsm02b-min4-console-debug-extern-ssot.md
  - docs/guides/wasm-guide/planning/unsupported_features.md
  - src/backend/wasm/codegen/builtins.rs
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29cc/README.md
---

# 29cc-125 WSM-02c-min1 BoxCall Console Info SSOT

## 0. Goal

`WSM-02c` の最小1件として、BoxCall `info` を WASM backend で受理する。

1. `builtins.rs` の method dispatch に `info` を追加
2. `console_info` import 呼び出しへ接続
3. unsupported fallback は増やさず fail-fast 方針を維持

## 1. Boundary (fixed)

In scope:
1. `src/backend/wasm/codegen/builtins.rs` に `info` method routing を追加
2. 既存 `log` と共有できる console-call helper に集約
3. unsupported inventory と phase pointer 同期

Out of scope:
1. BoxCall `debug` / `warn` 拡張（`WSM-02c-min2+`）
2. ExternCall coverage 拡張（`WSM-02b`）
3. `Load` / `Store` 対応（`WSM-02d` 以降）

## 2. Contract Lock

1. BoxCall `info` は `call $console_info` で lower する
2. BoxCall unsupported は `Unsupported BoxCall method: ...` で fail-fast
3. 既存 `log` 経路の挙動は維持する

## 3. Evidence (2026-02-26)

1. `cargo check --bin hakorune` -> PASS
2. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` -> PASS
3. `bash tools/smokes/v2/profiles/integration/apps/archive/phase134_plugin_best_effort_init.sh` -> PASS
4. `bash tools/vm_plugin_smoke.sh` -> PASS

## 4. Decision

Decision: accepted

- `WSM-02c-min1` は完了。
- wasm lane active next は `WSM-02c-min2`（BoxCall console family の次1件）とする。
