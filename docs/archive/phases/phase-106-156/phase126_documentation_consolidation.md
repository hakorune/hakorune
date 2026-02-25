# Phase 126: ドキュメント統合と整理

## 目的

Phase 122-125 で実装された ConsoleBox 関連の複数のドキュメントを統合し、重複を排除して、ユーザーフレンドリーな一元的な参照ドキュメントにする。

## 現在の状況

### Phase 122-125 で作成されたドキュメント（計 ~58KB）

```
📄 phase122_consolebox_println_unification.md      (15K)  - println/log統一設計
📄 phase122_5_nyash_toml_fix.md                    (3.8K) - method_id修正記録
📄 phase123_consolebox_code_unification.md         (13K)  - WASM/非WASM統一化
📄 phase124_vm_method_dispatch_unification.md      (13K)  - TypeRegistry統合
📄 phase125_delete_deprecated_console_box.md       (13K)  - ビルトイン削除
```

### 既存の関連ドキュメント（計 ~97KB）

```
📄 core_boxes_design.md      (65K)  - Core Box の全体設計
📄 logging_policy.md         (21K)  - ログ出力ポリシー
📄 hako_logging_design.md    (11K)  - Hako コンパイラのログ設計
```

### 問題点

1. **重複性**: Phase 122 の println/log 説明が hako_logging_design.md と重複
2. **散在性**: ConsoleBox 関連の情報が 5 つのファイルに分散
3. **ナビゲーション困難**: ユーザーが「ConsoleBox について知りたい」時、どのファイルを読むべきか不明
4. **保守性低下**: 同じ情報を複数箇所で修正する必要がある

## 統合戦略

### 戦略 A: マスタードキュメント作成（推奨）

**新規作成**: `consolebox_complete_guide.md` (統合マスター)

このドキュメントに以下を含める：

1. **概要セクション**
   - ConsoleBox の歴史（Phase 122-125 での進化）
   - デザイン決定の背景

2. **ユーザーガイド**
   - API 使用方法（println, log, warn, error, clear）
   - WASM/ネイティブ環境での動作
   - 実装例

3. **アーキテクチャ設計**
   - println/log エイリアス設計（Phase 122）
   - WASM/非WASM の統一化（Phase 123）
   - TypeRegistry ベースのディスパッチ（Phase 124）
   - プラグインへの移行（Phase 125）

4. **実装者向けガイド**
   - TypeRegistry での slot 管理
   - VM Method Dispatch の仕組み
   - プラグイン ConsoleBox の拡張方法

5. **クロスリファレンス**
   - Phase 122-125 の詳細ドキュメントへのリンク
   - core_boxes_design.md との関連セクション
   - logging_policy.md との統合例

### 戦略 B: 既存ドキュメント更新

**修正対象**:

1. **core_boxes_design.md**
   - Phase 125 の削除についての Section 追加
   -「ConsoleBox は現在プラグイン」の明記

2. **logging_policy.md**
   - Phase 122.5 の method_id 統一の記述追加
   - println の推奨使用例

3. **hako_logging_design.md**
   - Phase 122 の println サポートについて記述

## 実装ステップ

### Step 1: マスタードキュメント作成

**ファイル**: `docs/development/current/main/consolebox_complete_guide.md`

```markdown
# ConsoleBox Complete Guide - デザイン・実装・運用

## 📖 目次
1. 概要・歴史
2. ユーザーガイド
3. アーキテクチャ設計
4. 実装者向けガイド
5. FAQ・トラブルシューティング

## 1. 概要・歴史

### ConsoleBox の進化（Phase 122-125）

**Phase 122**: println/log エイリアス統一
- println を log のエイリアスとして実装
- TypeRegistry で slot 400（log と同じ）に統一
- nyash.toml での method_id 修正（Phase 122.5）

**Phase 123**: WASM/非WASM コード統一
- マクロベースの統一化で重複削減
- 67行削減（27.3%削減）

**Phase 124**: TypeRegistry ベースの統一ディスパッチ
- String, Array, ConsoleBox を統一的に dispatch_by_slot で処理
- 100行削減（method.rs 簡略化）

**Phase 125**: ビルトイン ConsoleBox 削除
- src/box_factory/builtin_impls/console_box.rs 削除
- プラグインのみへ移行（"Everything is Plugin" 実現）
- 52行削減

### 現在の状態（Phase 126 以降）

✅ ConsoleBox はプラグインのみ
✅ Rust 実装（src/boxes/console_box.rs）は内部用
✅ TypeRegistry ベースのディスパッチ
✅ println/log 統一

## 2. ユーザーガイド

### 基本的な使用方法

```nyash
// ConsoleBox インスタンス作成
local console
console = new ConsoleBox()

// 通常ログ（推奨）
console.println("Hello, Nyash!")
console.log("Same as println")

// 警告・エラー
console.warn("This is a warning")
console.error("Something went wrong")

// 画面クリア
console.clear()
```

### WASM 環境での動作

ブラウザの開発者ツール（F12）のコンソールに出力されます。

### ネイティブ環境での動作

標準出力にプレフィックス付きで出力されます：

```
[Console LOG] Hello, Nyash!
[Console WARN] This is a warning
[Console ERROR] Something went wrong
[Console CLEAR]
```

## 3. アーキテクチャ設計

[Phase 122-125 の詳細設計ドキュメントへのリンク]
[core_boxes_design.md のセクション reference]

## 4. 実装者向けガイド

[TypeRegistry 統合]
[VM Method Dispatch]
[プラグイン拡張方法]

## 5. FAQ・トラブルシューティング

Q: println と log の違いは？
A: Phase 122 以降、完全に同じです（println = log のエイリアス）

Q: println が動作しない
A: プラグイン（libnyash_console_plugin.so）が読み込まれているか確認してください

Q: println と log を区別する必要があります
A: TypeRegistry の slot をカスタマイズして異なる slot を割り当てることは可能ですが、推奨されません。

```

### Step 2: 既存ドキュメント更新

#### core_boxes_design.md の更新

**新規セクション追加**: Section 19 "Phase 125 ConsoleBox Transition"

```markdown
## Section 19: Phase 125 ConsoleBox Migration to Plugin

### 背景
Phase 122-125 で ConsoleBox は完全にプラグインベースに移行しました。

### 実装内容
- ビルトイン ConsoleBox（src/box_factory/builtin_impls/console_box.rs）削除
- Rust 実装（src/boxes/console_box.rs）は内部用として保持
- プラグイン（libnyash_console_plugin.so）のみが対外インターフェース

### 利点
- "Everything is Plugin" 原則の完全実装
- ビルトイン Factory の複雑性低減
- プラグイン拡張性の向上

### 参照
- [Phase 125 詳細](phase125_delete_deprecated_console_box.md)
- [ConsoleBox 完全ガイド](consolebox_complete_guide.md)
```

#### logging_policy.md の更新

**新規セクション追加**: Section X "Phase 122 println/log Unification"

```markdown
## Phase 122: println/log 統一化

### 背景
従来、println と log は別々のメソッドとして実装されていました。

### 実装内容
- println を log のエイリアスとして統一
- TypeRegistry で両者を同じ slot (400) に割り当て
- nyash.toml での method_id の統一（Phase 122.5）

### 使用ガイドライン
- **推奨**: println を使用（ユーザー向け API）
- **非推奨**: log を使用（互換性のみのため）

### 参照
- [Phase 122 詳細](phase122_consolebox_println_unification.md)
```

### Step 3: Phase 122-125 ドキュメントに統合フラグを追加

各ドキュメントの冒頭に以下を追加：

```markdown
# Phase 122: ConsoleBox println/log Unification

⚠️ **Note**: このドキュメントは Phase 122 の実装記録です。
           統合的なガイドは [ConsoleBox 完全ガイド](consolebox_complete_guide.md) をご参照ください。

## 詳細情報
[各セクションへの reference]
```

### Step 4: ナビゲーション改善

既存ドキュメント（core_boxes_design.md, logging_policy.md）に **Related Documents** セクションを追加：

```markdown
## 📚 Related Documents

### ConsoleBox について知りたい場合
- [ConsoleBox 完全ガイド](consolebox_complete_guide.md) - 統合的なリファレンス
- [Phase 122-125 実装記録](phase122_*.md) - 詳細な実装背景

### ログ出力について知りたい場合
- [ログポリシー](logging_policy.md) - この文書
- [Hako ログ設計](hako_logging_design.md) - コンパイラ側
```

## 統合後のドキュメント構造

```
📁 docs/development/current/main/

📄 consolebox_complete_guide.md (新規, ~25KB)
   ├─ ユーザーガイド
   ├─ アーキテクチャ設計
   ├─ 実装者向けガイド
   └─ クロスリファレンス

📄 core_boxes_design.md (修正, +Section 19 ~2KB)
   └─ Phase 125 ConsoleBox Migration セクション追加

📄 logging_policy.md (修正, +~3KB)
   └─ Phase 122 println/log 統一化 セクション追加

📄 hako_logging_design.md (修正, +~2KB)
   └─ Phase 122 println サポート セクション追加

🗂️ Phase 122-125 実装記録（参考用）
   ├─ phase122_consolebox_println_unification.md
   ├─ phase122_5_nyash_toml_fix.md
   ├─ phase123_consolebox_code_unification.md
   ├─ phase124_vm_method_dispatch_unification.md
   └─ phase125_delete_deprecated_console_box.md
```

## 実装上の注意

### 1. 重複排除のポイント

- **printf/log API説明**: 統合マスターに集約
- **TypeRegistry slot 定義**: 一度だけ説明
- **Phase 122-125 背景**: 統合マスターで説明

### 2. クロスリファレンスの管理

各ドキュメント間で相互参照を追加：

```markdown
[Related: Phase 122 実装記録](phase122_consolebox_println_unification.md)
[参照: TypeRegistry 設計](../architecture/type-registry-design.md)
```

### 3. バージョン管理

統合マスターの冒頭に：

```markdown
## 📅 Document Version
- **Last Updated**: Phase 126
- **Scope**: ConsoleBox API, Architecture, Implementation
- **Applies to**: Release X.Y.Z
```

## テスト・検証

### ドキュメント品質確認

1. **リンク確認**
   ```bash
   # 内部リンク（相対パス）が正しいか確認
   rg "\[.*\]\(\..*\.md\)" docs/development/current/main/
   ```

2. **重複確認**
   ```bash
   # 同じコード例やセクションが複数ドキュメントに無いか
   rg "console\.println|console\.log" docs/development/current/main/*.md | sort | uniq -c
   ```

3. **完全性確認**
   - すべての Phase 122-125 の情報が統合マスターに含まれるか
   - すべてのクロスリファレンスが有効か

## 期待される効果

### ドキュメント削減
- 統合マスター作成: +25KB
- 既存ドキュメント追加: +7KB
- **合計**: +32KB の新規マスター追加

### ナビゲーション改善
- 単一のエントリーポイント（consolebox_complete_guide.md）
- 階層的なセクション構造
- クロスリファレンスによる相互連携

### 保守性向上
- 重複説明を排除
- 同じ情報を一度だけ更新すれば OK
- Phase 122-125 の記録は参考用として保持

## ロールバック計画

修正後に問題が発生した場合：

```bash
# 修正前の状態に戻す
git checkout HEAD~ -- docs/development/current/main/
```

## 所要時間

**2時間程度**

- 統合マスター作成: 1時間
- 既存ドキュメント更新: 45分
- 検証・リンク確認: 15分

## 完了後

Phase 122-126 の全改善がコンプリート！

### 実装成果サマリー
- **Phase 122**: println/log エイリアス統一 ✅
- **Phase 122.5**: nyash.toml method_id 修正 ✅
- **Phase 123**: WASM/非WASM コード統一（67行削減） ✅
- **Phase 124**: TypeRegistry ディスパッチ統合（100行削減） ✅
- **Phase 125**: ビルトイン ConsoleBox 削除（52行削減） ✅
- **Phase 126**: ドキュメント統合・整理 ← **現在のフェーズ**

### 総削減コード量
**~219行削減** + **ドキュメント統合で保守性向上**

---

**進捗記録**:
- Phase 122.5: nyash.toml method_id 修正 ✅ 完了
- Phase 123: ConsoleBox WASM/非WASM 統一化 ✅ 完了
- Phase 124: VM Method Dispatch 統一化 ✅ 完了
- Phase 125: 削除：deprecated builtin ConsoleBox ✅ 完了
- Phase 126: ドキュメント統合 ← **現在のフェーズ**
Status: Historical
