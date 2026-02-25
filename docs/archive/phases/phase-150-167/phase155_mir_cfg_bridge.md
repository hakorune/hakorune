# Phase 155: MIR CFG データブリッジ実装

## 0. ゴール

**Phase 154 で設計した DeadBlockAnalyzerBox を、実際に MIR CFG データで動かすための「データブリッジ」を実装する。**

目的：
- Rust MIR → Analysis IR へ CFG データを抽出・変換
- `extract_mir_cfg()` builtin 関数を実装
- HC020（unreachable basic block 検出）を完全に動作させる

---

## 実装状況 (2025-12-04)

### ✅ 完了項目

1. **MIR JSON への CFG 追加** (Phase 155-1)
   - `src/runner/mir_json_emit.rs` を修正
   - `extract_cfg_info()` を MIR JSON 出力時に呼び出し
   - CFG データを JSON の `cfg` フィールドとして出力
   - v0/v1 両フォーマット対応

2. **Analysis IR への CFG フィールド追加** (Phase 155-2 MVP)
   - `tools/hako_check/analysis_consumer.hako` を修正
   - 空の CFG 構造体を Analysis IR に追加（暫定実装）
   - DeadBlockAnalyzerBox が `ir.get("cfg")` で CFG にアクセス可能

### 🔄 未完了項目（今後の実装）

3. **実際の CFG データ連携**
   - MIR JSON から CFG を読み込む処理が未実装
   - 現在は空の CFG 構造体のみ（ブロック情報なし）
   - HC020 はスキップされる（CFG functions が空のため）

4. **builtin 関数の実装**
   - `extract_mir_cfg()` builtin 関数は未実装
   - Phase 155 指示書では builtin 関数経由を想定
   - 現状では Rust 側で CFG を MIR JSON に含めるのみ

---

## 1. 背景：Phase 154 の現状

### 何が完了したか

- ✅ DeadBlockAnalyzerBox (HC020 ルール)
- ✅ CLI フラグ `--dead-blocks`
- ✅ テストケース 4 本
- ✅ スモークスクリプト

### 何が残っているか（このフェーズ）

- 🔄 **CFG データブリッジ**
  - MIR JSON から CFG 情報を抽出
  - Analysis IR の `cfg` フィールドに追加
  - `.hako` コード内で呼び出し可能にする

---

## 2. Scope / Non-scope

### ✅ やること

1. **Rust 側：CFG 抽出機能**
   - `src/mir/cfg_extractor.rs` からの CFG 抽出（既に Phase 154 で作成済み）
   - `extract_mir_cfg()` builtin 関数を作成
   - JSON シリアライズ対応

2. **.hako 側：Analysis IR 拡張**
   - `tools/hako_check/analysis_ir.hako` を拡張
   - `cfg` フィールドを Analysis IR に追加
   - `analysis_consumer.hako` から `extract_mir_cfg()` を呼び出し

3. **CLI 統合**
   - hako_check の `--dead-blocks` フラグで HC020 実行時に CFG が利用される
   - スモークテストで HC020 出力を確認

4. **テスト & 検証**
   - Phase 154 の 4 テストケースすべてで HC020 出力確認
   - スモークスクリプト成功確認

### ❌ やらないこと

- Phase 154 の HC020 ロジック修正（既に完成）
- 新しい解析ルール追加（Phase 156+ へ）
- CFG 可視化（DOT 出力など）

---

## 3. 技術概要

### 3.1 データフロー

```
MIR JSON (Phase 154 作成済み)
    ↓
extract_mir_cfg() builtin (Rust)  ← このフェーズで実装
    ↓
cfg: { functions: [...] } (JSON)
    ↓
analysis_consumer.hako (呼び出し側)
    ↓
Analysis IR (cfg フィールド付き)
    ↓
DeadBlockAnalyzerBox (HC020)
    ↓
HC020 出力
```

### 3.2 Analysis IR 拡張案

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
          {
            "id": 0,
            "successors": [1, 2],
            "terminator": "Branch"
          },
          {
            "id": 1,
            "successors": [3],
            "terminator": "Jump"
          },
          {
            "id": 2,
            "successors": [3],
            "terminator": "Jump"
          },
          {
            "id": 3,
            "successors": [],
            "terminator": "Return"
          }
        ]
      }
    ]
  }
}
```

---

## 4. Task 1: extract_mir_cfg() builtin 関数実装

### 対象ファイル

- `src/mir/cfg_extractor.rs` - 既存（Phase 154 作成済み）
- `src/runtime/builtin_functions.rs` または `src/runtime/builtin_registry.rs` - builtin 登録
- `src/mir/mod.rs` - モジュール露出

### やること

1. **extract_mir_cfg() 関数を実装**
   - 入力：MIR Function オブジェクト
   - 出力：CFG JSON オブジェクト
   - 実装例：
     ```rust
     pub fn extract_mir_cfg(function: &MirFunction) -> serde_json::Value {
         let blocks: Vec<_> = function.blocks.values().map(|block| {
             serde_json::json!({
                 "id": block.id.0,
                 "successors": get_successors(block),
                 "terminator": format!("{:?}", block.terminator)
             })
         }).collect();

         serde_json::json!({
             "name": "...",  // 関数名は別途指定
             "entry_block": 0,
             "blocks": blocks
         })
     }
     ```

2. **Builtin Registry に登録**
   - 関数シグネチャ：`extract_mir_cfg(mir_json: Object) -> Object`
   - JoinIR ビルダーから呼び出し可能に

3. **テスト**
   - 単体テスト作成：`test_extract_mir_cfg_simple()`
   - 複数ブロック、分岐、ループ対応確認

### 成果物

- `extract_mir_cfg()` builtin 実装
- Builtin 登録完了
- ユニットテスト

---

## 5. Task 2: analysis_consumer.hako 修正

### 対象ファイル

- `tools/hako_check/analysis_consumer.hako`

### やること

1. **MIR JSON を受け取り、CFG を抽出**
   ```hako
   method apply_ir(ir, options) {
       // ... 既存処理 ...

       // CFG 抽出（新規）
       local cfg_data = me.extract_cfg_from_ir(ir)

       // Analysis IR に cfg を追加
       ir.set("cfg", cfg_data)
   }

   method extract_cfg_from_ir(ir) {
       // builtin extract_mir_cfg() 呼び出し
       // または直接 JSON 操作

       local functions = ir.get("functions")
       local cfg_functions = ...

       return cfg_functions
   }
   ```

2. **HC020 実行時に CFG が利用される確認**
   - DeadBlockAnalyzerBox が `ir.cfg` を参照

### 成果物

- `analysis_consumer.hako` 修正
- CFG 抽出ロジック統合

---

## 6. Task 3: 統合テスト & 検証

### テスト項目

1. **Phase 154 の 4 テストケース全て実行**
   ```bash
   ./tools/hako_check_deadblocks_smoke.sh --with-cfg
   ```

2. **期待される HC020 出力**
   ```
   [HC020] Unreachable basic block: fn=TestEarlyReturn.test bb=2
   [HC020] Unreachable basic block: fn=TestAlwaysFalse.test bb=1
   [HC020] Unreachable basic block: fn=TestInfiniteLoop.test bb=2
   [HC020] Unreachable basic block: fn=TestAfterBreak.test bb=2
   ```

3. **スモークスクリプト更新**
   - CFG ブリッジ有効時の出力確認
   - HC019 + HC020 の両方が実行される確認

### 成果物

- 統合テスト結果
- スモークスクリプト成功

---

## 7. Task 4: ドキュメント & CURRENT_TASK 更新

### ドキュメント

1. **phase155_mir_cfg_bridge.md** に：
   - 実装結果を記録
   - データフロー図
   - テスト結果

2. **CURRENT_TASK.md**：
   - Phase 154 完了記録
   - Phase 155 完了記録
   - Phase 156 への推奨

### git commit

```
feat(hako_check): Phase 155 MIR CFG data bridge implementation

🌉 CFG データブリッジ完成！

🔗 実装内容:
- extract_mir_cfg() builtin 関数（Rust）
- analysis_consumer.hako 修正（.hako）
- HC020 完全動作確認

✅ テスト結果: 4/4 PASS
- TestEarlyReturn
- TestAlwaysFalse
- TestInfiniteLoop
- TestAfterBreak

🎯 Phase 154 + 155 で hako_check HC020 ルール完全実装！
```

---

## ✅ 完成チェックリスト（Phase 155）

- [ ] Task 1: extract_mir_cfg() builtin 実装
  - [ ] 関数実装
  - [ ] Builtin 登録
  - [ ] ユニットテスト
- [ ] Task 2: analysis_consumer.hako 修正
  - [ ] CFG 抽出ロジック統合
  - [ ] DeadBlockAnalyzerBox との連携確認
- [ ] Task 3: 統合テスト & 検証
  - [ ] 4 テストケース全て HC020 出力確認
  - [ ] スモークスクリプト成功
- [ ] Task 4: ドキュメント & CURRENT_TASK 更新
  - [ ] 実装ドキュメント完成
  - [ ] git commit

---

## 技術的考慮事項

### CFG 抽出の鍵

- **Entry Block**: 関数の最初のブロック（多くの場合 block_id = 0）
- **Successors**: terminator から判定
  - `Jump { target }` → 1 successor
  - `Branch { then_bb, else_bb }` → 2 successors
  - `Return` → 0 successors
- **Reachability**: DFS で entry から到達可能なブロックを収集

### .hako での JSON 操作

```hako
// JSON オブジェクト生成
local cfg_obj = {}
cfg_obj.set("name", "Main.main")
cfg_obj.set("entry_block", 0)

// JSON 配列操作
local blocks = []
blocks.push(block_info)

cfg_obj.set("blocks", blocks)
```

---

## 次のステップ

Phase 155 完了後：
- **Phase 156**: HC021（定数畳み込み検出）
- **Phase 157**: HC022（型不一致検出）

---

## 参考リソース

- **Phase 154**: `docs/development/current/main/phase154_mir_cfg_deadblocks.md`
- **MIR CFG 抽出**: `src/mir/cfg_extractor.rs` (Phase 154 で作成済み)
- **Analysis IR 定義**: `tools/hako_check/analysis_ir.hako`
- **DeadBlockAnalyzerBox**: `tools/hako_check/rules/rule_dead_blocks.hako`

---

**作成日**: 2025-12-04
**Phase**: 155（MIR CFG データブリッジ実装）
**予定工数**: 2-3 時間
**難易度**: 低（主に plumbing）

---

## Phase 155 MVP 実装詳細

### 実装アプローチ

**Phase 155-1: MIR JSON に CFG を含める** ✅ 完了
- 場所: `src/runner/mir_json_emit.rs`
- 変更: `emit_mir_json_for_harness()` と `emit_mir_json_for_harness_bin()`
- 処理:
  ```rust
  // Phase 155: Extract CFG information for hako_check
  let cfg_info = nyash_rust::mir::extract_cfg_info(module);

  let root = if use_v1_schema {
      let mut root = create_json_v1_root(json!(funs));
      if let Some(obj) = root.as_object_mut() {
          obj.insert("cfg".to_string(), cfg_info);
      }
      root
  } else {
      json!({"functions": funs, "cfg": cfg_info})
  };
  ```

**Phase 155-2: Analysis IR に CFG フィールド追加** ✅ MVP 完了
- 場所: `tools/hako_check/analysis_consumer.hako`
- 変更: `build_from_source_flags()` の最後に CFG フィールドを追加
- 処理:
  ```hako
  // Phase 155: Add mock CFG data for MVP (will be replaced with actual MIR CFG extraction)
  // For now, create empty CFG structure so DeadBlockAnalyzerBox doesn't crash
  local cfg = new MapBox()
  local cfg_functions = new ArrayBox()
  cfg.set("functions", cfg_functions)
  ir.set("cfg", cfg)
  ```

### MVP の制限事項

1. **CFG データは空**
   - MIR JSON に CFG は含まれるが、hako_check は読み込まない
   - Analysis IR の `cfg.functions` は空配列
   - DeadBlockAnalyzerBox は実行されるが、検出結果は常に 0 件

2. **MIR 生成パスが未統合**
   - hako_check は現在ソース解析のみ（AST ベース）
   - MIR 生成・読み込みパスがない
   - MIR JSON ファイルを中間ファイルとして使う設計が必要

3. **builtin 関数なし**
   - `extract_mir_cfg()` builtin 関数は未実装
   - .hako から Rust 関数を直接呼び出す仕組みが未整備

### 次のステップ（Phase 156 or 155.5）

**Option A: hako_check に MIR パイプライン統合**
1. hako_check.sh で MIR JSON を生成
2. cli.hako で MIR JSON を読み込み
3. CFG を Analysis IR に反映
4. HC020 が実際にブロックを検出

**Option B: builtin 関数経由**
1. Rust 側で builtin 関数システムを実装
2. `extract_mir_cfg(mir_json)` を .hako から呼び出し可能に
3. analysis_consumer.hako で MIR JSON を処理

**推奨**: Option A（よりシンプル、既存の hakorune_emit_mir.sh を活用）

---

## テスト結果

### 基本動作確認
```bash
# MIR JSON に CFG が含まれることを確認
$ ./tools/hakorune_emit_mir.sh test.hako /tmp/test.json
$ jq '.cfg.functions[0].blocks' /tmp/test.json
# → CFG ブロック情報が出力される ✅
```

### hako_check 実行
```bash
$ ./tools/hako_check.sh apps/tests/hako_check/test_dead_blocks_early_return.hako
# → エラーなく実行完了 ✅
# → HC020 出力なし（CFG が空のため）✅ 期待通り
```

---

## まとめ

Phase 155 MVP として以下を達成：
- ✅ MIR JSON に CFG データを追加（Rust 側）
- ✅ Analysis IR に CFG フィールドを追加（.hako 側）
- ✅ DeadBlockAnalyzerBox が CFG にアクセス可能な構造

今後の課題：
- 🔄 MIR JSON → Analysis IR のデータ連携
- 🔄 hako_check の MIR パイプライン統合 または builtin 関数実装

Phase 154 + 155 により、HC020 の基盤は完成。実際の検出機能は Phase 156 で実装推奨。

---

**実装日**: 2025-12-04
**実装者**: Claude (AI Assistant)
**コミット**: feat(hako_check): Phase 155 MIR CFG data bridge (MVP)
Status: Historical
