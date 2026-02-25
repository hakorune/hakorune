# Phase 131: JoinIR → LLVM 個別修正ライン（最小スコープ）

## 🎯 ゴール

Phase 130 で観測した LLVM 側の「赤ポイント3つ」を、**設計を崩さずピンポイントで修正** するフェーズ。

目的：
- LLVM backend の最小 re-enable（代表1本を green に）
- ConsoleBox の LLVM ライン統一（println/log 出力）
- JoinIR→MIR→LLVM の成功パス確立（JoinIR含み1本を green に）

```
Phase 130: 赤ポイント3つを「観測・リスト化」✅
        ↓
Phase 131: その3つを「ピンポイント修正」← ← ここ！
        ↓
JoinIR/LLVM 第3章クローズ準備完了
```

---

## 📋 スコープ（Phase 130 で見えた3点だけ）

Phase 130 で検出された問題：

1. **LLVM backend 機能が無効化中** → 最小 re-enable で1本 green に
2. **ConsoleBox の LLVM ライン登録・出力経路** → LLVM runtime レイヤで統一
3. **JoinIR→MIR は OK、MIR→LLVM は未検証** → JoinIR含みケース1本を成功させる

### ✅ やること
- LLVM backend の現状確認と最小 re-enable
- ConsoleBox の LLVM 出力経路整備
- JoinIR含みケース1本の LLVM 実行成功

### ❌ やらないこと
- 全7本のテストケースを green にする（1-2本で十分）
- LLVM最適化パスの追加
- 大規模な設計変更

---

## 🏗️ 4 つのタスク

### Task 1: LLVM backend の現状確認 & 最小 re-enable

**目標**: 代表1本（`peek_expr_block.hako`）を LLVM ハーネスで green にする

**やること**:

1. **LLVM backend 設定の確認**:
   ```bash
   # Cargo.toml で llvm feature の定義確認
   rg "features.*llvm" Cargo.toml

   # 現在のビルド確認
   ./target/release/nyash --version
   ```

2. **LLVM feature 付きビルド**:
   ```bash
   cargo build --release --features llvm
   ```
   - ビルドエラーがあれば記録（Phase 130 docs に追記）
   - 成功すれば次へ

3. **Python/llvmlite 環境確認**:
   ```bash
   # llvmlite ハーネス確認
   ls -la src/llvm_py/venv/

   # 無ければ再構築
   cd src/llvm_py
   python3 -m venv venv
   ./venv/bin/pip install llvmlite
   cd ../..
   ```

4. **代表1本を LLVM 実行**:
   ```bash
   # peek_expr_block.hako を LLVM で実行
   LLVM_SYS_180_PREFIX=$(llvm-config-18 --prefix) \
   NYASH_LLVM_USE_HARNESS=1 \
     ./target/release/nyash --backend llvm apps/tests/peek_expr_block.hako
   ```
   - ✅ green なら Task 1 完了
   - ❌ 赤なら、エラーメッセージを docs に記録して次の Task へ

5. **Phase 130 docs 更新**:
   - `phase130_joinir_llvm_baseline.md` の結果表を更新
   - LLVM backend の re-enable 手順を記録

---

### Task 2: ConsoleBox の LLVM ライン整合

**目標**: ConsoleBox の println/log を LLVM runtime レイヤで統一

**やること**:

1. **Rust VM での ConsoleBox 登録確認**:
   ```bash
   # TypeRegistry で ConsoleBox の println/log スロット確認
   rg "ConsoleBox.*println" src/runtime/type_registry.rs

   # Phase 122 の変更内容確認
   rg "Phase 122" docs/development/current/main/
   ```

2. **LLVM runtime での ConsoleBox 対応**:
   - 候補A: LLVM ハーネス側で print/log 外部関数を追加
   - 候補B: LLVM backend に println/log の簡易実装を追加
   - **推奨**: 候補A（Python ハーネス側での外部関数実装）

3. **Python ハーネス修正**（推奨アプローチ）:
   ```python
   # src/llvm_py/llvm_builder.py または runtime.py

   # extern_print 関数を追加（既存の print 処理を流用）
   def extern_println(msg):
       print(f"[Console LOG] {msg}")

   # LLVM IR 生成時に externCall("println", ...) を処理
   ```

4. **検証**:
   ```bash
   # esc_dirname_smoke.hako を LLVM で実行
   LLVM_SYS_180_PREFIX=$(llvm-config-18 --prefix) \
   NYASH_LLVM_USE_HARNESS=1 \
     ./target/release/nyash --backend llvm apps/tests/esc_dirname_smoke.hako
   ```
   - 期待出力: `[Console LOG] dir1/dir2`

5. **Phase 130 docs 更新**:
   - ConsoleBox の LLVM 対応状況を記録

---

### Task 3: JoinIR→MIR→LLVM の成功パス確立

**目標**: JoinIR含みケース1本を LLVM で green にする

**やること**:

1. **JoinIR含みケースの選定**:
   - 候補1: `local_tests/phase123_simple_if.hako`（シンプルな if）
   - 候補2: `apps/tests/joinir_if_select_simple.hako`（IfSelect）
   - **推奨**: `phase123_simple_if.hako`（軽量で検証しやすい）

2. **Rust VM での確認**（比較用）:
   ```bash
   ./target/release/nyash --backend vm local_tests/phase123_simple_if.hako
   # 出力: RC: 0
   ```

3. **LLVM 実行**:
   ```bash
   LLVM_SYS_180_PREFIX=$(llvm-config-18 --prefix) \
   NYASH_LLVM_USE_HARNESS=1 \
     ./target/release/nyash --backend llvm local_tests/phase123_simple_if.hako
   ```

4. **エラー対応**:
   - JoinIR → MIR 変換: ✅ Phase 130 で確認済み（問題なし）
   - MIR → LLVM IR: ここでエラーが出る可能性
     - 未対応命令（BoxCall/NewBox/PHI）があれば記録
     - 軽微な修正（1-2箇所）なら対応
     - 大規模修正が必要なら Phase 132 に回す

5. **成功条件**:
   - ✅ JoinIR含みケース1本が LLVM で実行成功
   - または
   - ⚠️ 未対応命令を特定して docs に記録（修正は Phase 132）

---

### Task 4: ドキュメント & CURRENT_TASK 更新

**やること**:

1. **phase130_joinir_llvm_baseline.md 更新**:
   ```markdown
   ## Phase 131 修正内容

   ### LLVM backend re-enable
   - ✅ `cargo build --release --features llvm` 成功
   - ✅ peek_expr_block.hako: Rust VM ✅ → LLVM ✅

   ### ConsoleBox LLVM 統合
   - ✅ Python ハーネス側で println/log 外部関数実装
   - ✅ esc_dirname_smoke.hako: Rust VM ✅ → LLVM ✅

   ### JoinIR→LLVM 成功パス
   - ✅ phase123_simple_if.hako: Rust VM ✅ → LLVM ✅
   - または
   - ⚠️ 未対応: [具体的な命令名] → Phase 132 対応予定

   ### 修正後の結果表

   | .hako ファイル | Rust VM | LLVM (Phase 130) | LLVM (Phase 131) | メモ |
   |---|---|---|---|---|
   | peek_expr_block.hako | ✅ | ❌ | ✅ | re-enable 成功 |
   | esc_dirname_smoke.hako | ❌ | ❌ | ✅ | ConsoleBox 統合 |
   | phase123_simple_if.hako | ✅ | ❌ | ✅ or ⚠️ | JoinIR パス |
   ```

2. **CURRENT_TASK.md 更新**:
   ```markdown
   ### Phase 131: JoinIR → LLVM 個別修正ライン ✅

   **完了内容**:
   - LLVM backend 最小 re-enable（peek_expr_block.hako ✅）
   - ConsoleBox LLVM 統合（esc_dirname_smoke.hako ✅）
   - JoinIR→LLVM 成功パス確立（phase123_simple_if.hako ✅ or ⚠️）

   **修正箇所**:
   - Cargo.toml: llvm feature 確認
   - src/llvm_py/*: ConsoleBox println/log 外部関数追加
   - [その他、修正したファイルを列挙]

   **テスト結果**:
   - 修正前: LLVM 0/7 実行可能
   - 修正後: LLVM 2-3/7 実行可能（最小成功パス確立）

   **成果**:
   - JoinIR → LLVM 経路の基本導線が動作確認できた
   - Phase 132 以降で残りのケースを green にする準備完了

   **次フェーズ**: Phase 132 - LLVM 未対応命令の個別対応（必要に応じて）
   ```

3. **30-Backlog.md 更新**:
   ```markdown
   ### Phase 132: LLVM 未対応命令の個別対応（必要時）

   Phase 131 で未解決の問題：
   - [Phase 131 で検出された未対応命令を列挙]
   - 例: BoxCall の特定メソッド、PHI の特殊ケース等

   または、Phase 131 で全て解決した場合：
   - ✅ JoinIR → LLVM 第3章クローズ
   - 次: selfhost Stage-4 拡張 or 次の大型改善へ
   ```

---

## ✅ 完成チェックリスト（Phase 131）

- [ ] LLVM backend が `--features llvm` でビルド成功
- [ ] Python/llvmlite 環境が構築済み
- [ ] peek_expr_block.hako が LLVM で実行成功（✅）
- [ ] esc_dirname_smoke.hako が LLVM で実行成功（✅、ConsoleBox 出力確認）
- [ ] phase123_simple_if.hako が LLVM で実行成功（✅ or ⚠️ + 問題記録）
- [ ] phase130_joinir_llvm_baseline.md に Phase 131 修正内容追記
- [ ] CURRENT_TASK.md に Phase 131 完了行追加
- [ ] 30-Backlog.md 更新（Phase 132 予告 or クローズ宣言）
- [ ] git commit で記録

---

## 所要時間

**6〜8 時間程度**

- Task 1 (LLVM backend re-enable): 2時間
- Task 2 (ConsoleBox 統合): 2時間
- Task 3 (JoinIR→LLVM 成功パス): 2〜3時間
- Task 4 (ドキュメント更新): 1時間

---

## 次のステップ

**Phase 132 or クローズ判断**:
- Phase 131 で全て解決 → JoinIR→LLVM 第3章クローズ
- Phase 131 で未解決問題あり → Phase 132 で個別対応

---

## 進捗

- ✅ Phase 130: JoinIR → LLVM ベースライン確立（完了）
- 🎯 Phase 131: JoinIR → LLVM 個別修正ライン（← **現在のフェーズ**）
- 📋 Phase 132: LLVM 未対応命令の個別対応（必要に応じて）
Status: Historical
