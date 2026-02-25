Status: Historical

# Hakorune Legacy Code Investigation Report

**調査日**: 2025-11-06
**Phase**: 20.46 (Phase 21.0完成後)
**調査対象**: 削除可能なレガシーコード、デッドコード、不要な機能

---

## 📊 Executive Summary

- **総Rustファイル数**: 618ファイル
- **#[allow(dead_code)]**: 76箇所
- **レガシーコード推定**: 約10,000行 (全体の約10%)
- **削除推奨**: 約5,000〜7,000行 (段階的削除)

---

## 🎯 カテゴリ別分析

### 1. ⚠️ Cranelift/JITバックエンド (高優先度・Safe削除)

**状態**: Phase 15でアーカイブ済み (`archive/jit-cranelift/`)
**問題**: 本体コードに参照が残存

#### 影響範囲
- **参照ファイル数**: 27ファイル
- **feature gate**: `cranelift-jit` (Cargo.tomlで既にコメントアウト済み)
- **推定削除可能行数**: 約1,500〜2,000行

#### 主要ファイル

##### 削除推奨 (Safe)
```
src/runner/modes/cranelift.rs          (46行) - Safe削除
src/runner/modes/aot.rs                (66行) - Safe削除 (cranelift feature依存)
src/runner/jit_direct.rs              (推定200行) - Safe削除

src/tests/core13_smoke_jit.rs         (約100行) - Safe削除
src/tests/core13_smoke_jit_map.rs     (約100行) - Safe削除
src/tests/identical_exec*.rs          (7ファイル) - feature gateで無効化済み
src/tests/policy_mutdeny.rs           (206行) - 一部cranelift依存
```

##### 要検証 (Investigate)
```
src/backend/mod.rs:29-52              - cranelift再エクスポート (削除可能)
src/backend/aot/                      - AOTコンパイル機能 (WASM依存, 要確認)
src/runner/dispatch.rs                - cranelift分岐 (削除可能)
src/cli/args.rs                       - --backend cranelift オプション (削除可能)
```

#### 推奨アクション
1. **Phase 1**: feature gate確認・テストファイル削除 (Safe, 約500行)
2. **Phase 2**: runner/modesのcranelift.rs削除 (Safe, 約200行)
3. **Phase 3**: backend/mod.rs再エクスポート削除 (Safe, 約30行)
4. **Phase 4**: 残存参照の完全削除 (要テスト, 約1,000行)

**削除時影響**: 無し (feature既に削除済み、ビルドエラーなし)

---

### 2. 🌐 WASMバックエンド (低優先度・要確認)

**状態**: Feature-gated (`wasm-backend`), 使用頻度不明

#### 影響範囲
- **ディレクトリ**: `src/backend/wasm/`, `src/backend/wasm_v2/`, `src/backend/aot/`
- **総行数**: 約3,170行
- **ディスクサイズ**: 140KB
- **参照ファイル数**: 8ファイル

#### 主要ファイル
```
src/backend/wasm/codegen.rs           (851行)
src/backend/wasm/memory.rs            (426行)
src/backend/wasm/runtime.rs           (369行)
src/backend/aot/executable.rs         (301行)
src/backend/wasm_v2/*                 (約250行)
```

#### 判定: **Risky** - 現時点では保持推奨

**理由**:
- CLAUDE.mdで「⚠️ WASM機能: レガシーインタープリター削除により一時無効」と記載
- 将来的なWASM対応の可能性あり
- Phase 21.0完成後の方針次第

**推奨アクション**:
1. 使用状況調査 (WASMバックエンド実際に動作するか?)
2. 動作しない場合: `archive/wasm-backend/`に移動
3. 動作する場合: ドキュメント整備・テスト追加

---

### 3. 🗂️ BID Copilot Modules (中優先度・Safe削除)

**状態**: 未使用プロトタイプ (READMEで「現在は使用していない」と明記)

#### 影響範囲
- **ディレクトリ**: `src/bid-codegen-from-copilot/`, `src/bid-converter-copilot/`
- **総行数**: 約1,894行
- **ディスクサイズ**: 124KB
- **総ファイル数**: 13ファイル

#### 詳細

##### bid-codegen-from-copilot/ (88KB, 9ファイル)
```
schema.rs                              (約300行)
codegen/generator.rs                   (推定500行)
codegen/targets/wasm.rs                (詳細実装)
codegen/targets/{vm,llvm,python,ts}.rs (スタブ)
```

**用途**: 将来的なプラグイン多言語対応
**判定**: **Safe削除** - `archive/bid-copilot-prototype/`に移動

##### bid-converter-copilot/ (36KB, 4ファイル)
```
tlv.rs                                 (TLVエンコード/デコード)
types.rs                               (BID型定義)
error.rs                               (BIDエラー型)
```

**用途**: 将来的なnyash2.toml実装時に活用予定
**判定**: **Safe削除** - 汎用性高いが現在未使用、`archive/bid-converter/`に移動

#### 推奨アクション
1. **Phase 1**: `archive/bid-copilot-prototype/`にディレクトリごと移動
2. **Phase 2**: `src/lib.rs`のコメント削除 (現在はモジュール宣言なし)
3. **影響**: 無し (既に未使用、インポート0)

**削減見込み**: 約1,900行

---

### 4. 📄 JSON v1 Bridge (中優先度・要確認)

**状態**: 使用状況不明

#### 影響範囲
- **ファイル**: `src/runner/json_v1_bridge.rs` (734行)
- **#[allow(dead_code)]**: `try_parse_v1_to_module()` 関数

#### コード抜粋
```rust
#[allow(dead_code)]
pub fn try_parse_v1_to_module(json: &str) -> Result<Option<MirModule>, String> {
    // MIR JSON v1スキーマパース
    // 最小限のサポート: const, copy, ret
}
```

#### 判定: **Investigate** - 使用確認が必要

**調査項目**:
1. JSON v1を出力するコードは存在するか?
2. テストで使用されているか?
3. JSON v0 bridgeで代替可能か?

**推奨アクション**:
1. **Phase 1**: 使用状況調査 (grep検索・テスト実行)
2. **Phase 2**: 未使用なら`archive/json-v1-bridge.rs`に移動
3. **Phase 3**: 使用中なら機能追加 or ドキュメント整備

**削減見込み**: 約700行 (未使用の場合)

---

### 5. 🔧 #[allow(dead_code)] マーカー (高優先度・段階削除)

**状態**: 76箇所存在

#### 主要カテゴリ

##### PyVM/JSON v0 Bridge (保持推奨)
```
src/runner/modes/common_util/pyvm.rs   - PyVMハーネス (セルフホスト専用)
src/runner/modes/common_util/exec.rs   - 実行ユーティリティ
```
**判定**: **保持** - Phase 15セルフホスティングで使用中

##### Legacy Expression/Type Check
```
src/mir/builder/exprs_legacy.rs:6     - build_expression_impl_legacy()
src/mir/verification/legacy.rs:6      - check_no_legacy_ops()
src/mir/builder/utils.rs               - 旧型チェック関数
```
**判定**: **Safe削除** - Phase 15でCore-13/14/15に統一済み

##### Parser関連
```
src/parser/common.rs                   - unknown_span() 等
src/parser/sugar.rs                    - 旧シュガー構文
src/parser/declarations/*/validators.rs - 古いバリデーター
```
**判定**: **Investigate** - 実際の使用状況確認が必要

##### Box Factory Deprecated
```
src/box_factory/builtin_impls/*.rs     - 18箇所のDEPRECATEDマーカー
```
**判定**: **Risky** - プラグイン移行戦略次第 (後述)

#### 推奨アクション
1. **Phase 1**: 明確に未使用な関数削除 (約20箇所, Safe)
2. **Phase 2**: Legacy MIR関連削除 (約10箇所, Safe)
3. **Phase 3**: Parser関連の実使用確認・削除 (約15箇所, Investigate)
4. **Phase 4**: 残存コードのリファクタリング (約30箇所, Risky)

**削減見込み**: 約500〜1,000行

---

### 6. ⚠️ Builtin Box DEPRECATED (低優先度・Phase戦略依存)

**状態**: プラグイン移行計画中

#### 影響範囲
- **ディレクトリ**: `src/box_factory/builtin_impls/`
- **総行数**: 約264行
- **DEPRECATEDマーカー**: 18箇所

#### 対象Box
```
console_box.rs    - ⚠️ nyash-console-plugin推奨 (既存)
string_box.rs     - ⚠️ nyash-string-plugin推奨
array_box.rs      - ⚠️ nyash-array-plugin推奨
bool_box.rs       - ⚠️ nyash-bool-plugin推奨 (未作成)
integer_box.rs    - ⚠️ nyash-integer-plugin推奨
map_box.rs        - ⚠️ nyash-map-plugin推奨
```

#### 判定: **Risky** - Phase 15.5-B戦略次第

**理由**:
- CLAUDE.mdで「2本柱体制: Core Box統一化完了」と記載
- プラグイン移行は段階的実施中
- 削除すると既存コードが大量に壊れる可能性

**推奨アクション**:
1. **現状**: DEPRECATEDマーカーは保持
2. **Phase 15.5-B完了後**: プラグイン移行状況確認
3. **Phase 16**: builtin→plugin完全移行計画策定
4. **Phase 17**: builtinコード削除

**削減見込み**: 約200〜300行 (Phase 16以降)

---

### 7. 🧪 Legacy Test Files (中優先度・Safe削除)

**状態**: Feature-gatedで無効化済み

#### 影響範囲
- **総数**: 7ファイル (cranelift依存)
- **推定行数**: 約1,000〜1,500行

#### 主要ファイル
```
src/tests/core13_smoke_jit.rs          (#[cfg(feature = "cranelift-jit")])
src/tests/core13_smoke_jit_map.rs      (#[cfg(feature = "cranelift-jit")])
src/tests/identical_exec.rs            (一部cranelift依存)
src/tests/identical_exec_collections.rs
src/tests/identical_exec_instance.rs
src/tests/identical_exec_string.rs
src/tests/policy_mutdeny.rs            (206行, 一部cranelift)
```

#### 判定: **Safe削除** - feature削除済み

**推奨アクション**:
1. **Phase 1**: cranelift feature完全依存ファイル削除 (5ファイル)
2. **Phase 2**: identical_exec系の整理 (4ファイル, 一部保持?)
3. **Phase 3**: policy_mutdeny.rsの分割 (cranelift部分のみ削除)

**削減見込み**: 約1,000行

---

### 8. 🐍 PyVM References (保持推奨)

**状態**: JSON v0 Bridge・セルフホスト専用

#### 影響範囲
- **参照数**: 102箇所
- **主要用途**: Phase 15セルフホスティング

#### 主要ファイル
```
src/runner/modes/pyvm.rs               - PyVM実行モード
src/runner/json_v0_bridge/             - JSON v0→MIR変換
src/llvm_py/pyvm/                      - Python VM実装
```

#### 判定: **保持** - Phase 15現役使用中

**理由**:
- CLAUDE.mdで「PyVM特化保持 (JSON v0ブリッジ・using処理のみ)」と明記
- Phase 15.5-Bセルフホスティング実装で必須
- 削除不可

**推奨アクション**: なし (現状維持)

---

## 📈 削減見込みサマリー

| カテゴリ | 推定行数 | 削除難易度 | 優先度 | 削減見込み |
|---------|---------|-----------|-------|----------|
| Cranelift/JIT | 1,500〜2,000行 | Safe | 高 | 1,500行 |
| WASM Backend | 3,170行 | Risky | 低 | 0行 (保持) |
| BID Copilot | 1,894行 | Safe | 中 | 1,900行 |
| JSON v1 Bridge | 734行 | Investigate | 中 | 700行? |
| #[allow(dead_code)] | 500〜1,000行 | Mixed | 高 | 500行 |
| Builtin Box | 264行 | Risky | 低 | 0行 (Phase 16) |
| Legacy Tests | 1,000行 | Safe | 中 | 1,000行 |
| PyVM | N/A | N/A | N/A | 0行 (保持) |
| **合計** | **約9,000行** | - | - | **約5,600行** |

---

## 🎯 段階的削除計画

### Phase A: 安全削除 (即実行可能)
**推定削減**: 約2,500行

1. **Cranelift JITファイル削除**
   - `src/runner/modes/cranelift.rs`
   - `src/runner/jit_direct.rs`
   - `src/tests/core13_smoke_jit*.rs` (2ファイル)

2. **BID Copilotアーカイブ**
   - `src/bid-codegen-from-copilot/` → `archive/`
   - `src/bid-converter-copilot/` → `archive/`

3. **明確なDead Code削除**
   - `#[allow(dead_code)]`で未使用確認済み関数 (約20箇所)

**リスク**: 無し (feature削除済み、未使用コード)

### Phase B: 調査後削除 (1週間)
**推定削減**: 約1,500行

1. **JSON v1 Bridge調査**
   - 使用状況確認 (grep + テスト実行)
   - 未使用なら削除 or アーカイブ

2. **Legacy Test Files整理**
   - `identical_exec*.rs` 系統の再評価
   - cranelift依存部分の削除

3. **Parser関連Dead Code**
   - 実使用確認後、未使用関数削除

**リスク**: 低 (調査後判断)

### Phase C: 段階的削除 (Phase 16以降)
**推定削減**: 約1,600行

1. **WASM Backend評価**
   - 動作確認 → 動作しないならアーカイブ

2. **Builtin Box移行**
   - Phase 15.5-B完了後
   - プラグイン移行戦略確定後

**リスク**: 中 (Phase戦略依存)

---

## 🚨 削除時の注意点

### 1. Feature Gate確認
```bash
# 削除前に必ず確認
cargo build --release                    # デフォルト
cargo build --release --features llvm    # LLVM
cargo test                               # テスト通過確認
```

### 2. Archive戦略
削除ではなく **アーカイブ** を推奨:
```
archive/
├── jit-cranelift/          (既存)
├── bid-copilot-prototype/  (新規)
├── bid-converter/          (新規)
├── json-v1-bridge/         (新規)
└── wasm-backend/           (Phase C)
```

### 3. Git履歴保持
```bash
# ファイル削除時はgit mvを使用 (履歴保持)
git mv src/runner/modes/cranelift.rs archive/jit-cranelift/runner_modes_cranelift.rs
```

### 4. ドキュメント更新
- CLAUDE.md
- README.md
- 削除理由を CHANGELOG.md に記録

---

## 📋 推奨実行順序

### 今すぐ実行 (Safe削除)
```bash
# 1. Cranelift JIT削除
rm src/runner/modes/cranelift.rs
rm src/runner/jit_direct.rs
rm src/tests/core13_smoke_jit*.rs

# 2. BID Copilotアーカイブ
mkdir -p archive/bid-copilot-prototype
git mv src/bid-codegen-from-copilot archive/bid-copilot-prototype/
git mv src/bid-converter-copilot archive/bid-copilot-prototype/

# 3. ビルド確認
cargo build --release
cargo test
```

### 1週間以内 (調査後削除)
1. JSON v1 Bridge使用状況調査
2. Legacy Test Files整理
3. Parser Dead Code削除

### Phase 16以降 (戦略確定後)
1. WASM Backend評価・アーカイブ
2. Builtin Box→Plugin完全移行
3. 残存Legacy MIR削除

---

## 📊 Impact Analysis

### 削除後の効果
- **コードベース削減**: 約5,600行 (約6%)
- **保守性向上**: レガシーコード除去で可読性向上
- **ビルド時間短縮**: 未使用コード削除で微減
- **Phase 15集中**: セルフホスティング開発に集中可能

### リスク評価
- **Phase A (Safe削除)**: リスク無し
- **Phase B (調査後削除)**: 低リスク
- **Phase C (Phase 16以降)**: 中リスク (戦略依存)

---

## 🔍 補足調査項目

### 1. 追加調査が必要な項目
- [ ] JSON v1 Bridgeの実使用確認
- [ ] WASM Backendの動作確認
- [ ] identical_exec系テストの保持必要性
- [ ] Parser関連Dead Codeの実使用状況

### 2. 今回調査対象外
- `crates/`ディレクトリ (別途調査推奨)
- `apps/`ディレクトリ (スクリプト例・テストケース)
- `tools/`ディレクトリ (ビルドスクリプト)
- Pythonコード (`src/llvm_py/`)

---

## 📝 結論

### 即実行推奨
✅ **Cranelift/JIT削除** (1,500行, Safe)
✅ **BID Copilotアーカイブ** (1,900行, Safe)
✅ **明確なDead Code削除** (500行, Safe)

**合計削減見込み**: 約3,900行 (即実行可能)

### 段階的実施
🔍 **JSON v1 Bridge調査** (700行?, 1週間)
🔍 **Legacy Test Files整理** (1,000行, 1週間)
⏳ **WASM Backend評価** (3,170行, Phase C)
⏳ **Builtin Box移行** (264行, Phase 16)

**追加削減見込み**: 約1,700〜5,000行 (段階的)

### 総削減見込み
**5,600〜8,900行** (6〜9%) の削減が可能

---

**調査担当**: Claude Code
**次のアクション**: Phase A (Safe削除) の実行
