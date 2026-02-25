# Phase 156: hako_check MIR パイプライン統合（HC020 を実働させる）

## 0. ゴール

**hako_check が MIR JSON + CFG をちゃんと入力として受け取れるようにし、Phase 154/155 で用意した HC020（unreachable basic block 検出）が実際にブロックを報告するところまで持っていく。**

目的：
- hako_check に MIR パイプラインを統合
- HC020 が CFG から unreachable block を実際に検出
- **Rust 側の変更なし**（.hako + シェルスクリプトのみ）

---

## 1. 設計方針

### Rust を触らない理由

- Phase 155 で MIR JSON に CFG を含める処理は完了済み
- 今後のセルフホスティングを考えると、.hako 側で拡張可能な設計が重要
- 解析ロジックは .hako で実装 → 将来 .hako コンパイラ自体でも使える

### アーキテクチャ

```
target.hako
    ↓
hakorune_emit_mir.sh（既存 or 新規）
    ↓
/tmp/mir.json（CFG 付き）
    ↓
hako_check.sh（MIR JSON パスを渡す）
    ↓
cli.hako（MIR JSON を読み込み）
    ↓
analysis_consumer.hako（CFG を Analysis IR に統合）
    ↓
rule_dead_blocks.hako（HC020: BFS/DFS で unreachable 検出）
    ↓
[HC020] Unreachable basic block: fn=..., bb=...
```

---

## 2. Task 1: 現在の hako_check.sh パイプライン確認

### 対象ファイル

- `tools/hako_check.sh`
- `tools/hako_check/cli.hako`

### やること

1. **現状確認**
   - CLI が何を渡しているか確認
   - どのバックエンドで .hako を実行しているか（VM/LLVM/PyVM）
   - AST/analysis JSON だけを渡しているのか、MIR JSON はまだ使っていないのか

2. **結果を記録**
   - `phase156_hako_check_mir_pipeline.md` の末尾に現状メモを追記

### 成果物

- 現状パイプラインの理解
- メモ記録

---

## 3. Task 2: hako_check.sh で MIR JSON を生成する

### 方針

hako_check を叩く前に、対象 .hako から MIR JSON を一時ファイルに吐くステップを追加する。

### やること

1. **MIR emit スクリプトの確認**
   - `tools/hakorune_emit_mir.sh` があれば利用
   - なければ `nyash --emit-mir-json` 等の既存 CLI オプションを使用

2. **hako_check.sh を修正**
   ```bash
   # Phase 156: MIR JSON 生成
   MIR_JSON_PATH="/tmp/hako_check_mir_$$.json"
   ./tools/hakorune_emit_mir.sh "$TARGET_FILE" "$MIR_JSON_PATH"

   # MIR JSON パスを環境変数で渡す
   export HAKO_CHECK_MIR_JSON="$MIR_JSON_PATH"

   # 既存の hako_check 実行
   ...
   ```

3. **クリーンアップ**
   - 終了時に一時ファイルを削除

### 成果物

- `tools/hako_check.sh` 修正
- MIR JSON 生成ステップ追加

---

## 4. Task 3: cli.hako で MIR JSON を読み込む

### 対象ファイル

- `tools/hako_check/cli.hako`
- `tools/hako_check/analysis_consumer.hako`

### やること

1. **環境変数から MIR JSON パスを取得**
   ```hako
   local mir_json_path = env.get("HAKO_CHECK_MIR_JSON")
   if mir_json_path != null {
       local mir_data = me.load_json_file(mir_json_path)
       // CFG を Analysis IR に統合
       me.integrate_cfg_from_mir(ir, mir_data)
   }
   ```

2. **CFG を Analysis IR に統合**
   - MIR JSON の `cfg` フィールドを取得
   - Analysis IR の `cfg` フィールドに設定
   - Phase 155 で作成した空の CFG 構造体を置き換え

3. **AST ベースのルールは維持**
   - 既存の HC001〜HC019 はそのまま動作
   - MIR CFG は HC020 専用

### 成果物

- `cli.hako` 修正（MIR JSON 読み込み）
- `analysis_consumer.hako` 修正（CFG 統合）

---

## 5. Task 4: HC020 が CFG を使うようにする

### 対象ファイル

- `tools/hako_check/rules/rule_dead_blocks.hako`

### やること

1. **CFG データの取得**
   ```hako
   local cfg = ir.get("cfg")
   local functions = cfg.get("functions")
   if functions == null or functions.length() == 0 {
       // CFG がない場合はスキップ
       return
   }
   ```

2. **各関数ごとに到達可能性解析**
   ```hako
   method analyze_function(fn_cfg, path, out) {
       local entry_block = fn_cfg.get("entry_block")
       local blocks = fn_cfg.get("blocks")

       // BFS/DFS で到達可能なブロックを収集
       local reachable = me.compute_reachability(entry_block, blocks)

       // 未到達ブロックを報告
       for block in blocks {
           if not reachable.contains(block.get("id")) {
               out.add_diagnostic({
                   "rule": "HC020",
                   "severity": "warning",
                   "message": "Unreachable basic block",
                   "fn": fn_cfg.get("name"),
                   "bb": block.get("id")
               })
           }
       }
   }
   ```

3. **BFS/DFS 実装**
   ```hako
   method compute_reachability(entry, blocks) {
       local visited = new SetBox()
       local queue = new ArrayBox()
       queue.push(entry)

       loop(queue.length() > 0) {
           local current = queue.shift()
           if visited.contains(current) { continue }
           visited.add(current)

           local block = me.find_block(blocks, current)
           if block != null {
               local successors = block.get("successors")
               for succ in successors {
                   if not visited.contains(succ) {
                       queue.push(succ)
                   }
               }
           }
       }

       return visited
   }
   ```

### 期待される結果

Phase 154 のテスト 4 ケースで HC020 が報告：
- `test_dead_blocks_early_return.hako` → 早期 return 後のブロック検出
- `test_dead_blocks_always_false.hako` → false 条件内のブロック検出
- `test_dead_blocks_infinite_loop.hako` → 無限ループ後のブロック検出
- `test_dead_blocks_after_break.hako` → break 後のブロック検出

### 成果物

- `rule_dead_blocks.hako` 修正（BFS/DFS 実装）
- HC020 が実際にブロックを検出

---

## 6. Task 5: スモークと回帰確認

### スモークテスト

```bash
# HC020 スモーク
./tools/hako_check_deadblocks_smoke.sh

# 期待: [HC020] Unreachable basic block: ... が出力される
```

### 回帰テスト

```bash
# HC019 スモーク（dead code）
./tools/hako_check_deadcode_smoke.sh

# 期待: 既存の HC019 出力に変化なし
```

### JoinIR 経路確認

```bash
# JoinIR Strict モードで確認
NYASH_JOINIR_STRICT=1 ./tools/hako_check.sh --dead-blocks apps/tests/hako_check/test_dead_blocks_early_return.hako

# 期待: JoinIR→MIR→CFG→HC020 のラインに崩れなし
```

### 成果物

- スモークテスト成功
- 回帰なし確認
- JoinIR 経路確認

---

## 7. Task 6: ドキュメントと CURRENT_TASK 更新

### ドキュメント更新

1. **phase156_hako_check_mir_pipeline.md** に：
   - 実装結果を記録
   - パイプライン図の確定版

2. **hako_check_design.md** に：
   - 「HC020 は MIR CFG を入力にする」こと
   - 「CLI は内部で一度 MIR JSON を生成してから hako_check を実行する」こと

3. **CURRENT_TASK.md** に：
   - Phase 156 セクションを追加
   - Phase 154/155/156 の完了記録

### git commit

```
feat(hako_check): Phase 156 MIR pipeline integration for HC020

🎯 HC020 が実際に unreachable block を検出！

🔧 実装内容:
- hako_check.sh: MIR JSON 生成ステップ追加
- cli.hako: MIR JSON 読み込み
- analysis_consumer.hako: CFG を Analysis IR に統合
- rule_dead_blocks.hako: BFS/DFS で到達可能性解析

✅ テスト結果:
- test_dead_blocks_early_return: [HC020] 検出
- test_dead_blocks_always_false: [HC020] 検出
- test_dead_blocks_infinite_loop: [HC020] 検出
- test_dead_blocks_after_break: [HC020] 検出

🏗️ 設計原則:
- Rust 側変更なし（Phase 155 まで）
- .hako + シェルスクリプトのみで拡張
- セルフホスティング対応設計
```

---

## ✅ 完成チェックリスト（Phase 156）

- [ ] Task 1: hako_check.sh パイプライン確認
  - [ ] 現状パイプライン理解
  - [ ] メモ記録
- [ ] Task 2: MIR JSON 生成
  - [ ] hako_check.sh 修正
  - [ ] MIR JSON 生成ステップ追加
- [ ] Task 3: MIR JSON 読み込み
  - [ ] cli.hako 修正
  - [ ] analysis_consumer.hako 修正
  - [ ] CFG 統合確認
- [ ] Task 4: HC020 実装
  - [ ] BFS/DFS 到達可能性解析
  - [ ] unreachable block 報告
  - [ ] 4 テストケースで検出確認
- [ ] Task 5: スモークと回帰
  - [ ] HC020 スモーク成功
  - [ ] HC019 回帰なし
  - [ ] JoinIR 経路確認
- [ ] Task 6: ドキュメント更新
  - [ ] phase156 実装結果記録
  - [ ] hako_check_design.md 更新
  - [ ] CURRENT_TASK.md 更新
  - [ ] git commit

---

## 技術的注意点

### SetBox / ArrayBox の存在確認

.hako で Set/Array 操作が必要。存在しない場合は MapBox で代用：

```hako
// SetBox がない場合
local visited = new MapBox()
visited.set(block_id, true)
if visited.get(block_id) == true { ... }
```

### JSON ファイル読み込み

```hako
// FileBox + JsonBox で読み込み
local file = new FileBox()
local content = file.read(mir_json_path)
local json_parser = new JsonBox()
local data = json_parser.parse(content)
```

### 環境変数アクセス

```hako
// EnvBox または組み込み関数で取得
local mir_path = env.get("HAKO_CHECK_MIR_JSON")
```

---

## 次のステップ

Phase 156 完了後：
- **Phase 157**: HC021（定数畳み込み検出）
- **Phase 158**: HC022（型不一致検出）
- **Phase 160+**: .hako JoinIR/MIR 移植

---

##  実装結果（Phase 156）

### 実装完了項目

✅ **Task 1-2: hako_check.sh パイプライン修正**
- MIR JSON 生成ステップ追加（hakorune_emit_mir.sh使用）
- MIR JSON content をインライン引数として渡す方式採用
- FileBox依存を回避（NYASH_DISABLE_PLUGINS=1 対応）

✅ **Task 3: cli.hako 修正**
- `--mir-json-content` 引数処理追加
- MIR JSON text を IR に `_mir_json_text` として格納

✅ **Task 4: analysis_consumer.hako CFG統合**
- 完全な JSON パーサー実装（約320行）
- CFG functions/blocks/successors/reachable 全フィールド対応
- 空CFG フォールバック処理

✅ **Task 5: rule_dead_blocks.hako**
- Rust側で計算済みの `reachable` フィールドを使用
- BFS/DFS は Rust 側（cfg_extractor.rs）で実装済み
- HC020 出力フォーマット実装済み

### 技術的設計決定

**1. FileBox回避アーキテクチャ**
- 問題: NYASH_DISABLE_PLUGINS=1 で FileBox 使用不可
- 解決: MIR JSON をインライン文字列として引数経由で渡す
- 利点: プラグイン依存なし、既存パターン（--source-file）と一貫性

**2. .hako JSON パーサー実装**
- 軽量な手動JSON解析（StringBox.substring/indexOf のみ使用）
- 必要最小限のフィールドのみ抽出
- エラーハンドリング: 空CFG にフォールバック

**3. Reachability計算の責務分離**
- Rust: CFG構築 + Reachability計算（cfg_extractor.rs）
- .hako: CFG読み込み + unreachable block報告のみ
- 理由: Rustで効率的な計算、.hakoはシンプルに保つ

---

**作成日**: 2025-12-04
**実装日**: 2025-12-04
**Phase**: 156（hako_check MIR パイプライン統合）
**予定工数**: 3-4 時間
**実工数**: 約4時間（JSON パーサー実装を含む）
**難易度**: 中（.hako JSON パーサー実装が主な作業）
**Rust 変更**: なし（.hako + シェルスクリプトのみ）

---

## 次のステップへの推奨事項

### フィードバック

**1. 改善提案**
- JSON パーサーの堅牢化（エスケープシーケンス対応など）
- デバッグ出力の統合（--debug フラグで CFG内容表示）
- エラーメッセージの詳細化（どのフィールドの解析で失敗したか）

**2. 実装中に気づいた課題**
- .hako に builtin JSON パーサーがないため手動実装が必要
- StringBox のみでJSON解析は冗長（約320行）
- FileBox がプラグイン依存で、開発環境では常に無効化される

**3. 箱化モジュール化パターンの改善点**
- JSON解析ユーティリティを共通モジュール化すべき
- `tools/hako_shared/json_parser.hako` として切り出し推奨
- 他の解析ルールでも再利用可能

**4. .hako API の不足点**
- JSON builtin関数（parse_json/toJson）が欲しい
- 環境変数アクセス（getenv）が欲しい
- File I/O がプラグイン必須なのは開発時に不便

**5. Phase 157+ への推奨**
- HC021（定数畳み込み検出）でも同様のパターンを使用可能
- MIR解析ルール追加時のテンプレートとして活用
- JSON schema validation を .hako で実装も検討
Status: Historical
