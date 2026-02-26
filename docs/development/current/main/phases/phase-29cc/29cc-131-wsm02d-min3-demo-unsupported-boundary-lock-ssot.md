---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: wasm lane `WSM-02d-min3` として demo-goal scope外機能の fail-fast 境界を fixture/smoke で固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-130-wsm02d-min2-demo-min-fixture-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-120-wasm-demo-goal-contract-ssot.md
  - apps/tests/phase29cc_wsm02d_demo_unsupported_boundary_min.hako
  - tests/wasm_demo_min_fixture.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm02d_demo_unsupported_boundary_vm.sh
  - tools/checks/dev_gate.sh
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29cc/README.md
---

# 29cc-131 WSM-02d-min3 Demo Unsupported Boundary Lock SSOT

## 0. Goal

`WSM-02d` の min3 として、`projects/nyash-wasm` の scope外機能が silent fallback せず fail-fast になる契約を固定する。

1. scope外メソッド呼び出し fixture を 1本追加
2. compile-to-WAT で `Unsupported BoxCall method` を返すことを固定
3. lightweight gate（`wasm-boundary-lite`）へ接続

## 1. Boundary (fixed)

In scope:
1. `apps/tests/phase29cc_wsm02d_demo_unsupported_boundary_min.hako` を追加
2. `tests/wasm_demo_min_fixture.rs` に unsupported 境界テストを追加
3. `tools/smokes/v2/profiles/integration/apps/phase29cc_wsm02d_demo_unsupported_boundary_vm.sh` を追加
4. `tools/checks/dev_gate.sh wasm-boundary-lite` へ統合
5. phase pointer 同期

Out of scope:
1. scope外機能（`group/groupEnd/separator` など）の実装追加
2. browser runtime parity
3. wasm executor 再導入

## 2. Contract Lock

1. scope外 fixture は parse/MIR は通る
2. WASM compile で `Unsupported BoxCall method: group` を返して fail-fast する
3. エラーメッセージに supported list が含まれる

## 3. Evidence (2026-02-26)

1. `cargo test --features wasm-backend wasm_demo_unsupported_boundary_fails_fast_contract -- --nocapture` -> PASS
2. `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm02d_demo_unsupported_boundary_vm.sh` -> PASS
3. `tools/checks/dev_gate.sh wasm-boundary-lite` -> PASS

## 4. Decision

Decision: accepted

- `WSM-02d-min3` は完了。
- wasm lane active next は `WSM-02d-min4`（milestone gate昇格の lock 化）とする。
