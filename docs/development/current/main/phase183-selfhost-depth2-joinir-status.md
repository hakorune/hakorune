# Phase 183: Selfhost Depth-2 JoinIR 再観測（観測フェーズ）

## 概要

Phase 150 で選定した「selfhost depth-2 の最小ループ 5本」について、
Phase 171-181 での JoinIR ループ統一・Pattern 整備の後で、
改めて「どのループが JoinIR で動いているか」を観測する。

目的は、selfhost depth-2 実装フェーズ（Phase 184+）での
「JoinIR → .hako 側フロントエンド」の設計に活かす。

## 背景: Selfhost Depth とは

### Depth-1（現在の実装状態）

**定義**: Rust コンパイラが .hako ソースをコンパイルする（1周目）

**パイプライン**:
```
target/release/hakorune (Rust)
    ↓ [Stage-B: 関数スキャン + scaffold]
stage1_cli.hako (JSON v0 scaffold)
    ↓ [Stage-1: CLI/using 解決]
stage1_output.hako (Stage-3 構文)
    ↓ [Stage-3: 実際のコンパイル本体 - Rust実装]
Program(JSON v0)
    ↓ [dev verify]
VM/LLVM 実行
```

**現状**: Phase 150 で 5本の代表ケースが動作確認済み

### Depth-2（Phase 183-184 での目標）

**定義**: .hako コンパイラが .hako ソースをコンパイルする（2周目）

**パイプライン**:
```
target/release/hakorune (Rust)
    ↓ [Stage-B: Rust実装]
stage1_cli.hako (JSON v0)
    ↓ [Stage-1: .hako実装（NEW!）]
stage1_output.hako
    ↓ [Stage-3: .hako実装（NEW!）]
Program(JSON v0)
    ↓ [dev verify]
VM/LLVM 実行
```

**Phase 183 の役割**: Depth-2 で動かす Stage-1/Stage-3 の .hako コンパイラを観測し、
どのループが JoinIR で動いているかを確認

## Phase 150 で選定した Depth-2 最小ループ（5本）

### Phase 150 選定内容の確認

Phase 150 のドキュメント（phase150_selfhost_stage3_depth1_baseline.md）によると、
Depth-1 で動作確認済みの 5本のケースは以下の通り：

| # | ケース名 | タイプ | 備考 |
|---|---------|--------|------|
| 1 | `peek_expr_block.hako` | block/match式 | Baseline: match式、ブロック式の基本動作確認 |
| 2 | `loop_min_while.hako` | loop基本 | Baseline: ループ変数、Entry PHI、Exit PHI |
| 3 | `string_method_chain.hako` | string処理 | NEW: メソッドチェーン（`substring().length()`） |
| 4 | `joinir_min_loop.hako` | loop+break | NEW: break制御、ControlForm::Loop検証 |
| 5 | `joinir_if_select_simple.hako` | if+return | NEW: 早期return、分岐の値伝播 |

**注意**: これらは Depth-1（Rust コンパイラで実行）の**テストケース**であり、
Depth-2 で観測する対象は「.hako コンパイラ自身のループ」である。

### Depth-2 観測対象の定義

**Phase 183 で観測すべき対象**:
- **Stage-1 コンパイラ**（`apps/selfhost-runtime/` 配下）のループ
- **Stage-3 コンパイラ**（`apps/selfhost-runtime/` 配下）のループ
- 特に「文字列解析」「トークン処理」「AST構築」に関わるループ

**観測方針**:
1. Phase 181 で分析した JsonParser のループパターンと同様の手法を使う
2. MirAnalyzerBox / JoinIrAnalyzerBox での実行時観測
3. ループごとに P1-P5 分類を行う

## 観測方法

### Step 1: Depth-2 ループの特定

**対象ファイル**（Phase 150 で定義済み）:
- `apps/selfhost-runtime/` 配下の .hako ファイル
- Stage-1/Stage-3 コンパイラの実装

**作業内容**:
1. selfhost-runtime 配下の .hako ファイルをリストアップ
2. 各ファイルのループを特定（行番号、関数名）
3. 最小の代表ループ 5-7本を選定（Phase 150 と同様）

**選定基準**:
- 頻繁に実行されるループ（トークン走査、文字列解析など）
- 異なるパターン（P1-P5）を代表するループ
- 実装難易度が低い〜中程度のループ

### Step 2: 各ループの Pattern 分類

**Phase 192 の AST feature extractor を使用**:

```bash
# ループの特徴を自動抽出
NYASH_FEATURES=stage3 NYASH_USE_NY_COMPILER=1 \
  ./target/release/hakorune --analyze-loop <file.hako> <function_name>
```

**分類観点**:
- loop_condition: LoopParam/OuterLocal/LoopBodyLocal のどれに依存するか
- body_pattern: simple, break, continue, PHI など
- キャリア: 単一/複数、型
- ブロック有無: if/else ネスト
- MethodCall: 有無、呼び出し回数

### Step 3: MirAnalyzerBox / JoinIrAnalyzerBox での検証

**実行方法**:
```bash
# Depth-2 selfhost 実行（Stage-1/Stage-3 を .hako で動かす）
NYASH_FEATURES=stage3 NYASH_USE_NY_COMPILER=1 \
  NYASH_JOINIR_STRICT=1 \
  NYASH_JOINIR_DEBUG=1 \
  ./target/release/hakorune <selfhost_target.hako>
```

**観測ポイント**:
- どのループが JoinIR ルートで通っているか
- どのループが Fail-Fast しているか（理由付き）
- Phase 192 の AST feature extractor が正しく動作しているか

**ログ確認**:
```bash
# JoinIR 詳細ログ
grep "\[trace:joinir\]" log.txt

# Pattern 検出ログ
grep "\[trace:pattern\]" log.txt

# Fail-Fast 理由
grep "JoinIR blocked" log.txt
```

### Step 4: 結果まとめ

**観測結果テーブル**（Task 183-2 で作成）:
```markdown
| ループ名 | ファイル | 行番号 | Pattern 予想 | Pattern 実績 | JoinIR通過 | Fail-Fast理由 | 実装優先度 |
|---------|---------|--------|------------|-----------|-----------|-------------|-----------|
| ... | ... | ... | ... | ... | ... | ... | ... |
```

**分析観点**:
- P1-P5 のどのパターンが多いか
- Fail-Fast が多い原因は何か（LoopBodyLocal, MethodCall, etc.）
- Phase 184+ で優先実装すべきループはどれか

## ドキュメント出力

### Task 183-1: ループ特定とパターン分類

**ファイル**: `phase183-selfhost-depth2-loop-inventory.md`（新規作成）

**内容**:
```markdown
# Phase 183: Selfhost Depth-2 ループ一覧

## 選定ループ一覧（5-7本）

| # | ループ名 | ファイル | 行番号 | 関数名 | 役割 |
|---|---------|---------|--------|--------|------|
| 1 | ... | ... | ... | ... | ... |
| 2 | ... | ... | ... | ... | ... |
| ... | ... | ... | ... | ... | ... |

## 各ループの詳細構造

### Loop 1: ...

**ソースコード**:
```nyash
loop(condition) {
    ...
}
```

**特徴**:
- loop_condition: ...
- キャリア: ...
- PHI: ...
- MethodCall: ...
- break/continue: ...

**P1-P5 分類**: Pattern X

---
```

### Task 183-2: JoinIR 通過・Fail-Fast 観測結果

**ファイル**: `phase183-selfhost-depth2-joinir-observation.md`（新規作成）

**内容**:
```markdown
# Phase 183: Selfhost Depth-2 JoinIR 観測結果

## 実行環境

- **Date**: 2025-12-08
- **Rust VM**: ./target/release/hakorune
- **JoinIR Strict**: NYASH_JOINIR_STRICT=1
- **JoinIR Debug**: NYASH_JOINIR_DEBUG=1
- **Selfhost**: NYASH_FEATURES=stage3 NYASH_USE_NY_COMPILER=1

## 観測結果サマリー

| Pattern | JoinIR通過数 | Fail-Fast数 | 合計 |
|---------|------------|------------|------|
| P1 Simple | ... | ... | ... |
| P2 Break | ... | ... | ... |
| P3 If-PHI | ... | ... | ... |
| P4 Continue | ... | ... | ... |
| P5 Trim | ... | ... | ... |
| **合計** | ... | ... | ... |

## 各ループの観測結果

### Loop 1: ...

**実行コマンド**:
```bash
NYASH_FEATURES=stage3 NYASH_USE_NY_COMPILER=1 \
  NYASH_JOINIR_STRICT=1 \
  ./target/release/hakorune <file.hako>
```

**結果**: ✅ JoinIR 通過 / ❌ Fail-Fast

**ログ**:
```
[trace:joinir] pattern1: 3 functions, 13 blocks
[trace:pattern] route: Pattern1_Minimal MATCHED
```

**Pattern 分類**: Pattern X

**Fail-Fast 理由**（該当する場合）:
- LoopBodyLocal が条件に含まれる
- MethodCall が複数存在
- その他

---
```

### Task 183-3: Phase 184+ への提案

**ファイル**: `phase183-selfhost-depth2-implementation-plan.md`（新規作成）

**内容**:
```markdown
# Phase 183: Selfhost Depth-2 実装計画（Phase 184+ への提案）

## 観測結果に基づく戦略

### 高優先度ループ（Phase 184A）

**対象**:
- ループ名: ...
- Pattern: ...
- 理由: ...

**実装戦略**:
- ...

### 中優先度ループ（Phase 184B）

**対象**:
- ループ名: ...
- Pattern: ...
- 理由: ...

**実装戦略**:
- ...

### 低優先度ループ（Phase 184C+）

**対象**:
- ループ名: ...
- Pattern: ...
- 理由: ...

**実装戦略**:
- ...

## .hako 側 JoinIR フロントエンド設計

### 必要な機能

1. **AST → JoinIR 変換**（.hako 実装）
2. **Pattern 検出ロジック**（.hako 実装）
3. **Carrier 情報の伝播**（.hako 実装）
4. **PHI 命令生成**（.hako 実装）

### 段階的実装方針

**Phase 184A**: P1/P2 Simple ループの .hako 実装
**Phase 184B**: P3/P4 複雑ループの .hako 実装
**Phase 184C**: P5 Trim/CharComparison ループの .hako 実装

## Analyzer 拡張

### MirAnalyzerBox 拡張

**追加機能**:
- Selfhost Depth-2 専用ルート
- ループごとの Pattern 自動分類
- Fail-Fast 理由の詳細レポート

### JoinIrAnalyzerBox 拡張

**追加機能**:
- JoinIR 構造の可視化（DOT/JSON）
- ループごとの lowering ログ
- Phase 184+ 向けの実装ヒント出力

---
```

## 関連ドキュメント

- `phase150_selfhost_stage3_depth1_baseline.md` - Depth-1 ベースライン（Phase 150）
- `phase150_selfhost_stage3_depth1_results.md` - Depth-1 実行結果
- `phase181-jsonparser-loop-roadmap.md` - JsonParser ループ分析（Phase 181）
- `phase182-char-comparison-generalization.md` - CharComparison 汎用化（Phase 182）
- `joinir-architecture-overview.md` - JoinIR アーキテクチャ全体図

## タイムライン

### Phase 183（このフェーズ）

- **Task 183-1**: ループ特定とパターン分類（60分）
  - selfhost-runtime 配下の .hako ファイル解析
  - 5-7本の代表ループ選定
  - P1-P5 分類
- **Task 183-2**: JoinIR 通過・Fail-Fast 観測（90分）
  - MirAnalyzerBox / JoinIrAnalyzerBox での実行
  - 観測ログの収集・解析
  - 結果テーブル作成
- **Task 183-3**: Phase 184+ への提案（45分）
  - 実装優先度の決定
  - .hako 側 JoinIR フロントエンドの設計方針
  - Analyzer 拡張提案

**Total Estimated Time**: 195分（約3.25時間）

### Phase 184+（次フェーズ以降）

- **Phase 184A**: 高優先度ループ実装（P1/P2 Simple）
- **Phase 184B**: 中優先度ループ実装（P3/P4 Complex）
- **Phase 184C**: 低優先度ループ実装（P5 Trim/CharComparison）
- **Phase 185**: Analyzer 拡張実装
- **Phase 186**: Depth-2 統合テスト

## 成功基準

### Phase 183

- ✅ Depth-2 ループを 5-7本特定
- ✅ 各ループの P1-P5 分類完了
- ✅ MirAnalyzerBox / JoinIrAnalyzerBox での観測完了
- ✅ 観測結果ドキュメント 3本作成
  - phase183-selfhost-depth2-loop-inventory.md
  - phase183-selfhost-depth2-joinir-observation.md
  - phase183-selfhost-depth2-implementation-plan.md
- ✅ Phase 184+ への実装方針明確化

### Phase 184+

- ✅ .hako 側 JoinIR フロントエンド実装開始
- ✅ 高優先度ループの JoinIR 統一完了
- ✅ Depth-2 selfhost 実行成功（最小ケース）

## 重要な観点

### Depth-1 と Depth-2 の違い

| 観点 | Depth-1（Phase 150） | Depth-2（Phase 183+） |
|------|---------------------|---------------------|
| コンパイラ | Rust | .hako |
| 対象 | テストケース（5本） | コンパイラ自身のループ（5-7本） |
| JoinIR | Rust実装済み | .hako実装が必要 |
| 目的 | 基本動作確認 | selfhost 完全実装 |

### JoinIR → .hako フロントエンド設計の鍵

**Phase 183 の観測が重要な理由**:
1. **Pattern 分布の把握**: どのパターンが多いか → 実装優先度決定
2. **Fail-Fast 原因の特定**: どのような制約がボトルネックか → 解決策設計
3. **実装難易度の評価**: 段階的実装の計画立案

**Phase 184+ での活用**:
- P1/P2 Simple が多い → 優先実装
- P3/P4 Complex が多い → PHI/MethodCall 対応が鍵
- P5 Trim/CharComparison が多い → Phase 182 の汎用化が活きる

## 備考

### Phase 150 との関係

Phase 150 は Depth-1 の**テストケース**を選定・実行したフェーズ。
Phase 183 は Depth-2 の**コンパイラ自身のループ**を観測するフェーズ。

**相違点**:
- Phase 150: Rust コンパイラで .hako テストケースを実行
- Phase 183: .hako コンパイラで .hako コンパイラを実行

**共通点**:
- どちらも「5-7本の代表ケース」を選定
- どちらも JoinIR の動作確認が目的

### JsonParser との類似性

Phase 181 で JsonParser の全11ループを分析した手法を、
Phase 183 では selfhost コンパイラのループに適用する。

**共通の分析手法**:
- ループの特徴抽出（loop_condition, carrier, PHI, etc.）
- P1-P5 分類
- Fail-Fast 原因の特定

---

**作成日**: 2025-12-08
**Phase**: 183（Selfhost Depth-2 JoinIR 再観測・設計）
**ステータス**: 観測計画ドキュメント（実装なし）
**次フェーズ**: Phase 184+（.hako 側 JoinIR フロントエンド実装）
