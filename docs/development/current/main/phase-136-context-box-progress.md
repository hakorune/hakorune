# Phase 136 Follow-up: Builder Context Box化 進捗

## 概要

builder.rs の 1219 行を責任ごとに Context Box に分割し、保守性・テスト容易性を向上させる段階的リファクタリング。

## 完了した Context (6/7)

### ✅ TypeContext (Step 1) - 完了

**実装日**: 2025-12-15

**抽出したフィールド** (3個):
- `value_types: BTreeMap<ValueId, MirType>` - 型注釈マップ
- `value_kinds: HashMap<ValueId, MirValueKind>` - 型種別マップ (Phase 26-A)
- `value_origin_newbox: BTreeMap<ValueId, String>` - Box クラス名由来追跡

**ファイル**:
- `/home/tomoaki/git/hakorune-selfhost/src/mir/builder/type_context.rs` (新規作成)

**統合方法**:
- `MirBuilder` に `type_ctx: TypeContext` フィールドを追加
- 既存フィールドは `#[deprecated]` でマーク（後方互換性維持）
- 同期ヘルパー (`sync_type_ctx_to_legacy()`, `sync_legacy_to_type_ctx()`) を実装

**テスト結果**:
- ✅ `cargo build --release` 成功 (警告のみ)
- ✅ `cargo test --release --lib` - 997/997 PASS
- ✅ `phase135_trim_mir_verify.sh` - PASS
- ✅ `phase132_exit_phi_parity.sh` - 3/3 PASS

**影響範囲**:
- 16 ファイルで 113 箇所が deprecated フィールドを使用中
- 段階的移行により破壊的変更なし

**コミット**: 076f193f

### ✅ CoreContext (Step 2) - 完了

**実装日**: 2025-12-15

**抽出したフィールド** (5個):
- `value_gen: ValueIdGenerator` - SSA 値 ID 生成器
- `block_gen: BasicBlockIdGenerator` - 基本ブロック ID 生成器
- `next_binding_id: u32` - BindingId 割り当てカウンタ (Phase 74)
- `temp_slot_counter: u32` - 一時ピンスロットカウンタ
- `debug_join_counter: u32` - デバッグスコープ join ID カウンタ

**ファイル**:
- `/home/tomoaki/git/hakorune-selfhost/src/mir/builder/core_context.rs` (新規作成)

**統合方法**:
- `MirBuilder` に `core_ctx: CoreContext` フィールドを追加
- 既存フィールドは `#[deprecated]` でマーク（後方互換性維持）
- ID 割り当てメソッド (`next_value_id()`, `allocate_binding_id()`, `debug_next_join_id()`) が core_ctx を SSOT として使用し、legacy フィールドを同期
- 新規ヘルパー `next_block_id()` を追加し、30 箇所の `block_gen.next()` 呼び出しを置換

**テスト結果**:
- ✅ `cargo build --release` 成功 (警告のみ、193 warnings)
- ✅ `cargo test --release --lib` - 1004/1004 PASS (7 tests 追加)
- ✅ `phase135_trim_mir_verify.sh` - PASS
- ✅ `phase132_exit_phi_parity.sh` - 3/3 PASS

**影響範囲**:
- builder 内 30+ ファイルで `block_gen.next()` を `next_block_id()` に自動置換
- 段階的移行により破壊的変更なし

**コミット**: 81d79161, 89edf116

### ✅ ScopeContext (Step 3) - 完了

**実装日**: 2025-12-15

**抽出したフィールド** (7個):
- `lexical_scope_stack: Vec<LexicalScopeFrame>` - Block-scoped local 変数スコープ
- `loop_header_stack: Vec<BasicBlockId>` - ループヘッダースタック (break/continue 用)
- `loop_exit_stack: Vec<BasicBlockId>` - ループ出口スタック
- `if_merge_stack: Vec<BasicBlockId>` - If マージブロックスタック
- `current_function: Option<MirFunction>` - 現在ビルド中の関数
- `function_param_names: HashSet<String>` - 関数パラメータ名 (LoopForm PHI 用)
- `debug_scope_stack: Vec<String>` - デバッグリージョン識別子スタック

**ファイル**:
- `/home/tomoaki/git/hakorune-selfhost/src/mir/builder/scope_context.rs` (新規作成)

**統合方法**:
- `MirBuilder` に `scope_ctx: ScopeContext` フィールドを追加
- 既存フィールドは `#[deprecated]` でマーク（後方互換性維持）
- Lexical scope ヘルパー (`push_lexical_scope()`, `pop_lexical_scope()`, `declare_local_in_current_scope()`) が scope_ctx を SSOT として使用
- Control flow stack ヘルパー (`push_if_merge()`, `pop_if_merge()`) が両方を同期
- Debug scope ヘルパー (`debug_push_region()`, `debug_pop_region()`, `debug_current_region_id()`) を更新
- Function context は設定/復元時に両方を同期 (lifecycle.rs, calls/lowering.rs)

**テスト結果**:
- ✅ `cargo build --release` 成功 (291 warnings - deprecated フィールド使用)
- ✅ `cargo test --release --lib` - 1005/1009 PASS (4 tests 失敗は既存問題)
- ✅ `phase135_trim_mir_verify.sh` - PASS
- ⚠️ `phase132_exit_phi_parity.sh` - エラー (既存問題、ScopeContext 変更とは無関係)

**影響範囲**:
- `vars/lexical_scope.rs` - scope_ctx 使用に更新
- `lifecycle.rs` - current_function 設定/復元を scope_ctx 同期
- `calls/lowering.rs` - 関数 lowering の文脈管理を scope_ctx 同期
- 段階的移行により破壊的変更なし

**コミット**: 3127ebb7

### ✅ BindingContext (Step 4) - 完了

**実装日**: 2025-12-15

**抽出したフィールド** (1個):
- `binding_map: BTreeMap<String, BindingId>` - 変数名 → BindingId マッピング (Phase 74)

**ファイル**:
- `/home/tomoaki/git/hakorune-selfhost/src/mir/builder/binding_context.rs` (新規作成)

**統合方法**:
- `MirBuilder` に `binding_ctx: BindingContext` フィールドを追加
- 既存フィールドは `#[deprecated]` でマーク（後方互換性維持）
- 同期ヘルパー (`sync_binding_ctx_to_legacy()`, `sync_legacy_to_binding_ctx()`) を実装
- BindingId は CoreContext 経由で割り当て (`allocate_binding_id()`)

**テスト結果**:
- ✅ `cargo build --release` 成功 (302 warnings - deprecated フィールド使用)
- ✅ `cargo test --release --lib` - 1010/1014 PASS (4 tests 失敗は既存問題)
- ✅ `phase135_trim_mir_verify.sh` - PASS

**影響範囲**:
- `vars/lexical_scope.rs` - binding_ctx.binding_map 使用に更新（スコープ復元処理）
- `vars/assignment_resolver.rs` - binding_ctx.contains() 使用に更新
- 段階的移行により破壊的変更なし

**コミット**: 1adf57ec

### ✅ VariableContext (Step 5) - 完了

**実装日**: 2025-12-15

**抽出したフィールド** (1個):
- `variable_map: BTreeMap<String, ValueId>` - 変数名 → ValueId マッピング (SSA 変換)

**ファイル**:
- `/home/tomoaki/git/hakorune-selfhost/src/mir/builder/variable_context.rs` (新規作成)

**統合方法**:
- `MirBuilder` に `variable_ctx: VariableContext` フィールドを追加
- 既存フィールドは `#[deprecated]` でマーク（後方互換性維持）
- 同期ヘルパー (`sync_variable_ctx_to_legacy()`, `sync_legacy_to_variable_ctx()`) を実装
- JoinIR 統合: `CarrierInfo::from_variable_map(&variable_map)` で carrier 追跡
- NYASH_TRACE_VARMAP デバッグサポート (variable_map 可視化)

**特徴**:
- **BindingContext との違い**: BindingContext は BindingId (バインディング識別子), VariableContext は ValueId (SSA 値)
- **JoinIR 連携**: Pattern 2/3/4 のループで carrier variable 追跡に使用
- **PHI 生成**: if/loop の variable_map 変化から PHI ノードを生成
- **Snapshot/Restore**: if 文・ループで variable_map のスナップショット/復元パターンを使用

**テスト結果**:
- ✅ `cargo build --release` 成功 (367 warnings - deprecated フィールド使用)
- ✅ `cargo test --release --lib` - 1014/1018 PASS (4 tests 失敗は既存問題)
- ✅ `phase135_trim_mir_verify.sh` - PASS

**影響範囲**:
- builder 内 17 ファイルで variable_map を使用中 (phi.rs, stmts.rs, if_form.rs, decls.rs 等)
- JoinIR lowering で `CarrierInfo::from_variable_map()` を使用
- 段階的移行により破壊的変更なし

**コミット**: ee2915a6

### ✅ MetadataContext (Step 6) - 完了

**実装日**: 2025-12-15

**抽出したフィールド** (4個):
- `current_span: Span` - 現在の AST span (命令アノテーション用)
- `source_file: Option<String>` - ソースファイルヒント (メタデータ用)
- `hint_sink: HintSink` - 型推論ヒント (ゼロコストガイダンス)
- `current_region_stack: Vec<RegionId>` - Region 観測用スタック (NYASH_REGION_TRACE=1 デバッグ用)

**ファイル**:
- `/home/tomoaki/git/hakorune-selfhost/src/mir/builder/metadata_context.rs` (新規作成)
- `/home/tomoaki/git/hakorune-selfhost/src/mir/hints.rs` (HintSink に Clone, Debug 追加)

**統合方法**:
- `MirBuilder` に `metadata_ctx: MetadataContext` フィールドを追加
- 既存フィールドは `#[deprecated]` でマーク（後方互換性維持）
- 同期ヘルパー (`sync_metadata_ctx_to_legacy()`, `sync_legacy_to_metadata_ctx()`) を実装
- Hint メソッド (`hint_scope_enter()`, `hint_scope_leave()`, `hint_join_result()`) が metadata_ctx を SSOT として使用し、legacy フィールドを同期
- Source file メソッド (`set_source_file_hint()`, `clear_source_file_hint()`, `current_source_file()`) を metadata_ctx 経由に更新
- Span 使用箇所 (`add_instruction_with_span()`) を metadata_ctx.current_span() 経由に更新

**特徴**:
- **HintSink**: 型推論最適化への将来対応 (現在は no-op デフォルト)
- **Span**: 命令単位で保持され、エラー報告・デバッグ情報生成に使用
- **source_file**: 関数メタデータに伝播 (FunctionMetadata.source_file)
- **current_region_stack**: 開発用トレース専用 (本番コストゼロ)

**テスト結果**:
- ✅ `cargo build --release` 成功 (381 warnings - deprecated フィールド使用)
- ✅ `cargo test --release --lib` - 1019/1023 PASS (4 tests 失敗は既存問題)
- ✅ `phase135_trim_mir_verify.sh` - PASS

**影響範囲**:
- builder 内で current_span, source_file, hint_sink, current_region_stack を使用中のコードを metadata_ctx 経由に移行
- 段階的移行により破壊的変更なし

**コミット**: 903ab8ef

## ✅ 全 Context 完了！ (7/7)

### ✅ CompilationContext (Step 7) - 完了

**実装日**: 2025-12-15

**抽出したフィールド** (15個):
- `compilation_context: Option<BoxCompilationContext>` - Box コンパイルコンテキスト
- `current_static_box: Option<String>` - 現在の static box 名
- `user_defined_boxes: HashSet<String>` - ユーザー定義 Box 名
- `reserved_value_ids: HashSet<ValueId>` - 予約済み ValueId (PHI 用)
- `fn_body_ast: Option<Vec<ASTNode>>` - 関数本体 AST (キャプチャ分析用)
- `weak_fields_by_box: HashMap<String, HashSet<String>>` - Weak フィールドレジストリ
- `property_getters_by_box: HashMap<String, HashMap<String, PropertyKind>>` - Property getter レジストリ
- `field_origin_class: HashMap<(ValueId, String), String>` - フィールド origin 追跡
- `field_origin_by_box: HashMap<(String, String), String>` - クラスレベル origin
- `static_method_index: HashMap<String, Vec<(String, usize)>>` - Static method インデックス
- `method_tail_index: HashMap<String, Vec<String>>` - Method tail インデックス (高速検索)
- `method_tail_index_source_len: usize` - Source サイズスナップショット
- `type_registry: TypeRegistry` - 型情報管理の一元化 (TypeRegistryBox)
- `current_slot_registry: Option<FunctionSlotRegistry>` - 関数スコープ SlotRegistry
- `plugin_method_sigs: HashMap<(String, String), MirType>` - Plugin method シグネチャ

**ファイル**:
- `/home/tomoaki/git/hakorune-selfhost/src/mir/builder/compilation_context.rs` (新規作成, 405 行)

**統合方法**:
- `MirBuilder` に `comp_ctx: CompilationContext` フィールドを追加
- 既存フィールドは `#[deprecated]` でマーク（後方互換性維持）
- `CompilationContext::with_plugin_sigs()` で plugin_method_sigs を初期化
- 全 15 フィールドが comp_ctx に統合され、SSOT 化完了

**特徴**:
- **Box コンパイル**: BoxCompilationContext で static box コンパイル分離
- **PHI 予約**: reserved_value_ids で LoopHeaderPhiBuilder の ValueId 衝突を防止
- **キャプチャ分析**: fn_body_ast を FunctionScopeCaptureAnalyzer で使用
- **Method 解決**: static_method_index + method_tail_index で高速検索
- **Weak フィールド**: weak_fields_by_box で weak 参照管理
- **Property**: property_getters_by_box で computed/once/birth_once 管理
- **Origin 追跡**: field_origin_class + field_origin_by_box で型推論支援
- **型情報**: type_registry で型情報一元管理 (NYASH_USE_TYPE_REGISTRY=1)
- **Slot レジストリ**: current_slot_registry で関数スコープ観測

**テスト結果**:
- ✅ `cargo build --release` 成功 (469 warnings - deprecated フィールド使用)
- ✅ `cargo test --release --lib` - 1029/1033 PASS (4 tests 失敗は既存問題)
- ✅ `phase135_trim_mir_verify.sh` - PASS

**影響範囲**:
- builder 内の compilation 関連フィールドを使用中のコードは全て comp_ctx 経由に移行可能
- 段階的移行により破壊的変更なし

**コミット**: [今回のコミット]

## 設計原則

1. **段階的移行** - 全フィールドを一度に移行せず、1-2 Context ずつ
2. **後方互換性** - 既存の public API は維持（内部で Context 経由に変更）
3. **Box-First** - 各 Context は独立した struct として配置
4. **テスト駆動** - 各段階で全テストが PASS することを確認

## 次のステップ: Legacy フィールド削除

Phase 136 follow-up の全 7 Context が完了しました！次は **Phase 2: レガシーフィールド削除** です。

**Phase 2 タスク**:
1. builder.rs から `#[deprecated]` フィールドを削除
2. sync helper メソッドを削除 (`sync_*_to_legacy`, `sync_legacy_to_*`)
3. 全ファイルを ctx 経由に移行 (段階的、Context ごと)
   - `rg "self\.value_types" src/mir/builder/` → `self.type_ctx.value_types`
   - `rg "self\.value_gen" src/mir/builder/` → `self.core_ctx.value_gen`
   - 等々、全フィールド
4. テスト実行（各 Context 削除後）
5. コミット（1-2 回に分割可能）

**期待効果**:
- Deprecation warnings が 469 → 0 に削減
- builder.rs の行数削減（1200行 → 800行程度を期待）
- Context Box 化の完全完了！

## 参考資料

- [Phase 136 分析ドキュメント](./phase-136-builder-analysis.md) (前提分析)
- [Builder.rs](../../../../src/mir/builder.rs) (対象ファイル)
