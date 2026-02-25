Status: Active  
Date: 2025-12-26  
Scope: Phase 285 P0（docs-only）手順書。コード変更なしで “Box lifecycle / weakref / finalization / GC” の SSOT と差分運用を固定する。  
Related:
- docs/development/current/main/phases/phase-285/README.md
- docs/reference/language/lifecycle.md
- docs/reference/language/types.md

# Phase 285 P0（docs-only）: Box lifecycle / weakref / finalization / GC SSOT

目的: “実装が仕様” になっている Box の寿命・弱参照・最終化を、docs と smoke の SSOT に固定する。

## 1. このP0でやること（コード変更なし）

1) 仕様SSOTを 1 ファイルにまとめる  
   - 言語レベルの SSOT は `docs/reference/language/lifecycle.md`（lifecyle/weak/fini/GC）と `docs/reference/language/types.md`（truthiness と `null`/`void`）に集約する。
   - Phase 285 は「実装の棚卸し・差分追跡・受け入れ条件」を書く（言語SSOTを書き換えない）。

2) 用語と境界を固定する
   - strong/weak/roots/finalizer/collection の定義
   - weakref の API（weak_to_strong/生存判定）
   - finalizer の禁止事項（再入・例外・順序）

3) LLVM harness の扱いを明文化する
   - 未対応なら “未対応” を差分として書く（差分を隠さない）。
   - 差分は「仕様差」ではなく「未実装/バグ/保留」として分類する（言語SSOTは揺らさない）。

## 2. README に必ず書く事項（チェックリスト）

- [ ] “roots” は何か（stack/local/global/handle/plugin 等）
- [ ] strong/weak の意味（weak_to_strong の成否条件）
- [ ] finalizer はあるか／いつ発火するか／何が禁止か
- [ ] GC/解放のトリガ（自動/手動/閾値/テスト用）
- [ ] VM と LLVM harness の差分（未対応の場合の方針）
  - 分類: (A) 仕様通り / (B) 未実装 / (C) 既知バグ / (D) 仕様外(禁止)

追加ルール（運用）:
- [ ] 新しい環境変数トグルは増やさない（既存の診断導線の範囲で）
- [ ] 未対応は隠さず、smoke で理由付き SKIP として固定する（silent fallback 禁止）

## 3. 次（P1/P2）への導線（箇条書きでOK）

- P1（investigation）: 棚卸し対象のファイル一覧と観測ポイント
- P2（smoke）: fixture の仕様（stdout/exit）と LLVM 側の扱い（PASS/SKIP）

## 4. P1 調査チェックリスト（提案）

### Rust VM（SSOT）

- `src/value.rs`
  - `NyashValue::WeakBox` の生成箇所（weak をどう作るか）
  - `weak_to_strong()` 失敗時の観測方法（文字列化/判定API）
  - unit test: `test_weak_reference_drop` の仕様（何を固定しているか）
- `src/finalization.rs`
  - finalizer の存在（あれば: 登録、呼び出しタイミング、順序）
  - 禁止事項（再入/例外/I/O/alloc）をどこでガードするか
- `src/box_trait.rs` / `src/scope_tracker.rs`
  - Box の所有モデル（Arc/Weakの境界、roots の形成点）

### LLVM harness（差分を SSOT 化）

- `src/llvm_py/`
  - weakref/finalizer の相当機能があるか（まず “無いなら無い” を明文化）
  - 未対応の場合は smoke を SKIP にし、理由をログで固定する方針
