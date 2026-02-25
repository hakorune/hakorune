# Phase 153: hako_check / dead code 検出モードの復活（JoinIR 版）

## 0. ゴール

**hako_check を JoinIR 専用パイプラインの上で安定稼働させ、以前できていた「未使用関数・デッドコード検出」モードを、今の構造に合わせて復活・整備する。**

目的：
- selfhost / .hako 側のコンパイラコードに対して、hako_check で死んだ関数・到達しない分岐をすぐ見つけられる状態を作る
- Phase 160+ の .hako JoinIR/MIR 移植に入ったときの安全ネットを確立する

---

## 1. Scope / Non-scope

### ✅ やること

1. **hako_check の現状インベントリ**
   - CLI スクリプト (`tools/hako_check.sh` / `tools/hako_check/*.hako`) と実行経路を確認
   - 「何をチェックしているか」「以前動いていた dead code モードが今どうなっているか」を整理

2. **JoinIR 専用 hako_check パイプラインの再確認**
   - Phase 121–124 の設計どおり、AST→JoinIR→MIR→解析が一つの線になっているかを確認・doc 固定

3. **dead code 検出モードの再実装／復活（箱化モジュール化）**
   - 「未呼び出しの関数」「reachable ではない cfg ノード」などの検出を、今の JoinIR/MIR 情報から計算
   - 結果を CLI から見やすい形（テキスト）で出力
   - **DeadCodeAnalyzerBox** として箱化

4. **テストとスモーク**
   - 小さい .hako フィクスチャで「明らかに死んでいる関数」が検出できることを確認
   - 既存の hako_check テストが退行していないことを確認

### ❌ やらないこと

- JoinIR / MIR の意味論を変えない（解析は「読むだけ」）
- 新しい Stage‑3 構文を追加しない（Phase 152-A/B で決めた範囲のまま）
- 環境変数を増やさない（CLI フラグ `--dead-code` のみ。必要になったら後で追加）

---

## 2. Task 1: hako_check 現状インベントリ

### 対象ファイル

- `tools/hako_check.sh`
- `tools/hako_check/*.hako`（CLI エントリ・本体）
- `docs/development/` 内の hako_check 関連 doc：
  - `hako_check_design.md`
  - `phase121_*`
  - `phase124_hako_check_joinir_finalization.md`

### やること

1. **hako_check の実行フローを再確認**
   - CLI → .hako スクリプト → VM → MirBuilder / JoinIR → 解析、という線を図にする

2. **以前の dead code モードの存在を調査**
   - 関数名に `dead` / `unused` / `reachability` 等が含まれる Box や関数を探す
   - 設計 doc に「dead code モード」がどう書かれていたかを確認

3. **結果をドキュメントにまとめる**
   - `docs/development/current/main/phase153_hako_check_inventory.md`（新規）に：
     - エントリポイント
     - 現在有効なチェック種類
     - 死んでいる / 途中まで実装されていた dead code 検出の一覧
   を書き出す

### 成果物

- `phase153_hako_check_inventory.md` 作成

---

## 3. Task 2: JoinIR 専用パイプラインの再確認と固定

### 目的

Phase 123–124 で「hako_check を JoinIR 専用にした」設計が、実際のコードでも守られているか確認

### やること

1. MirBuilder / JoinIR lowering で、hako_check 用の関数がどのルートを通っているかを確認

2. もしまだ legacy PHI / 旧 LoopBuilder にフォールバックしている箇所があれば：
   - hako_check 経路だけでも確実に JoinIR 経由になるように修正する案を設計（実装は別フェーズでも可）

3. `hako_check_design.md` / `phase121_integration_roadmap.md` に：
   - AST→JoinIR→MIR→hako_check の経路を最終形として追記

### 成果物

- パイプライン確認結果の記録（インベントリ doc に追記でも可）

---

## 4. Task 3: dead code 検出モードの設計（箱化モジュール化）

### 目的

hako_check 内で dead code 判定を行う「箱」を 1 個に閉じ込める

### 方針

- JoinIR/MIR の CFG/シンボル情報を読んで：
  - 入口（エントリポイント関数）からの到達可能性を DFS/BFS
  - 到達しなかった関数・ブロック・分岐を列挙

### 箱単位の設計

**DeadCodeAnalyzerBox** として：
- 入力: Program JSON v0 / MIR JSON / JoinIR JSON
- 出力: 「未使用関数名」「未到達ブロックID」のリスト

### やること

1. どのフォーマットを入力にするか決める
   - 解析専用なら MIR JSON v0 / JoinIR JSON のどちらか
   - 既存の hako_check 本体がどちらを見やすいかも考える

2. Dead code 箱の API を決める（.hako レベルの関数シグネチャ）

3. 出力フォーマット（CLI 表示）のラフ案：
   ```
   unused function: Foo.bar/1
   unreachable block: fn=Main.main bb=10 (if false branch)
   ```

### 成果物

- DeadCodeAnalyzerBox の設計（API シグネチャ）
- 入力フォーマット決定

---

## 5. Task 4: dead code 検出モードの実装と CLI 統合

### 実装

- hako_check 本体 .hako に **DeadCodeAnalyzerBox** を追加
- 入口の CLI から、`--dead-code` フラグで dead code モードを選べるようにする

### CLI 挙動

```bash
# 従来の基本チェック
hako_check target.hako

# dead code レポートを追加表示
hako_check --dead-code target.hako
```

### 環境変数について

- **Phase 153 では ENV 追加なし**（CLI フラグのみ）
- 将来「CI からフラグを渡しにくい」等のニーズが出たら、そのときに `NYASH_HAKO_CHECK_DEAD=1` を 1 個だけ足す

### 成果物

- DeadCodeAnalyzerBox の実装
- hako_check CLI に `--dead-code` フラグ追加

---

## 6. Task 5: テストとスモーク

### テスト用 .hako を 2〜3 本作る

1. **単純に未使用関数 1 個だけあるケース**
2. **入口から到達しない if/loop 分岐を含むケース**
3. **selfhost 関連の .hako から代表 1 本**（小さなコンパイラ部品）を選び、未使用関数が検出できるか見る

### スモークスクリプト

- `tools/hako_check_deadcode_smoke.sh`（など）を作成し：
  - 上のテスト .hako をまとめて実行
  - 期待する dead code 行が出ているかを軽く grep

### JoinIR 依存の確認

- 少なくとも代表 1 本は JoinIR lowering → MIR → hako_check まで通し、
  Rust VM と hako_check の結果が論理的に矛盾していないことを確認

### 成果物

- テスト .hako 2〜3 本
- `tools/hako_check_deadcode_smoke.sh`

---

## 7. Task 6: ドキュメント / CURRENT_TASK 更新

### ドキュメント更新

1. **phase153_hako_check_inventory.md** に：
   - どのモードがあり、今どこまで動くかを確定版として追記

2. **hako_check_design.md** を更新：
   - 新しい dead code モードの説明と CLI/API を追加

3. **CURRENT_TASK.md**：
   - Phase 153 セクションを追加し、以下を 2〜3 行で書く：
     - 「hako_check が JoinIR 専用パイプラインで動き、dead code モードが復活した」
     - 「これを .hako JoinIR/MIR 移植時の安全ネットとして使う」

### 成果物

- 各種ドキュメント更新
- git commit

---

## ✅ 完成チェックリスト（Phase 153）

- [ ] Task 1: hako_check 現状インベントリ完了
  - [ ] phase153_hako_check_inventory.md 作成
- [ ] Task 2: JoinIR 専用パイプライン確認
  - [ ] パイプライン図確定・記録
- [ ] Task 3: DeadCodeAnalyzerBox 設計
  - [ ] API シグネチャ決定
  - [ ] 入力フォーマット決定
- [ ] Task 4: 実装と CLI 統合
  - [ ] DeadCodeAnalyzerBox 実装
  - [ ] `--dead-code` フラグ追加
- [ ] Task 5: テストとスモーク
  - [ ] テスト .hako 作成（2〜3 本）
  - [ ] hako_check_deadcode_smoke.sh 作成
- [ ] Task 6: ドキュメント更新
  - [ ] phase153_hako_check_inventory.md 確定版
  - [ ] hako_check_design.md 更新
  - [ ] CURRENT_TASK.md 更新
  - [ ] git commit

---

## 次のステップ

Phase 153 完了後：
- **Phase 160+**: .hako JoinIR/MIR 移植章（hako_check が安全ネットとして機能）
- **Phase 200+**: Python → Hakorune トランスパイラ構想

---

**作成日**: 2025-12-04
**Phase**: 153（hako_check / dead code 検出モードの復活）
Status: Historical
