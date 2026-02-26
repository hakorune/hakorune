---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: wasm lane `WSM-02a` として assignment/local path の最小 unblock（Copy/ReleaseStrong/KeepAlive）を固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-118-wasm-grammar-compat-map-ssot.md
  - docs/guides/wasm-guide/planning/unsupported_features.md
  - src/backend/wasm/codegen/instructions.rs
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29cc/README.md
---

# 29cc-119 WSM-02a Assignment/Local Unblock SSOT

## 0. Goal

`WSM-02a` の最小目標として、assignment/local 経路で頻出する MIR 命令を WASM backend で受理する。

1. `Copy` を実lower
2. `ReleaseStrong` と `KeepAlive` を no-op lower
3. unsupported fallback を増やさず fail-fast 方針を維持

## 1. Boundary (fixed)

In scope:
1. `src/backend/wasm/codegen/instructions.rs` の `Copy` 対応
2. `src/backend/wasm/codegen/instructions.rs` の `ReleaseStrong`/`KeepAlive` no-op 対応
3. docs/pointer 同期（WSM-02a done, WSM-02b active）

Out of scope:
1. `Load`/`Store` 対応
2. ExternCall/BoxCall の対応面拡張
3. scheduler/thread semantics の導入

## 2. Contract Lock

1. `Copy` は `local.get src` -> `local.set dst` で lowering
2. `ReleaseStrong`/`KeepAlive` は現行 WASM runtime 前提で no-op
3. その他未対応命令は引き続き fail-fast（`UnsupportedInstruction`）

## 3. Evidence (2026-02-26)

1. `cargo check --bin hakorune` -> PASS
2. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` -> PASS
3. `bash tools/smokes/v2/profiles/integration/apps/archive/phase134_plugin_best_effort_init.sh` -> PASS
4. `bash tools/vm_plugin_smoke.sh` -> PASS

## 4. Decision

Decision: accepted

- `WSM-02a` は完了。
- wasm lane active next は `WSM-02b`（ExternCall coverage expansion）。
