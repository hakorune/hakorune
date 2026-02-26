---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: wasm lane `WSM-02b-min1` として `env.console.warn` の ExternCall coverage を最小追加し、fail-fast 境界を維持する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-120-wasm-demo-goal-contract-ssot.md
  - docs/guides/wasm-guide/planning/unsupported_features.md
  - src/backend/wasm/codegen/instructions.rs
  - src/backend/wasm/runtime.rs
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29cc/README.md
---

# 29cc-121 WSM-02b-min1 Console Warn Extern SSOT

## 0. Goal

`WSM-02b` の最小1件として、`env.console.warn` を WASM backend で受理する。

1. ExternCall name mapping を追加
2. Runtime import + browser JS import object を追加
3. unsupported fallback は増やさず fail-fast 方針を維持

## 1. Boundary (fixed)

In scope:
1. `src/backend/wasm/codegen/instructions.rs` の ExternCall mapping 追加（`env.console.warn` -> `console_warn`）
2. `src/backend/wasm/runtime.rs` の import 定義と JS glue 追加
3. unsupported inventory と phase pointer 同期

Out of scope:
1. `env.console.error` など他 extern family
2. BoxCall coverage 拡張（WSM-02c scope）
3. `Load` / `Store` 対応（WSM-02d以降）

## 2. Contract Lock

1. `env.console.warn` は `(ptr, len)` の2引数契約で `console_warn` import に接続する
2. unsupported extern は従来通り `Unsupported extern call: ...` で fail-fast
3. `console.warn` JS bridge を通じて browser runtime で可視化可能にする

## 3. Evidence (2026-02-26)

1. `cargo check --bin hakorune` -> PASS
2. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` -> PASS
3. `bash tools/smokes/v2/profiles/integration/apps/archive/phase134_plugin_best_effort_init.sh` -> PASS
4. `bash tools/vm_plugin_smoke.sh` -> PASS

## 4. Decision

Decision: accepted

- `WSM-02b-min1` は完了。
- wasm lane active next は `WSM-02b-min2`（次 extern family 1件）とする。
