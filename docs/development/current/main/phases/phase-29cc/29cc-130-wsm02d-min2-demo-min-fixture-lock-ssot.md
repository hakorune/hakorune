---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: wasm lane `WSM-02d-min2` として nyash-wasm demo-min fixture の compile 境界を fixture/smoke で固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-129-wsm02d-min1-boundary-fastfail-tests-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-120-wasm-demo-goal-contract-ssot.md
  - apps/tests/phase29cc_wsm02d_demo_min.hako
  - tests/wasm_demo_min_fixture.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm02d_demo_min_boundary_vm.sh
  - tools/checks/dev_gate.sh
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29cc/README.md
---

# 29cc-130 WSM-02d-min2 Demo-Min Fixture Lock SSOT

## 0. Goal

`WSM-02d` の min2 として、`projects/nyash-wasm` G2 入口に対応する demo-min を fixture 化し、WASM compile 境界を gate 化する。

1. demo-min fixture を 1本追加（console family 5 method）
2. fixture -> parser -> MIR -> WAT compile 契約をテスト固定
3. lightweight gate（`wasm-boundary-lite`）へ接続

## 1. Boundary (fixed)

In scope:
1. `apps/tests/phase29cc_wsm02d_demo_min.hako` を追加
2. `tests/wasm_demo_min_fixture.rs` を追加（feature: `wasm-backend`）
3. `tools/smokes/v2/profiles/integration/apps/phase29cc_wsm02d_demo_min_boundary_vm.sh` を追加
4. `tools/checks/dev_gate.sh wasm-boundary-lite` へ統合
5. phase pointer 同期

Out of scope:
1. browser 実行（G2本体）
2. `Load` / `Store` など未対応命令の受理拡張
3. wasm executor 再導入

## 2. Contract Lock

1. demo-min fixture は parse/compile で落ちない
2. WAT に `console_log/warn/error/info/debug` import が含まれる
3. `dev_gate.sh wasm-boundary-lite` で demo-min 境界が自動検証される

## 3. Evidence (2026-02-26)

1. `cargo check --bin hakorune` -> PASS
2. `cargo check --features wasm-backend --bin hakorune` -> PASS
3. `cargo test --features wasm-backend wasm_demo_min_fixture_compile_to_wat_contract -- --nocapture` -> PASS
4. `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm02d_demo_min_boundary_vm.sh` -> PASS
5. `tools/checks/dev_gate.sh wasm-boundary-lite` -> PASS

## 4. Decision

Decision: accepted

- `WSM-02d-min2` は完了。
- wasm lane active next は `WSM-02d-min3`（demo-goal boundary gate 昇格）とする。
