Status: Historical

# Nyash ドキュメント再編成計画 📚

## 📊 現状の分析結果

### ファイル数統計
- **総ファイル数**: 283個（.md: 211個, .txt: 72個）
- **archive/**: 113ファイル（40%）
- **予定/**: 111ファイル（39%）  
- **説明書/**: 41ファイル（14%）
- **その他**: 18ファイル（7%）

### 重複・散在ファイル
- **README系**: 18個（各ディレクトリに散在）
- **GETTING_STARTED**: 3バージョン存在
- **Phase関連**: 42ファイル（主に予定/native-plan/issues/）
- **AI相談記録**: 41ファイル（複数箇所に散在）
- **ビルドログ**: 13ファイル（archive各所）

## 🔍 現状の問題点

### 1. **構造の混乱**
- トップレベルに開発中/正式文書が混在
- `説明書/` と直下に同じような内容が散在
- `archive/` に無秩序に大量のファイル

### 2. **重複と不明確な階層**
- `説明書/reference/` と `reference/` が併存
- GETTING_STARTED.md が複数存在（通常版と2025版）
- MIR関連ドキュメントが複数箇所に分散

### 3. **AI相談記録の散在**
- gemini/chatgpt相談が archive/, トップ, design/decisions/ など複数箇所
- ビルドログも同様に散在

### 4. **開発フェーズ管理の複雑さ**
- Phase関連ファイルが `予定/native-plan/issues/` に大量
- 現在のタスクと将来計画が混在

## 📋 提案する新構造

```
docs/
├── README.md                    # ドキュメントマップ（どこに何があるか）
│
├── 📖 reference/               # 正式な技術仕様（安定版）
│   ├── README.md
│   ├── language/               # 言語仕様
│   │   ├── syntax.md          # 構文リファレンス
│   │   ├── types.md           # 型システム
│   │   ├── boxes.md           # Box仕様
│   │   └── delegation.md      # デリゲーション仕様
│   ├── architecture/          # アーキテクチャ
│   │   ├── overview.md        # 全体設計
│   │   ├── mir.md             # MIR仕様
│   │   ├── vm.md              # VM仕様
│   │   └── plugins.md         # プラグインシステム
│   └── api/                   # API仕様
│       ├── builtin-boxes.md   # ビルトインBox一覧
│       └── stdlib.md          # 標準ライブラリ
│
├── 📚 guides/                 # 利用者向けガイド
│   ├── README.md
│   ├── getting-started.md     # はじめに（統一版）
│   ├── tutorials/             # チュートリアル
│   │   ├── hello-world.md
│   │   ├── basic-boxes.md
│   │   └── p2p-apps.md
│   ├── examples/              # サンプルコード
│   └── playground.md          # プレイグラウンドガイド
│
├── 🔧 development/            # 開発者向け（進行中）
│   ├── README.md
│   ├── current/               # 現在の作業
│   │   ├── CURRENT_TASK.md
│   │   └── VM_CHANGES.md
│   ├── roadmap/               # 開発計画
│   │   ├── phases/            # Phase別計画
│   │   │   ├── phase-8/
│   │   │   ├── phase-9/
│   │   │   └── phase-10/
│   │   └── native-plan/       # ネイティブビルド計画
│   └── proposals/             # 提案・RFC
│       └── phase_9_78e.md
│
├── 🗄️ archive/                # アーカイブ（古い/歴史的文書）
│   ├── README.md              # アーカイブの説明
│   ├── consultations/         # AI相談記録
│   │   ├── gemini/
│   │   ├── chatgpt/
│   │   └── codex/
│   ├── decisions/             # 過去の設計決定
│   ├── build-logs/            # ビルドログ
│   └── old-versions/          # 古いドキュメント
│
└── 🎨 assets/                 # 画像・図表など
    ├── diagrams/
    └── screenshots/
```

## 🔄 移行計画

### Phase 1: 基本構造作成（優先度: 高）
1. 新しいディレクトリ構造を作成
2. README.md でドキュメントマップ作成
3. 現在進行中のファイルを `development/current/` へ移動

### Phase 2: リファレンス整理（優先度: 高）
1. `説明書/reference/` の内容を `reference/` に統合
2. 重複ファイルの統合（GETTING_STARTED統一版作成）
3. MIR/VM関連ドキュメントを適切な場所に配置

### Phase 3: アーカイブ整理（優先度: 中）
1. AI相談記録を `archive/consultations/` に集約
2. ビルドログを `archive/build-logs/` に集約
3. 古いドキュメントを `archive/old-versions/` へ

### Phase 4: 開発ドキュメント整理（優先度: 中）
1. Phase関連ファイルを番号順に整理
2. 現在のタスクと将来計画を明確に分離
3. native-plan を整理して見やすく

## 💡 メリット

1. **明確な分類**: 利用者向け/開発者向け/アーカイブが明確
2. **検索性向上**: 必要な情報がどこにあるか分かりやすい
3. **メンテナンス性**: 新しいドキュメントの追加場所が明確
4. **バージョン管理**: 現在の仕様と過去の記録が分離

## 🤔 検討事項

### 日本語ディレクトリ名の変換案
現在の日本語ディレクトリを英語に統一：
- `説明書/` → `guides/` に内容を移動
- `予定/` → `development/roadmap/` に内容を移動
- 日本語ファイル名（.txt/.md）はそのまま維持（内容が日本語のため）

**理由**:
- 国際的な開発者にもアクセスしやすい
- パス指定時のエンコーディング問題を回避
- Git操作が簡単に

### 既存リンクの対応
- 多くの場所から参照されている可能性
- → **対策**: 移行期間は旧パスにシンボリックリンクまたはREADME配置

### 自動生成ドキュメント
- MIRダンプやベンチマーク結果など
- → **提案**: `development/generated/` または `archive/generated/` に配置

## 📝 実装順序の提案

### Step 1: 基本構造作成スクリプト
```bash
#!/bin/bash
# create_new_structure.sh

# 新構造の作成
mkdir -p reference/{language,architecture,api}
mkdir -p guides/{tutorials,examples}
mkdir -p development/{current,roadmap/{phases/{phase-8,phase-9,phase-10},native-plan},proposals}
mkdir -p archive/{consultations/{gemini,chatgpt,codex},decisions,build-logs,old-versions}
mkdir -p assets/{diagrams,screenshots}

# 各ディレクトリにREADME.mdを配置
echo "# Reference Documentation" > reference/README.md
echo "# User Guides" > guides/README.md
echo "# Development Documentation" > development/README.md
echo "# Archive" > archive/README.md
```

### Step 2: 優先移動ファイルリスト
1. **現在の作業ファイル**
   - `CURRENT_TASK.md` → `development/current/`
   - `CURRENT_VM_CHANGES.md` → `development/current/`
   - `phase_9_78e_summary.md` → `development/current/`

2. **言語仕様**
   - `LANGUAGE_REFERENCE_2025.md` → `reference/language/`
   - `TECHNICAL_ARCHITECTURE_2025.md` → `reference/architecture/`
   - `説明書/reference/` の内容 → `reference/` へ統合

3. **利用者ガイド**
   - GETTING_STARTED系を統合 → `guides/getting-started.md`
   - `説明書/guides/` → `guides/tutorials/`

### Step 3: 自動整理スクリプト案
```bash
#!/bin/bash
# reorganize_docs.sh

# Phase関連ファイルの整理
find 予定/native-plan/issues -name "phase_*.md" | while read f; do
    phase=$(echo $f | grep -o 'phase_[0-9]\+' | head -1)
    mkdir -p development/roadmap/phases/$phase
    mv "$f" development/roadmap/phases/$phase/
done

# AI相談記録の集約
find . -name "*gemini*.txt" -exec mv {} archive/consultations/gemini/ \;
find . -name "*chatgpt*.txt" -exec mv {} archive/consultations/chatgpt/ \;
find . -name "*consultation*.txt" -exec mv {} archive/consultations/ \;

# ビルドログの集約
find . -name "*build*.log" -o -name "*build*.txt" -exec mv {} archive/build-logs/ \;
```

## 🚀 実装提案

1. **まず `DOCUMENTATION_REORGANIZATION_PLAN.md` の承認**
2. **バックアップ作成**
   ```bash
   tar -czf docs_backup_$(date +%Y%m%d).tar.gz docs/
   ```
3. **基本ディレクトリ構造の作成**（上記スクリプト）
4. **段階的移行**（優先度順）
5. **リンク切れチェック**
6. **最終確認とクリーンアップ**

## 📋 具体的な移行マッピング（主要ファイル）

### 現在のトップレベルファイル
```
docs/
├── CURRENT_TASK.md                → development/current/
├── CURRENT_VM_CHANGES.md          → development/current/
├── LANGUAGE_REFERENCE_2025.md     → reference/language/
├── TECHNICAL_ARCHITECTURE_2025.md → reference/architecture/
├── Phase-9.75g-0-BID-FFI-*.md    → development/roadmap/phases/phase-9/
├── README.md                      → そのまま（インデックスとして）
├── execution-backends.md          → reference/architecture/
├── nyash_core_concepts.md         → reference/language/
└── plugin-migration-*.md         → reference/plugin-system/
```

### archiveの主要整理
```
archive/
├── chatgpt5/              → archive/consultations/chatgpt/
├── codex-analysis/        → archive/consultations/codex/
├── design/                → 一部をreference/へ、残りはarchive/decisions/
├── build_logs/            → archive/build-logs/に統合
└── *.txt (相談記録)      → archive/consultations/へ分類
```

### 説明書/の再構成
```
説明書/
├── GETTING_STARTED*.md    → guides/getting-started.md (統合版)
├── guides/                → guides/tutorials/
├── reference/             → reference/ (トップレベルに移動)
└── wasm/                  → guides/wasm-guide/
```

## ✅ チェックリスト

移行前の確認事項：
- [ ] バックアップの作成
- [ ] 重要なドキュメントの特定
- [ ] 外部からのリンク確認（README、コード内のコメント等）
- [ ] CIスクリプトでのパス参照確認

移行後の確認事項：
- [ ] すべてのドキュメントが新構造に配置されているか
- [ ] 重複ファイルが統合されているか
- [ ] 各ディレクトリにREADME.mdがあるか
- [ ] リンク切れがないか

どうでしょうか？この計画で進めてよろしいですか？

もし承認いただければ、まず小規模なテスト移行から始めることもできます。

### 🎯 期待される効果
- **検索時間短縮**: 必要な情報への到達が3クリック以内に
- **メンテナンス性向上**: 新規ドキュメントの追加場所が明確
- **重複削減**: 18個のREADME → 5個程度に集約
- **整理度**: 283ファイル → 適切に分類された構造
