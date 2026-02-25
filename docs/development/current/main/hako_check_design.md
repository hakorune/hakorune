# hako_check 設計（Phase 121 時点）

## 概要

**hako_check** は .hako ファイルの静的解析・検証を行うツール。
Phase 121 では、この経路を JoinIR 統合に向けて設計する。

## hako_check の役割

### 現在の機能

- **構文チェック**: パーサーエラーの検出
- **型チェック**: 基本的な型の整合性確認
- **MIR 生成**: .hako → MIR への変換（検証用）
- **制御フローチェック**: unreachable code などの検出
- **ルールベース診断**: 19種類の診断ルール（HC001-HC031）

### hako_check の実装構造（Phase 121 調査時点）

**重要**: hako_check は **Rust バイナリではなく .hako スクリプト**として実装されている。

```
tools/hako_check.sh               # シェルスクリプトラッパー
  ↓
tools/hako_check/cli.hako         # .hako エントリーポイント
  ↓
HakoAnalyzerBox.run()             # 静的解析メインロジック
  ↓
HakoAnalysisBuilderBox            # IR 生成（analysis_consumer.hako）
  ↓
hakorune --backend vm             # Rust VM で実行
  ↓
MirCompiler::compile()            # AST → MIR 変換
  ↓
MirBuilder::build_module()        # MIR 生成（ここで PHI 生成）
```

### エントリーポイントの詳細

**ファイル**: `tools/hako_check.sh`

**実行フロー**:
1. `tools/hako_check.sh` がシェルスクリプトとして起動
2. `$BIN` (通常は `./target/release/hakorune`) を使用して `cli.hako` を実行
3. 環境変数で VM バックエンドを指定: `--backend vm`

**重要な環境変数**:
```bash
NYASH_DISABLE_PLUGINS=1              # プラグイン無効化（安定性優先）
NYASH_BOX_FACTORY_POLICY=builtin_first  # ビルトイン Box 優先
NYASH_FEATURES=stage3                # Stage-3 パーサー使用
NYASH_ENABLE_USING=1                 # using 文有効化
```

### MIR 生成経路

**hako_check での MIR 生成フロー**:

```
1. cli.hako が .hako ファイルを読み込む
   ↓
2. HakoAnalysisBuilderBox.build_from_source_flags() を呼び出す
   ↓
3. 内部で HakoParserCoreBox.parse() を使用（.hako パーサー）
   ↓
4. AST を取得（Rust VM 内部で nyash_rust::parser::NyashParser を使用）
   ↓
5. Rust VM が AST を MirCompiler に渡す
   ↓
6. MirCompiler::compile_with_source() が AST → MIR 変換
   ↓
7. MirBuilder::build_module() で MIR 生成
   ↓
8. If/Loop 文は control_flow.rs で処理
```

### PHI 生成経路（Phase 121 時点）

**If 文の PHI 生成**:
- **ファイル**: `src/mir/builder/if_form.rs`
- **関数**: `MirBuilder::cf_if()` → PHI 生成ロジック
- **JoinIR 統合**: ❌ **未実装**（旧 PHI 生成器を使用中）

**Loop の PHI 生成**:
- **ファイル**: `src/mir/builder/control_flow.rs`
- **関数**: `MirBuilder::cf_loop()` → `try_cf_loop_joinir()`
- **JoinIR 統合**: ⚠️ **部分実装**（Phase 49 で mainline 統合開始）
  - `HAKO_JOINIR_PRINT_TOKENS_MAIN=1`: 特定関数のみ JoinIR 経由
  - `HAKO_JOINIR_ARRAY_FILTER_MAIN=1`: 特定関数のみ JoinIR 経由
  - デフォルトは旧 LoopBuilder へフォールバック

### Phase 121 での課題

- **旧 MIR/PHI 経路**: 現在は旧 PHI 生成器を主に使用している
- **JoinIR 統合**: If/Loop の JoinIR Lowering 経由への移行が必要
- **Strict モード**: NYASH_JOINIR_STRICT=1 での動作確認が未実施

## 現在の JoinIR 統合状況

> Note (2025-12): 現在は LoopBuilder を物理削除し、JoinIR は常時 ON（NYASH_JOINIR_CORE は deprecated/no-op）。以下のコードスケッチは Phase 121 当時の歴史メモとして残しているよ。

### Loop PHI 生成（部分統合済み）

**Phase 49 Mainline Integration**:

```rust
// src/mir/builder/control_flow.rs
fn try_cf_loop_joinir(&mut self, condition: &ASTNode, body: &ASTNode) -> Result<Option<ValueId>, String> {
    let core_on = crate::config::env::joinir_core_enabled();
    let mainline_targets = vec!["print_tokens", "filter"];

    if core_on && is_mainline_target(&func_name) {
        // JoinIR Frontend を試す
        return self.cf_loop_joinir_impl(condition, body, &func_name, debug);
    }

    // フォールバック: 旧 LoopBuilder
    Ok(None)
}
```

**特徴**:
- **選択的統合**: 特定関数のみ JoinIR 経由（mainline targets）
- **フォールバック**: 失敗時は旧 LoopBuilder に戻る
- **環境変数制御**:
  - `NYASH_JOINIR_CORE=1`: JoinIR Core 有効化
  - `HAKO_JOINIR_PRINT_TOKENS_MAIN=1`: print_tokens/0 を JoinIR 経由
  - `HAKO_JOINIR_ARRAY_FILTER_MAIN=1`: ArrayExtBox.filter/2 を JoinIR 経由

### If PHI 生成（未統合）

**Phase 121 調査結果**: If 文の PHI 生成は旧経路を使用中

**ファイル**: `src/mir/builder/if_form.rs`

```rust
// 旧 PHI 生成器（Phase 121 時点で現役）
fn cf_if(&mut self, condition: &ASTNode, then_block: &ASTNode, else_block: Option<&ASTNode>) -> Result<ValueId, String> {
    // 旧 PHI 生成ロジック
    // Phase 33-10 で JoinIR If Lowering が実装されたが、
    // MirBuilder 経路ではまだ統合されていない
}
```

## JoinIR 統合設計

### 統合方針

**3段階移行戦略**:

1. **Phase 122**: 環境変数で JoinIR 経路を選択可能に（デフォルトは旧経路）
   - `NYASH_HAKO_CHECK_JOINIR=1` で JoinIR 経路を有効化
   - 旧経路との互換性維持（フォールバック可能）

2. **Phase 123**: JoinIR 経路をデフォルトに（旧経路は `NYASH_LEGACY_PHI=1` でのみ有効）
   - デフォルトで JoinIR 経路を使用
   - 旧経路は明示的に有効化した場合のみ使用

3. **Phase 124**: 旧経路完全削除（JoinIR のみ）
   - 旧 PHI 生成器の完全削除
   - `NYASH_LEGACY_PHI=1` 環境変数も削除

### 設計原則

**Baseline First**:
- Phase 120 で確立した selfhost 経路のベースラインを参考に
- hako_check 経路でも同様のベースライン確立が必要

**Fail-Fast**:
- フォールバック処理は原則禁止（Phase 123 以降）
- エラーは早期に明示的に失敗させる

**環境変数制御**:
- `NYASH_HAKO_CHECK_JOINIR=1`: hako_check で JoinIR 経路を有効化
- `NYASH_JOINIR_STRICT=1`: フォールバック禁止（厳格モード）

### hako_check 特有の考慮事項

**1. .hako スクリプトとしての実行**:
- hako_check は Rust バイナリではなく .hako スクリプト
- Rust VM 経由で実行されるため、VM の MirBuilder を使用
- JoinIR 統合は **VM の MirBuilder** に実装する必要がある

**2. 診断ルールとの関係**:
- hako_check は 19 種類の診断ルール（HC001-HC031）を実装
- MIR 生成は診断の前段階（IR 生成用）
- JoinIR 統合による診断ルールへの影響は最小限

**3. selfhost 経路との違い**:
- **selfhost**: .hako コンパイラを実行（実行時 PHI 必要）
- **hako_check**: 静的解析のみ（MIR 生成は検証用）
- PHI 生成の要件は同じだが、実行環境が異なる

## selfhost 経路との違い

| 項目 | selfhost 経路 | hako_check 経路 |
|------|---------------|-----------------|
| **目的** | .hako コンパイラ実行 | .hako 静的解析 |
| **実行** | VM/LLVM で実行 | MIR 生成のみ（VM で .hako スクリプト実行） |
| **PHI 生成** | 実行時に必要 | 検証用のみ |
| **エラー処理** | 実行時エラー | 静的エラー |
| **環境変数** | `NYASH_USE_NY_COMPILER=1` | `NYASH_DISABLE_PLUGINS=1` |
| **実装形式** | Rust バイナリ | .hako スクリプト（VM で実行） |

## Phase 122+ 実装計画

### Phase 122: 環境変数で JoinIR 選択可能に

**実装内容**:
- [ ] `NYASH_HAKO_CHECK_JOINIR=1` 環境変数追加
- [ ] MirBuilder の `cf_if()` / `cf_loop()` で環境変数確認
- [ ] 条件分岐で JoinIR 経路 or 旧経路を選択

**実装箇所**:
- `src/mir/builder/if_form.rs`: If 文の JoinIR 統合
- `src/mir/builder/control_flow.rs`: Loop の JoinIR 統合拡張

**テスト**:
- [ ] 既存テスト全 PASS（旧経路）
- [ ] JoinIR 経路でのスモークテスト作成

### Phase 123: JoinIR 経路をデフォルトに

**実装内容**:
- [ ] デフォルトを JoinIR 経路に変更
- [ ] `NYASH_LEGACY_PHI=1` で旧経路に戻せるように

**テスト**:
- [ ] JoinIR 経路で全テスト PASS
- [ ] 旧経路でも互換性維持確認

### Phase 124: 旧経路完全削除

**実装内容**:
- [ ] 旧 PHI 生成器削除（`if_form.rs` の旧ロジック削除）
- [ ] `NYASH_LEGACY_PHI=1` 環境変数削除
- [ ] 関連ドキュメント更新

**テスト**:
- [ ] JoinIR 経路のみで全テスト PASS

## まとめ

Phase 121 は設計と調査のみ。実装は Phase 122+ で段階的に実施する。

**重要な発見**:
- hako_check は .hako スクリプトとして実装されている
- JoinIR 統合は VM の MirBuilder に実装する必要がある
- Loop は部分統合済み（Phase 49）、If は未統合
- 段階的移行戦略により互換性を維持しながら統合可能

---

## Phase 124: 実行フロー図（JoinIR 専用化完了）

### JoinIR Only パス (Phase 124 以降)

```
.hako file
    ↓
Tokenize / Parse (Rust Parser)
    ↓
AST Generation
    ↓
MIR Builder (JoinIR lowering for if/loop)
    ├─ cf_if() → lower_if_form() (JoinIR-based PHI generation)
    └─ cf_loop() → LoopBuilder (JoinIR-based PHI generation)
    ↓
MIR Generation (with JoinIR PHI)
    ↓
VM Interpreter
    ↓
Execution Result
```

### Phase 124 実装状態

**✅ 実装完了**:
- NYASH_HAKO_CHECK_JOINIR フラグ完全削除
- MIR Builder から legacy if/loop lowering 分岐削除
- JoinIR 一本化（Fail-Fast エラーハンドリング）
- 代表テスト 4 ケース全て JoinIR Only で PASS

### 実行時の動作確認

```bash
# JoinIR Only (デフォルト - 環境変数不要)
$ ./target/release/hakorune --backend vm test.hako
[MirBuilder] cf_if() → lower_if_form() (JoinIR)
RC: 0

# デバッグ出力 (環境変数は削除済み)
$ ./target/release/hakorune --backend vm test.hako
[MirBuilder] Using JoinIR lowering (legacy path removed)
```

### アーキテクチャの進化

**Phase 121**: JoinIR 統合設計確立
**Phase 123**: 環境変数フラグで 2パス選択可能
**Phase 124**: JoinIR 一本化 & レガシー削除 ✅

### 設計原則

**Fail-Fast**:
- JoinIR が失敗した場合、明示的にエラーを返す
- フォールバック処理は完全削除（CLAUDE.md Fail-Fast 原則遵守）

**JoinIR 専用化の利点**:
- コードベース簡素化（環境変数分岐削除）
- テスト対象経路の一本化
- 保守性向上

---

## Phase 123 実装完了記録 (2025-12-04)

### 変更ファイル一覧

1. `src/config/env/hako_check.rs` (新規) - 環境変数フラグ実装
2. `src/config/env.rs` - hako_check モジュール追加
3. `src/mir/builder/control_flow.rs` - JoinIR スイッチ実装
4. `docs/reference/environment-variables.md` - ドキュメント更新
5. `local_tests/phase123_*.hako` (4件) - テストケース作成
6. `tools/smokes/v2/profiles/integration/hako_check_joinir.sh` - テストスクリプト

### テスト結果

**Legacy Path**: 4/4 PASS (100%)
**JoinIR Path**: 4/4 PASS (100%)

**Note**: JoinIR 経路はプレースホルダー実装のため、実際にはレガシー経路で処理。
環境変数読み取りとフラグ分岐は完全に動作しており、Phase 124 で JoinIR 実装を追加すれば即座に動作可能。


## HC020: Unreachable Basic Block (Phase 154)

**Rule ID:** HC020
**Severity:** Warning
**Category:** Dead Code (Block-level)

### Description

Detects unreachable basic blocks using MIR CFG information. Complements HC019 by providing fine-grained analysis at the block level rather than method level.

### Patterns Detected

1. **Early return**: Code after unconditional return
2. **Constant conditions**: Branches that can never be taken (`if 0`, `if false`)
3. **Infinite loops**: Code after `loop(1)`
4. **Unconditional break**: Code after break statement

### Usage

```bash
# Enable HC020 alone
./tools/hako_check.sh --dead-blocks program.hako

# Combined with HC019
./tools/hako_check.sh --dead-code --dead-blocks program.hako

# Via rules filter
./tools/hako_check.sh --rules dead_blocks program.hako
```

### Example Output

```
[HC020] Unreachable basic block: fn=Main.test bb=5 (after early return) :: test.hako:10
[HC020] Unreachable basic block: fn=Foo.bar bb=12 (dead conditional) :: test.hako:25
```

### Requirements

- Requires MIR CFG information (Phase 154+)
- Gracefully skips if CFG unavailable
- Works with NYASH_JOINIR_STRICT=1 mode

### Implementation

- **Analyzer:** `tools/hako_check/rules/rule_dead_blocks.hako`
- **CFG Extractor:** `src/mir/cfg_extractor.rs`
- **Tests:** `apps/tests/hako_check/test_dead_blocks_*.hako`

