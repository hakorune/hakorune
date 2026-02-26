---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: wasm lane `WSM-02d-min1` として supported/unsupported 境界の fail-fast テストを固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-128-wsm02c-min4-boxcall-console-error-ssot.md
  - src/backend/wasm/extern_contract.rs
  - src/backend/wasm/codegen/tests.rs
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29cc/README.md
---

# 29cc-129 WSM-02d-min1 Boundary Fast-Fail Tests SSOT

## 0. Goal

`WSM-02d` の最小1件として、WASM lane の supported/unsupported 境界を unit test で固定する。

1. ExternCall unsupported が fail-fast になることを固定
2. BoxCall unsupported が fail-fast になることを固定
3. extern contract table の supported map をテスト固定

## 1. Boundary (fixed)

In scope:
1. `src/backend/wasm/extern_contract.rs` に境界ユニットテストを追加
2. `src/backend/wasm/codegen/tests.rs` に unsupported 境界テストを追加
3. phase pointer 同期

Out of scope:
1. wasm demo fixture (`WSM-02d-min2`)
2. runtime/import 実装面の追加変更
3. parser/MIR受理範囲の拡張

## 2. Contract Lock

1. unsupported extern は `Unsupported extern call: ...` で fail-fast
2. unsupported BoxCall method は `Unsupported BoxCall method: ...` で fail-fast
3. supported 一覧は message に含まれる（診断距離短縮）

## 3. Evidence (2026-02-26)

1. `cargo check --bin hakorune` -> PASS
2. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` -> PASS
3. `bash tools/smokes/v2/profiles/integration/apps/archive/phase134_plugin_best_effort_init.sh` -> PASS
4. `bash tools/vm_plugin_smoke.sh` -> PASS
5. `cargo test --features wasm-backend ...` は既存の wasm feature 側 compile blocker（runner/benchmarks）により現時点では gate 外運用（別途修正レーンで扱う）

## 4. Decision

Decision: accepted

- `WSM-02d-min1` は完了。
- wasm lane active next は `WSM-02d-min2`（demo-min fixture lock）とする。
