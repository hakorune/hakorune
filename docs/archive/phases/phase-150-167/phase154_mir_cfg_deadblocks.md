# Phase 154: MIR CFG 統合 & ブロックレベル unreachable 検出

## 0. ゴール

**hako_check に MIR CFG 情報を取り込み、「到達不能な basic block」を検出する HC020 ルールを追加する。**

目的：
- Phase 153 で復活した dead code 検出（メソッド・Box 単位）を、ブロック単位まで細粒度化
- JoinIR/MIR の CFG 情報を hako_check の Analysis IR に統合
- 「unreachable basic block」を検出し、コード品質向上に寄与

---

## 1. Scope / Non-scope

### ✅ やること

1. **MIR/CFG 情報のインベントリ**
   - 現在の MIR JSON v0 に含まれる CFG 情報（blocks, terminators）を確認
   - hako_check の Analysis IR に追加すべきフィールドを特定

2. **DeadBlockAnalyzerBox の設計（箱化モジュール化）**
   - Phase 153 の DeadCodeAnalyzerBox パターンを踏襲
   - 入力: Analysis IR（CFG 情報付き）
   - 出力: 未到達ブロックのリスト

3. **hako_check パイプラインへの統合設計**
   - Analysis IR 生成時に CFG 情報を含める方法を決定
   - HC020 ルールの位置付け（HC019 の後に実行）

4. **テストケース設計（ブロックレベル）**
   - 到達不能な if/else 分岐
   - 早期 return 後のコード
   - 常に false のループ条件

5. **実装 & テスト**
   - DeadBlockAnalyzerBox 実装
   - HC020 ルール実装
   - スモークテスト作成

6. **ドキュメント & CURRENT_TASK 更新**

### ❌ やらないこと

- JoinIR/MIR の意味論を変えない（解析は「読むだけ」）
- 新しい Stage-3 構文を追加しない
- 環境変数を増やさない（CLI フラグ `--dead-blocks` のみ）

---

## 2. Task 1: MIR/CFG 情報のインベントリ

### 対象ファイル

- `src/mir/join_ir/json.rs` - JoinIR JSON シリアライズ
- `src/mir/join_ir_runner.rs` - JoinIR 実行
- `src/mir/` - MIR 構造定義
- `tools/hako_check/analysis_ir.hako` - 現在の Analysis IR 定義

### やること

1. **MIR JSON v0 の CFG 情報を確認**
   - blocks 配列の構造
   - terminator の種類（Jump, Branch, Return）
   - predecessors / successors の有無

2. **Analysis IR に追加すべきフィールドを特定**
   - `blocks: Array<BlockInfo>` ?
   - `cfg_edges: Array<Edge>` ?
   - `entry_block: BlockId` ?

3. **JoinIR Strict モードでの動作確認**
   - `NYASH_JOINIR_STRICT=1` で MIR が正しく生成されているか
   - Phase 150 の代表ケースで CFG 情報が取れるか

### 成果物

- CFG 情報インベントリ結果の記録

---

## 3. Task 2: DeadBlockAnalyzerBox の設計（箱化モジュール化）

### 目的

Phase 153 の DeadCodeAnalyzerBox パターンを踏襲し、ブロックレベル解析を箱化

### 方針

- エントリブロックからの到達可能性を DFS/BFS で計算
- 到達しなかったブロックを列挙
- 各ブロックがどの関数に属するかも記録

### 箱単位の設計

**DeadBlockAnalyzerBox** として：
- 入力: Analysis IR（CFG 情報付き）
- 出力: 「未到達ブロック」のリスト

### API シグネチャ案

```hako
static box DeadBlockAnalyzerBox {
  method apply_ir(ir, path, out) {
    // CFG 情報を取得
    local blocks = ir.get("blocks")
    local edges = ir.get("cfg_edges")
    local entry = ir.get("entry_block")

    // 到達可能性解析
    local reachable = me._compute_reachability(entry, edges)

    // 未到達ブロックを検出
    me._report_unreachable_blocks(blocks, reachable, path, out)
  }

  method _compute_reachability(entry, edges) {
    // DFS/BFS で到達可能なブロックを収集
    // return: Set<BlockId>
  }

  method _report_unreachable_blocks(blocks, reachable, path, out) {
    // 到達不能なブロックを HC020 として報告
  }
}
```

### 出力フォーマット

```
[HC020] Unreachable basic block: fn=Main.main bb=10 (after early return)
[HC020] Unreachable basic block: fn=Foo.bar bb=15 (if false branch never taken)
```

### 成果物

- DeadBlockAnalyzerBox の設計（API シグネチャ）
- Analysis IR 拡張フィールド決定

---

## 4. Task 3: hako_check パイプラインへの統合設計

### 目的

HC020 ルールを既存の hako_check パイプラインに統合

### やること

1. **Analysis IR 生成の拡張**
   - `tools/hako_check/analysis_ir.hako` を拡張
   - CFG 情報（blocks, edges, entry_block）を含める

2. **CLI フラグ追加**
   - `--dead-blocks` フラグで HC020 を有効化
   - または `--dead-code` に統合（ブロックレベルも含む）

3. **ルール実行順序**
   - HC019（dead code）の後に HC020（dead blocks）を実行
   - または `--rules dead_blocks` で個別指定可能に

### 設計方針

**Option A**: `--dead-code` に統合
```bash
# HC019 + HC020 を両方実行
./tools/hako_check.sh --dead-code target.hako
```

**Option B**: 別フラグ
```bash
# HC019 のみ
./tools/hako_check.sh --dead-code target.hako

# HC020 のみ
./tools/hako_check.sh --dead-blocks target.hako

# 両方
./tools/hako_check.sh --dead-code --dead-blocks target.hako
```

**推奨**: Option A（ユーザーは「dead code」を広義に捉えるため）

### 成果物

- パイプライン統合設計
- CLI フラグ仕様確定

---

## 5. Task 4: テストケース設計（ブロックレベル）

### テストケース一覧

#### Case 1: 早期 return 後のコード
```hako
static box TestEarlyReturn {
  test(x) {
    if x > 0 {
      return 1
    }
    // ここに到達不能コード
    local unreachable = 42  // HC020 検出対象
    return unreachable
  }
}
```

#### Case 2: 常に false の条件
```hako
static box TestAlwaysFalse {
  test() {
    if false {
      // このブロック全体が到達不能
      return 999  // HC020 検出対象
    }
    return 0
  }
}
```

#### Case 3: 無限ループ後のコード
```hako
static box TestInfiniteLoop {
  test() {
    loop(true) {
      // 無限ループ
    }
    // ここに到達不能
    return 0  // HC020 検出対象
  }
}
```

#### Case 4: break 後のコード（ループ内）
```hako
static box TestAfterBreak {
  test() {
    loop(true) {
      break
      // break 後のコード
      local x = 1  // HC020 検出対象
    }
    return 0
  }
}
```

### 成果物

- テスト .hako ファイル 4 本
- 期待される HC020 出力の定義

---

## 6. Task 5: 実装 & テスト

### 実装ファイル

1. **`tools/hako_check/rules/rule_dead_blocks.hako`** - 新規作成
   - DeadBlockAnalyzerBox 実装
   - HC020 ルール実装

2. **`tools/hako_check/analysis_ir.hako`** - 拡張
   - CFG 情報フィールド追加

3. **`tools/hako_check/cli.hako`** - 修正
   - `--dead-blocks` または `--dead-code` 拡張
   - HC020 実行統合

### テストファイル

1. **`apps/tests/hako_check/test_dead_blocks_early_return.hako`**
2. **`apps/tests/hako_check/test_dead_blocks_always_false.hako`**
3. **`apps/tests/hako_check/test_dead_blocks_infinite_loop.hako`**
4. **`apps/tests/hako_check/test_dead_blocks_after_break.hako`**

### スモークスクリプト

- `tools/hako_check_deadblocks_smoke.sh` - HC020 スモークテスト

### 成果物

- DeadBlockAnalyzerBox 実装
- HC020 ルール実装
- テストケース 4 本
- スモークスクリプト

---

## 7. Task 6: ドキュメント & CURRENT_TASK 更新

### ドキュメント更新

1. **phase154_mir_cfg_deadblocks.md** に：
   - 実装結果を記録
   - CFG 統合の最終設計

2. **hako_check_design.md** を更新：
   - HC020 ルールの説明
   - CFG 解析機能の説明

3. **CURRENT_TASK.md**：
   - Phase 154 セクションを追加

4. **CLAUDE.md**：
   - hako_check ワークフローに `--dead-blocks` 追記（必要なら）

### 成果物

- 各種ドキュメント更新
- git commit

---

## ✅ 完成チェックリスト（Phase 154）

- [ ] Task 1: MIR/CFG 情報インベントリ完了
  - [ ] CFG 構造確認
  - [ ] Analysis IR 拡張フィールド決定
- [ ] Task 2: DeadBlockAnalyzerBox 設計
  - [ ] API シグネチャ決定
  - [ ] 到達可能性アルゴリズム決定
- [ ] Task 3: パイプライン統合設計
  - [ ] CLI フラグ仕様確定
  - [ ] ルール実行順序確定
- [ ] Task 4: テストケース設計
  - [ ] テスト .hako 4 本設計
- [ ] Task 5: 実装 & テスト
  - [ ] DeadBlockAnalyzerBox 実装
  - [ ] HC020 ルール実装
  - [ ] テストケース実装
  - [ ] スモークスクリプト作成
- [ ] Task 6: ドキュメント更新
  - [ ] phase154_mir_cfg_deadblocks.md 確定版
  - [ ] hako_check_design.md 更新
  - [ ] CURRENT_TASK.md 更新
  - [ ] git commit

---

## 技術的考慮事項

### JoinIR Strict モードとの整合性

Phase 150 で確認済みの代表ケースで CFG 情報が取れることを確認：
- `peek_expr_block.hako` - match 式、ブロック式
- `loop_min_while.hako` - ループ変数、Entry/Exit PHI
- `joinir_min_loop.hako` - break 制御
- `joinir_if_select_simple.hako` - 早期 return

### Analysis IR の CFG 拡張案

```json
{
  "methods": [...],
  "calls": [...],
  "boxes": [...],
  "entrypoints": [...],
  "cfg": {
    "functions": [
      {
        "name": "Main.main",
        "entry_block": 0,
        "blocks": [
          {"id": 0, "successors": [1, 2], "terminator": "Branch"},
          {"id": 1, "successors": [3], "terminator": "Jump"},
          {"id": 2, "successors": [3], "terminator": "Jump"},
          {"id": 3, "successors": [], "terminator": "Return"}
        ]
      }
    ]
  }
}
```

---

## 次のステップ

Phase 154 完了後：
- **Phase 155+**: より高度な解析（定数畳み込み、型推論など）
- **Phase 160+**: .hako JoinIR/MIR 移植章

---

**作成日**: 2025-12-04
**Phase**: 154（MIR CFG 統合 & ブロックレベル unreachable 検出）
Status: Historical
