# selfhost Stage-3 期待フロー（Phase 120 時点）

## 用語メモ（Stage-B / Stage-3 / selfhost depth）

- **Stage-3**  
  - 言語／パーサのステージを指すよ。  
  - 「break/continue/try/throw を含む現在の最終構文」を Stage-3 構文と呼んでいて、Rust 版も selfhost 版もこの構文を受理できるコンパイラかどうかで Stage-3 かを判断する。

- **Stage-B**  
  - selfhost パイプライン内部の **特定フェーズのコンパイラ箱**（`compiler_stageb.hako` など）を指す技術用語だよ。  
  - `.hako → Program(JSON v0)` を作るスキャナ／エミッタの段階名であって、「self-host 全体の段階（何周目か）」を意味するラベルではない。

- **selfhost depth（1周目/2周目）**  
  - depth-1: Rust でビルドした Ny コンパイラ（Stage-3 構文対応）を使って、別の .hako プログラムや selfhost コンパイラ自身を 1 回ビルドするライン。  
  - depth-2: Ny でビルドされたコンパイラで、さらに Ny コンパイラをビルドし直しても同じ挙動になることを確認するライン（「自分で自分をビルドする」を 2 周以上回す、本来の self-host ゴール）。

本ドキュメントでの「selfhost Stage-3」は、

- Stage-3 構文を受理する selfhost コンパイラを使って、
- Rust → Ny コンパイラ(Stage-3) → JSON v0 → Rust VM/LLVM の **depth-1 代表パス**

を指す意味で使っているよ。Stage-B はあくまでその途中で使われるコンパイラ箱の名前だよ。

## 概要

Phase 106-115 完了時点での selfhost 経路（Stage-3 .hako コンパイラ）の動作フローを記録。

## Selfhost パイプライン構成図（Phase 150 統合版）

```
┌─────────────────────────────────────────────────────────────────┐
│ target/release/hakorune (Rust バイナリ)                        │
│   - Entry point: Rust で書かれた Nyash/Hakorune 実行器         │
│   - 環境変数: NYASH_FEATURES=stage3 NYASH_USE_NY_COMPILER=1    │
└─────────────────────────────────────────────────────────────────┘
                            ↓
                  [Stage-B: 関数スキャン + scaffold]
                  - 関数シグネチャのスキャン
                  - Program(JSON v0) scaffold 生成
                  - 初期 JSON v0 構造の構築
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│ stage1_cli.hako (JSON v0 scaffold)                             │
│   - Stage-B 出力の JSON v0 中間表現                            │
│   - CLI/using 解決前の状態                                      │
└─────────────────────────────────────────────────────────────────┘
                            ↓
                  [Stage-1: CLI/using 解決]
                  - CLI 引数処理
                  - using nyashstd 解決
                  - 名前空間解決
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│ stage1_output.hako (Stage-3 構文)                              │
│   - CLI/using 解決済みの .hako ソースコード                     │
│   - Stage-3 構文（break/continue/try/throw 含む）              │
└─────────────────────────────────────────────────────────────────┘
                            ↓
                  [Stage-3: 実際のコンパイル本体]
                  - MIR/JoinIR 生成
                  - 制御フロー解析（If/Loop Lowering）
                  - PHI 命令生成
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│ Program(JSON v0)                                                │
│   - 最終的な JSON v0 IR                                         │
│   - Rust VM または LLVM バックエンドで実行可能な形式           │
└─────────────────────────────────────────────────────────────────┘
                            ↓
                  [dev_verify: JSON v0 検証]
                  - JSON v0 形式の検証
                  - NewBox → birth() 呼び出し検証
                  - 実行準備処理
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│ VM/LLVM 実行                                                    │
│   - Rust VM: ./target/release/hakorune (デフォルト)            │
│   - LLVM: ./target/release/hakorune --backend llvm             │
└─────────────────────────────────────────────────────────────────┘
```

## 各ステージの責務

### Stage-B: 関数スキャン + scaffold
**目的**: .hako ファイルから関数構造をスキャンし、JSON v0 scaffold を生成

**処理内容**:
- 関数シグネチャ（名前、パラメータ、戻り値型）のスキャン
- Box 定義の抽出
- Program(JSON v0) の初期構造構築
- static box/method の認識

**入力**: `program.hako` (Stage-3 構文)
**出力**: `stage1_cli.hako` (JSON v0 scaffold)

### Stage-1: CLI/using 解決
**目的**: CLI 引数と using 宣言を解決し、実行可能な .hako ソースを生成

**処理内容**:
- `using nyashstd` の名前空間解決
- CLI 引数の処理（コマンドライン引数の解釈）
- グローバル変数の初期化
- 名前空間の統一化

**入力**: `stage1_cli.hako` (JSON v0 scaffold)
**出力**: `stage1_output.hako` (Stage-3 構文、using 解決済み)

### Stage-3: 実際のコンパイル本体
**目的**: 実際の MIR/JoinIR 生成と制御フロー解析を実行

**処理内容**:
- MIR (Middle-level Intermediate Representation) 生成
- JoinIR If/Loop Lowering
- 制御フロー解析（CFG 構築）
- PHI 命令生成（分岐・ループの値合流）
- 最適化パス（Optional）

**入力**: `stage1_output.hako` (Stage-3 構文)
**出力**: `Program(JSON v0)` (最終 IR)

**重要**: JoinIR Strict モード（`NYASH_JOINIR_STRICT=1`）では旧 PHI 経路へのフォールバックを禁止

### dev_verify: JSON v0 検証
**目的**: 生成された JSON v0 IR の整合性を検証

**処理内容**:
- JSON v0 形式の構文検証
- NewBox 命令後の birth() 呼び出し検証
- 制御フロー整合性チェック
- 実行準備処理（Box 初期化等）

**入力**: `Program(JSON v0)`
**出力**: 検証済み JSON v0（そのまま VM/LLVM に渡す）

**警告例**:
- `[warn] dev verify: NewBox ConsoleBox not followed by birth()`
- birth() 呼び出しが省略可能な設計では警告レベル

## 実行環境

- **VM バックエンド**: `./target/release/hakorune program.hako`（デフォルト）
- **LLVM バックエンド**: `./target/release/hakorune --backend llvm program.hako`
- **selfhost 有効**: `NYASH_FEATURES=stage3 NYASH_USE_NY_COMPILER=1`

## JoinIR Strict モードとは

**環境変数**: `NYASH_JOINIR_STRICT=1`

**目的**: JoinIR 経路で旧 MIR/PHI 経路へのフォールバックを禁止し、厳格に JoinIR Lowering のみを使用

**期待される動作**:
- ✅ If/Loop Lowering が完全に JoinIR 経由で動作
- ❌ 旧 PHI 生成器へのフォールバックは禁止（エラーで停止）
- ⚠️ 警告: フォールバック候補があれば警告出力

## 代表パスの期待フロー

### 1. peek_expr_block.hako（簡易パーサーテスト）

**ファイルパス**: `apps/tests/peek_expr_block.hako`

**プログラム内容**:
- match 式によるパターンマッチング（"0", "1", "_" パターン）
- ブロック式（複数文を含む式ブロック）
- 返り値の代入と返却

**期待される JoinIR 処理**:
- ✅ match 式が If Lowering で複数の条件分岐に変換
- ✅ ブロック式が正常に評価（最後の式が値として返却）
- ✅ NYASH_JOINIR_STRICT=1 でもエラーなし

**検証ポイント**:
- If 文の JoinIR Lowering が正常動作
- PHI 命令の生成（各分岐からの値の合流）
- ブロック式の値伝播

### 2. loop_min_while.hako（ループ・PHI 含む）

**ファイルパス**: `apps/tests/loop_min_while.hako`

**プログラム内容**:
- loop 構文（条件: `i < 3`）
- ループ変数の更新（`i = i + 1`）
- ループ内での print 呼び出し

**期待される JoinIR 処理**:
- ✅ Loop が JoinIR Loop Lowering で処理
- ✅ PHI 命令が正しく生成（ループ変数 i の合流）
- ✅ ループの終了条件が正しく評価

**検証ポイント**:
- Loop の JoinIR Lowering が正常動作
- Entry PHI（ループ変数の初期値と更新値の合流）
- Exit PHI（ループ終了時の値伝播）
- ⚠️ 警告: 旧 PHI 経路へのフォールバック候補があるかもしれない（Phase 120 調査対象）

### 3. esc_dirname_smoke.hako（実用スクリプト）

**ファイルパス**: `apps/tests/esc_dirname_smoke.hako`

**プログラム内容**:
- 複数のメソッド定義（esc_json, dirname, main）
- 文字列操作（substring, length, lastIndexOf）
- 複雑な制御構造（ネストした if 文、ループ）
- Box 操作（ConsoleBox の使用）

**期待される JoinIR 処理**:
- ✅ StringBox メソッド呼び出しが正常動作
- ✅ ConsoleBox の生成と使用が正常動作
- ✅ 複雑なネスト構造が JoinIR 経由で処理
- ✅ ループと条件分岐の組み合わせが正常動作

**検証ポイント**:
- 複数メソッドの呼び出し（me.esc_json, me.dirname）
- StringBox の基本操作（length, substring, lastIndexOf）
- ネストした制御構造の JoinIR Lowering
- ConsoleBox プラグインとの連携
- ⚠️ 警告: 複雑さによってはフォールバックや警告が出る可能性

## Phase 120 の目標

上記の「期待」と「実際の動作」を比較し、ギャップを記録する。
実装修正は Phase 122+ で行う（Phase 120 はベースライン確立のみ）。

## 実行コマンド例

```bash
# 基本実行（VM バックエンド）
NYASH_JOINIR_STRICT=1 NYASH_USE_NY_COMPILER=1 \
  ./target/release/nyash apps/tests/peek_expr_block.hako

# 詳細ログ付き実行
NYASH_JOINIR_STRICT=1 NYASH_USE_NY_COMPILER=1 NYASH_CLI_VERBOSE=1 \
  ./target/release/nyash apps/tests/loop_min_while.hako

# LLVM バックエンド（オプション）
NYASH_JOINIR_STRICT=1 NYASH_USE_NY_COMPILER=1 \
  ./target/release/nyash --backend llvm apps/tests/esc_dirname_smoke.hako
```

## 記録対象

Phase 120 実行調査では以下を記録する:

1. **実行結果**: 成功/警告あり/エラー
2. **エラーメッセージ**: 完全なエラーログ
3. **警告メッセージ**: JoinIR 関連の警告
4. **出力結果**: プログラムの標準出力
5. **特記事項**: 予期しない動作や注目すべき点

## Phase 122+ への課題

Phase 120 での記録をもとに、以下の優先順位で課題を整理する:

**優先度高（エラー）**:
- プログラムが実行できない致命的な問題

**優先度中（警告）**:
- 実行は成功するが、警告が出る問題
- JoinIR 経路の不完全性

**優先度低（最適化）**:
- 動作するが、改善の余地がある点
- パフォーマンス最適化の候補

---

**作成日**: 2025-12-04
**Phase**: 120（selfhost Stage-3 代表パスの安定化）
