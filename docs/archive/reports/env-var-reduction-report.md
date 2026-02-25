Status: Historical

# 環境変数徹底調査レポート - 25個から5個への削減計画

**作成日**: 2025-11-21
**目的**: セルフホスティング実装における環境変数の複雑さを根本解決

---

## 📊 使用箇所マトリックス

| 環境変数 | 定義箇所 | 読み取り回数 | 影響範囲 | デフォルト値 | カテゴリ |
|---------|---------|------------|---------|------------|---------|
| **Stage1関連** |
| `NYASH_USE_STAGE1_CLI` | stage1_bridge.rs:93 | 7回 | 必須（Stage1起動） | なし | Stage1制御 |
| `STAGE1_EMIT_PROGRAM_JSON` | stage1_bridge.rs:118 | 5回 | オプション（emit mode） | OFF | Stage1制御 |
| `STAGE1_EMIT_MIR_JSON` | stage1_bridge.rs:119 | 5回 | オプション（emit mode） | OFF | Stage1制御 |
| `STAGE1_BACKEND` | stage1_bridge.rs:157 | 5回 | オプション（backend選択） | vm | Stage1制御 |
| `STAGE1_SOURCE` | stage1_bridge.rs:115 | 6回 | オプション（入力ソース） | 第1引数 | Stage1制御 |
| `STAGE1_INPUT` | stage1_bridge.rs:116 | 1回 | オプション（入力ソース別名） | なし | Stage1制御 |
| `STAGE1_PROGRAM_JSON` | stage1_bridge.rs:135 | 5回 | オプション（中間JSON） | なし | Stage1制御 |
| `STAGE1_CLI_DEBUG` | stage1_cli.hako:27 | 11回 | オプション（デバッグ） | OFF | デバッグ |
| `NYASH_STAGE1_CLI_CHILD` | stage1_bridge.rs:90 | 3回 | 必須（再帰防止） | OFF | 内部制御 |
| **Using/Parser関連** |
| `NYASH_ENABLE_USING` | env.rs:429 | 10回 | オプション | **ON（デフォルト）** | 機能トグル |
| `HAKO_ENABLE_USING` | env.rs:435 | 8回 | 非推奨（互換性） | なし | 廃止予定 |
| `HAKO_STAGEB_APPLY_USINGS` | stage1_bridge.rs:224 | 6回 | オプション | ON | Stage1制御 |
| `NYASH_PARSER_STAGE3` | env.rs:540 | 38回 | オプション | OFF | 機能トグル |
| `HAKO_PARSER_STAGE3` | env.rs:543 | 15回 | 非推奨（互換性） | なし | 廃止予定 |
| **Runtime/Plugin関連** |
| `NYASH_DISABLE_PLUGINS` | plugins.rs:26 | 20回 | オプション | OFF | プラグイン制御 |
| `NYASH_FILEBOX_MODE` | provider_env.rs:37 | 8回 | オプション | auto | プラグイン制御 |
| `NYASH_BOX_FACTORY_POLICY` | mod.rs:135 | 9回 | オプション | builtin_first | プラグイン制御 |
| **Module/Config関連** |
| `HAKO_STAGEB_MODULES_LIST` | stage1_bridge.rs:239 | 5回 | オプション（モジュール一覧） | なし | Stage1制御 |
| `NYASH_CONFIG` | なし | 0回 | **未使用** | なし | **削除済み（2025-11）** |
| **Entry/Execution関連** |
| `NYASH_ENTRY` | stage1_bridge.rs:185 | 6回 | オプション | Stage1CliMain.main/1 | エントリー制御 |
| `NYASH_SCRIPT_ARGS_JSON` | stage1_bridge.rs:167 | 13回 | オプション | [] | 引数渡し |
| `NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN` | env.rs:576 | 8回 | オプション | **ON（デフォルト）** | エントリー制御 |
| **Debug/Verbose関連** |
| `NYASH_CLI_VERBOSE` | mod.rs:317 | 45回 | オプション | OFF | デバッグ |
| ~~`NYASH_DEBUG`~~ | なし | 0回 | **未使用 → 2025-11 削除** | なし | **削除済み** |
| `NYASH_NYRT_SILENT_RESULT` | stage1_bridge.rs:212 | 2回 | オプション | OFF | 出力制御 |

**合計**: 25個 → 現在 23個（NYASH_CONFIG / NYASH_DEBUG 削除後）
- **使用中**: 23個 → **21個**
- **未使用**: 0個（今回の2個を削除済み）
- **非推奨**: 2個（HAKO_ENABLE_USING, HAKO_PARSER_STAGE3）

---

## 🔍 依存関係分析

### グループ1: Stage1制御（9個 → 3個に統合可能）

**排他的関係**:
```
STAGE1_EMIT_PROGRAM_JSON=1  ─┐
                              ├─→ 排他的（1つだけ有効）
STAGE1_EMIT_MIR_JSON=1       ─┤
                              │
（なし：実行モード）          ─┘
```

**統合案**:
```bash
# 現在の複雑な設定
NYASH_USE_STAGE1_CLI=1 STAGE1_EMIT_PROGRAM_JSON=1 STAGE1_SOURCE=foo.hako

# 統合後（シンプル）
NYASH_STAGE1_MODE=emit-program NYASH_STAGE1_INPUT=foo.hako
```

**新変数**: `NYASH_STAGE1_MODE`
- 値: `emit-program | emit-mir | run`
- デフォルト: `run`
- 効果:
  - `emit-program` → `STAGE1_EMIT_PROGRAM_JSON=1`
  - `emit-mir` → `STAGE1_EMIT_MIR_JSON=1`
  - `run` → 実行モード

**削減できる変数**:
1. `NYASH_USE_STAGE1_CLI` → `NYASH_STAGE1_MODE` の存在で判定
2. `STAGE1_EMIT_PROGRAM_JSON` → `NYASH_STAGE1_MODE=emit-program`
3. `STAGE1_EMIT_MIR_JSON` → `NYASH_STAGE1_MODE=emit-mir`
4. `STAGE1_SOURCE` + `STAGE1_INPUT` → `NYASH_STAGE1_INPUT` に統合

---

### グループ2: Using制御（4個 → 1個に統合）

**統合案**:
```bash
# 現在の複雑な設定
NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 HAKO_STAGEB_APPLY_USINGS=1

# 統合後
NYASH_FEATURES=using  # カンマ区切りで複数機能対応
```

**削減できる変数**:
1. `NYASH_ENABLE_USING` → `NYASH_FEATURES=using`
2. `HAKO_ENABLE_USING` → 廃止（互換性エイリアス削除）
3. `HAKO_STAGEB_APPLY_USINGS` → `NYASH_FEATURES=using` で自動

---

### グループ3: Parser制御（2個 → 1個に統合）

**統合案**:
```bash
# 現在
NYASH_PARSER_STAGE3=1 HAKO_PARSER_STAGE3=1

# 統合後
NYASH_FEATURES=stage3  # または NYASH_PARSER_FEATURES=stage3
```

**削減できる変数**:
1. `NYASH_PARSER_STAGE3` → `NYASH_FEATURES=stage3`
2. `HAKO_PARSER_STAGE3` → 廃止

---

### グループ4: Plugin制御（3個 → 維持）

**現状維持推奨**:
- `NYASH_DISABLE_PLUGINS`: 全プラグイン無効化（重要）
- `NYASH_FILEBOX_MODE`: FileBox詳細制御
- `NYASH_BOX_FACTORY_POLICY`: Box Factory優先順位

**理由**: 独立した機能で統合するメリットが少ない

---

### グループ5: Debug制御（3個 → 1個に統合）

**統合案**:
```bash
# 現在
NYASH_CLI_VERBOSE=1 STAGE1_CLI_DEBUG=1

# 統合後
NYASH_DEBUG=1  # または NYASH_DEBUG_LEVEL=1
```

**削減できる変数**:
1. `NYASH_CLI_VERBOSE` → `NYASH_DEBUG=1`
2. `STAGE1_CLI_DEBUG` → `NYASH_DEBUG=1`
3. `NYASH_NYRT_SILENT_RESULT` → `NYASH_DEBUG=0` の時に自動ON

---

## 📋 削減ロードマップ

### Phase 1（即座に削除可能 - 2個）

**完全未使用（0回参照）**:

**削除対象（実施済み）**:
1. ✅ `NYASH_CONFIG`: 使用箇所0個（完全未使用、将来構想のみ）→ tools/stage1_* から除去済み。
2. ✅ `NYASH_DEBUG`: 使用箇所0個（NYASH_DEBUG_* は別変数、Phase 10構想のみ）→ tools/stage1_* から除去済み。

**影響**: なし（誰も使っていない）。削除は完了済み。

---

### Phase 2（非推奨エイリアス削除 - 2個）

**廃止予定（互換性のみ）**:

**削除対象**:
1. ⚠️ `HAKO_ENABLE_USING` → `NYASH_ENABLE_USING` に移行（警告済み）
2. ⚠️ `HAKO_PARSER_STAGE3` → `NYASH_PARSER_STAGE3` に移行（警告済み）

**影響**: 警告が出ているので移行済みのはず

**実装**: `src/config/env.rs` から互換性処理を削除

---

### Phase 3（Stage1統合 - 7個 → 3個）

**統合変数セット**:
```bash
# 新設計
NYASH_STAGE1_MODE=<emit-program|emit-mir|run>
NYASH_STAGE1_INPUT=<source.hako>
NYASH_STAGE1_BACKEND=<vm|llvm|pyvm>  # オプション
```

**削減できる変数**（7個 → 3個）:
1. `NYASH_USE_STAGE1_CLI` → MODE存在で自動判定
2. `STAGE1_EMIT_PROGRAM_JSON` → `MODE=emit-program`
3. `STAGE1_EMIT_MIR_JSON` → `MODE=emit-mir`
4. `STAGE1_SOURCE` + `STAGE1_INPUT` → `NYASH_STAGE1_INPUT`
5. `STAGE1_BACKEND` → `NYASH_STAGE1_BACKEND`
6. `STAGE1_PROGRAM_JSON` → 中間ファイル（環境変数不要）

**保持する変数**:
- `NYASH_STAGE1_CLI_CHILD`: 内部制御（外部非公開）

---

### Phase 4（Using/Parser統合 - 4個 → 1個）

**統合変数**:
```bash
# 新設計
NYASH_FEATURES=<using,stage3,unified-members>  # カンマ区切り
```

**削減できる変数**:
1. `NYASH_ENABLE_USING` → `FEATURES=using`
2. `HAKO_STAGEB_APPLY_USINGS` → `FEATURES=using` で自動
3. `NYASH_PARSER_STAGE3` → `FEATURES=stage3`

---

### Phase 5（Debug統合 - 3個 → 1個）

**統合変数**:
```bash
# 新設計
NYASH_DEBUG=<0|1|2|3>  # レベル制御
```

**削減できる変数**:
1. `NYASH_CLI_VERBOSE` → `DEBUG=1`
2. `STAGE1_CLI_DEBUG` → `DEBUG=2`（詳細）
3. `NYASH_NYRT_SILENT_RESULT` → `DEBUG=0` で自動ON

---

### Phase 6（nyash.toml化 - 4個）

**永続設定に移動すべき変数**:
```toml
[runtime]
disable_plugins = false
filebox_mode = "auto"
box_factory_policy = "builtin_first"

[entry]
allow_toplevel_main = true
```

**削減できる変数**（環境変数 → 設定ファイル）:
1. `NYASH_DISABLE_PLUGINS` → `runtime.disable_plugins`
2. `NYASH_FILEBOX_MODE` → `runtime.filebox_mode`
3. `NYASH_BOX_FACTORY_POLICY` → `runtime.box_factory_policy`
4. `NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN` → `entry.allow_toplevel_main`

**注意**: 環境変数は緊急時オーバーライド用に残す

---

## 🎯 最終推奨5変数セット

**Phase 1-5完了後の理想形**:

```bash
# 1. モード制御（Stage1）
NYASH_STAGE1_MODE=<emit-program|emit-mir|run>

# 2. 入力ソース
NYASH_STAGE1_INPUT=<source.hako>

# 3. デバッグレベル
NYASH_DEBUG=<0|1|2|3>

# 4. 機能トグル
NYASH_FEATURES=<using,stage3,unified-members>

# 5. バックエンド選択
NYASH_STAGE1_BACKEND=<vm|llvm|pyvm>
```

**削減実績**: 25個 → 5個（**80%削減**）

**補助変数**（内部制御・特殊用途）:
- `NYASH_STAGE1_CLI_CHILD`: 再帰防止（外部非公開）
- `NYASH_SCRIPT_ARGS_JSON`: 引数渡し（自動生成）
- `HAKO_STAGEB_MODULES_LIST`: モジュール一覧（自動生成）
- `NYASH_ENTRY`: エントリーポイント（特殊用途）

**nyash.toml化**（環境変数から設定ファイルへ）:
- `NYASH_DISABLE_PLUGINS`
- `NYASH_FILEBOX_MODE`
- `NYASH_BOX_FACTORY_POLICY`
- `NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN`

---

## 📈 削減効果まとめ

| Phase | 削減数 | 残数 | 削減率 |
|-------|--------|------|--------|
| 開始 | - | 25個 | - |
| Phase 1（未使用削除） | 2個 | 23個 | 8% |
| Phase 2（非推奨削除） | 2個 | 21個 | 16% |
| Phase 3（Stage1統合） | 4個 | 17個 | 32% |
| Phase 4（Using/Parser統合） | 3個 | 14個 | 44% |
| Phase 5（Debug統合） | 2個 | 12個 | 52% |
| Phase 6（nyash.toml化） | 4個 | 8個 | 68% |
| **補助変数移動後** | - | **5個（公開）** | **80%削減** |

**補助変数**（4個）:
- `NYASH_STAGE1_CLI_CHILD`（内部）
- `NYASH_SCRIPT_ARGS_JSON`（自動生成）
- `HAKO_STAGEB_MODULES_LIST`（自動生成）
- `NYASH_ENTRY`（特殊用途）

---

## 🚀 実装優先度

### 🔴 高優先度（即座に実行）
- **Phase 1**: 完全未使用削除（2個）
  - 影響: なし
  - 作業時間: 5分
  - コマンド: 上記参照

### 🟡 中優先度（1週間以内）
- **Phase 2**: 非推奨エイリアス削除（2個）
  - 影響: 警告表示済み
  - 作業時間: 30分
  - 注意: 1リリース後に削除推奨

### 🟢 低優先度（設計検討が必要）
- **Phase 3-5**: 統合変数設計（9個削減）
  - 影響: 大きい（破壊的変更）
  - 作業時間: 2-3日
  - 要件: 移行パス設計

- **Phase 6**: nyash.toml化（4個削減）
  - 影響: 中（環境変数残す）
  - 作業時間: 1日
  - 要件: TOML読み込み実装

---

## 🎓 学んだこと

1. **80/20ルール適用**:
   - 未使用変数2個（8%）を削除するだけで即効果
   - 非推奨変数2個（8%）も簡単に削除可能
   - 合計16%を簡単に削減できる

2. **統合可能性の発見**:
   - Stage1関連7個 → 3個（排他的制御）
   - Using/Parser関連4個 → 1個（機能フラグ統合）
   - Debug関連3個 → 1個（レベル制御統合）

3. **nyash.toml化のチャンス**:
   - Plugin制御3個は永続設定向き
   - Entry制御1個も永続設定向き
   - 環境変数は緊急時オーバーライド専用に

4. **内部変数の分離**:
   - `NYASH_STAGE1_CLI_CHILD`（再帰防止）
   - `NYASH_SCRIPT_ARGS_JSON`（自動生成）
   - これらは公開APIから除外可能

---

## 📚 参考情報

**主要ファイル**:
- `src/runner/stage1_bridge.rs`: Stage1制御
- `src/config/env.rs`: 環境変数読み取り
- `src/config/provider_env.rs`: Provider制御
- `lang/src/runner/stage1_cli.hako`: Stage1 CLI実装
- `tools/stage1_debug.sh`: デバッグツール

**現在の状況**:
- 合計25個の環境変数
- 使用中23個、未使用2個
- 非推奨2個（警告付き）

---

## 🌟 依存関係グラフ（視覚化）

### グループ構造

```
┌─────────────────────────────────────────────────────────────┐
│ Stage1制御グループ（9個 → 3個に統合可能）                    │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─ NYASH_USE_STAGE1_CLI ────┐                              │
│  │                             │                              │
│  ├─ STAGE1_EMIT_PROGRAM_JSON ─┼─→ NYASH_STAGE1_MODE         │
│  │                             │   (emit-program|emit-mir|run)│
│  └─ STAGE1_EMIT_MIR_JSON ─────┘                              │
│                                                               │
│  ┌─ STAGE1_SOURCE ────────────┐                              │
│  │                             ├─→ NYASH_STAGE1_INPUT        │
│  └─ STAGE1_INPUT ─────────────┘                              │
│                                                               │
│     STAGE1_BACKEND ───────────────→ NYASH_STAGE1_BACKEND     │
│                                                               │
│     STAGE1_PROGRAM_JSON ──────────→ （削除：中間ファイル）    │
│                                                               │
│     NYASH_STAGE1_CLI_CHILD ───────→ （保持：内部制御）       │
│                                                               │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ Using/Parser制御グループ（4個 → 1個に統合）                  │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─ NYASH_ENABLE_USING ──────┐                              │
│  │                             │                              │
│  ├─ HAKO_ENABLE_USING ────────┼─→ NYASH_FEATURES=using      │
│  │   （非推奨・廃止予定）      │                              │
│  │                             │                              │
│  └─ HAKO_STAGEB_APPLY_USINGS ─┘                              │
│                                                               │
│  ┌─ NYASH_PARSER_STAGE3 ─────┐                              │
│  │                             ├─→ NYASH_FEATURES=stage3     │
│  └─ HAKO_PARSER_STAGE3 ───────┘                              │
│      （非推奨・廃止予定）                                     │
│                                                               │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ Debug制御グループ（3個 → 1個に統合）                         │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌─ NYASH_CLI_VERBOSE ───────┐                              │
│  │                             │                              │
│  ├─ STAGE1_CLI_DEBUG ─────────┼─→ NYASH_DEBUG=<0|1|2|3>     │
│  │                             │                              │
│  └─ NYASH_NYRT_SILENT_RESULT ─┘                              │
│                                                               │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ Plugin制御グループ（3個 → 維持）                             │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│     NYASH_DISABLE_PLUGINS ────────→ （維持・重要）           │
│     NYASH_FILEBOX_MODE ───────────→ （維持・重要）           │
│     NYASH_BOX_FACTORY_POLICY ─────→ （維持・重要）           │
│                                                               │
│     ※ Phase 6で nyash.toml 化推奨                           │
│                                                               │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ 削除可能グループ（2個）                                       │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│     NYASH_CONFIG ──────────────────→ （削除：未使用）        │
│     NYASH_DEBUG ───────────────────→ （削除：未使用）        │
│                                                               │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ 補助変数グループ（4個 → 内部制御・自動生成）                 │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│     NYASH_ENTRY ───────────────────→ （特殊用途）            │
│     NYASH_SCRIPT_ARGS_JSON ────────→ （自動生成）            │
│     HAKO_STAGEB_MODULES_LIST ──────→ （自動生成）            │
│     NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN → （nyash.toml化推奨）  │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

### 排他的関係

```
Stage1モード（排他的）:
  ┌─────────────────────┐
  │ emit-program        │ ← STAGE1_EMIT_PROGRAM_JSON=1
  ├─────────────────────┤
  │ emit-mir            │ ← STAGE1_EMIT_MIR_JSON=1
  ├─────────────────────┤
  │ run (default)       │ ← 両方OFF
  └─────────────────────┘

  ※ 同時に複数ONにはできない（排他的）
```

### 統合後の最終形態

```
┌───────────────────────────────────────────────────────────┐
│ 公開環境変数（5個）                                        │
├───────────────────────────────────────────────────────────┤
│                                                             │
│  1. NYASH_STAGE1_MODE     : emit-program|emit-mir|run     │
│  2. NYASH_STAGE1_INPUT    : <source.hako>                 │
│  3. NYASH_DEBUG           : 0|1|2|3                       │
│  4. NYASH_FEATURES        : using,stage3,unified-members  │
│  5. NYASH_STAGE1_BACKEND  : vm|llvm|pyvm                  │
│                                                             │
└───────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────┐
│ 内部変数（4個・非公開）                                     │
├───────────────────────────────────────────────────────────┤
│                                                             │
│  - NYASH_STAGE1_CLI_CHILD   : 再帰防止（自動設定）         │
│  - NYASH_SCRIPT_ARGS_JSON   : 引数渡し（自動生成）         │
│  - HAKO_STAGEB_MODULES_LIST : モジュール一覧（自動生成）   │
│  - NYASH_ENTRY              : エントリーポイント（特殊）   │
│                                                             │
└───────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────┐
│ nyash.toml 設定（4個）                                     │
├───────────────────────────────────────────────────────────┤
│                                                             │
│  [runtime]                                                 │
│    disable_plugins = false                                 │
│    filebox_mode = "auto"                                   │
│    box_factory_policy = "builtin_first"                    │
│                                                             │
│  [entry]                                                   │
│    allow_toplevel_main = true                              │
│                                                             │
│  ※ 環境変数で緊急時オーバーライド可能                      │
│                                                             │
└───────────────────────────────────────────────────────────┘

削減実績: 25個 → 5個（公開） + 4個（内部） = 9個
削減率: 64%（25→9） or 80%（25→5 公開のみ）
```

### 移行パス

```
Phase 1 (即座)
  25個 ──┬─ 削除2個 ───→ 23個
         │  (NYASH_CONFIG, NYASH_DEBUG)
         │
Phase 2 (1週間)
  23個 ──┬─ 削除2個 ───→ 21個
         │  (HAKO_ENABLE_USING, HAKO_PARSER_STAGE3)
         │
Phase 3 (2-3日)
  21個 ──┬─ 統合4個 ───→ 17個
         │  (Stage1: 9個→3個で4個削減)
         │
Phase 4 (1-2日)
  17個 ──┬─ 統合3個 ───→ 14個
         │  (Using/Parser: 4個→1個で3個削減)
         │
Phase 5 (1日)
  14個 ──┬─ 統合2個 ───→ 12個
         │  (Debug: 3個→1個で2個削減)
         │
Phase 6 (1日)
  12個 ──┴─ toml化4個 ──→ 8個
           (環境変数残す・設定ファイル優先)

Final: 8個（公開5個 + 内部3個）
```

---

## ✅ 次のアクション

1. **今すぐ実行**（5分）: Phase 1 - 未使用変数2個削除
2. **1週間以内**（30分）: Phase 2 - 非推奨エイリアス削除
3. **設計検討**（1-2週間）: Phase 3-5 - 統合変数設計
4. **実装**（3-5日）: Phase 3-5 - 統合変数実装
5. **TOML実装**（1-2日）: Phase 6 - nyash.toml化

**最終目標**: 25個 → 5個（公開） + 4個（内部） = **80%削減達成**
