# Phase 222.5-D: HashMap/BTreeMap Inventory

**作成日**: 2025-12-10
**目的**: JoinIR パイプライン周辺の HashMap を BTreeMap に統一し、決定性を保証する

---

## 📊 全体サマリー

- **HashMap 使用箇所**: 74 箇所（src/mir 配下）
- **BTreeMap 使用箇所**: 50+ 箇所（Phase 25.1 で PHI 周辺は移行済み）
- **変更対象**: JoinIR パイプライン周辺の HashMap（推定 15-20 箇所）

---

## 🎯 カテゴリ分類

### ✅ Category A: Already BTreeMap (Phase 25.1 完了)

**PHI / Loop / Snapshot 周辺** - 決定性保証済み

- `builder.rs:93` - `variable_map: BTreeMap<String, ValueId>` ✅
- `builder/context.rs:28` - `variable_map: BTreeMap<String, ValueId>` ✅
- `builder/context.rs:35` - `value_origin_newbox: BTreeMap<ValueId, String>` ✅
- `builder/context.rs:42` - `value_types: BTreeMap<ValueId, MirType>` ✅
- `phi_core/phi_builder_box.rs` - 全 PHI 関連マップ ✅
- `phi_core/loop_snapshot_merge.rs` - 全スナップショットマップ ✅
- `phi_core/loopform/builder_core.rs` - 全ループフォームマップ ✅

### 🔶 Category B: JoinIR Pipeline - 要 BTreeMap 化

**JoinIR パイプライン（Pattern/Merge/Boundary/ValueId）周辺**

#### 🔴 優先度高（PHI/ValueId 割り当てに直接影響）

1. **exit_binding 系** (3 箇所)
   - `exit_binding.rs:35` - `variable_map: &'a mut HashMap<String, ValueId>`
   - `exit_binding_constructor.rs:30` - `variable_map: &mut HashMap<String, ValueId>`
   - `exit_binding_applicator.rs:33` - `variable_map: &HashMap<String, ValueId>`

2. **carrier_info 系** (2 箇所)
   - `carrier_info.rs:77` - `variable_map: &HashMap<String, ValueId>`
   - `carrier_info.rs:142` - `variable_map: &HashMap<String, ValueId>`

3. **pattern_pipeline 系** (1 箇所)
   - `pattern_pipeline.rs:105` - `carrier_updates: Option<HashMap<String, UpdateExpr>>`

4. **loop update analyzer 系** (2 箇所)
   - `loop_update_analyzer.rs:84` - `HashMap<String, UpdateExpr>`
   - `loop_update_analyzer.rs:103` - `updates: &mut HashMap<String, UpdateExpr>`

5. **loop with break/continue 系** (2 箇所)
   - `loop_with_break_minimal.rs:142` - `carrier_updates: &HashMap<String, UpdateExpr>`
   - `loop_with_continue_minimal.rs:125` - `carrier_updates: &HashMap<String, UpdateExpr>`

6. **pattern4 carrier analyzer** (1 箇所)
   - `pattern4_carrier_analyzer.rs:89` - `HashMap<String, UpdateExpr>`

7. **condition_env 系** (2 箇所)
   - `condition_env.rs:47` - `name_to_join: HashMap<String, ValueId>`
   - `condition_env.rs:56` - `captured: HashMap<String, ValueId>`

#### 🟡 優先度中（Merge/Boundary 処理）

8. **merge 系** (6 箇所)
   - `instruction_rewriter.rs:47` - `value_to_func_name: &HashMap<ValueId, String>`
   - `instruction_rewriter.rs:48` - `function_params: &HashMap<String, Vec<ValueId>>`
   - `instruction_rewriter.rs:71` - `function_entry_map: HashMap<String, BasicBlockId>`
   - `instruction_rewriter.rs:124` - `local_block_map: HashMap<BasicBlockId, BasicBlockId>`
   - `value_collector.rs:24-25` - 2 HashMap (ValueId → String, String → Vec<ValueId>)
   - `value_collector.rs:36-37` - 2 HashMap (初期化)

9. **joinir_id_remapper 系** (2 箇所)
   - `joinir_id_remapper.rs:14` - `block_map: HashMap<(String, BasicBlockId), BasicBlockId>`
   - `joinir_id_remapper.rs:16` - `value_map: HashMap<ValueId, ValueId>`

10. **boundary injector 系** (2 箇所)
    - `joinir_inline_boundary_injector.rs:58` - `value_map: &HashMap<ValueId, ValueId>`
    - `joinir_inline_boundary_injector.rs:61` - `HashMap<ValueId, ValueId>` (戻り値)

### 🟢 Category C: Keep HashMap（変更不要）

**Builder 内部状態・キャッシュ・最適化パス**

#### Builder 内部（変更不要）

- `builder.rs:109` - `weak_fields_by_box: HashMap<String, HashSet<String>>` - 箱フィールド弱参照管理
- `builder.rs:112` - `property_getters_by_box: HashMap<String, HashMap<String, PropertyKind>>` - プロパティゲッター
- `builder.rs:115` - `field_origin_class: HashMap<(ValueId, String), String>` - フィールド起源追跡
- `builder.rs:117` - `field_origin_by_box: HashMap<(String, String), String>` - 箱別フィールド起源
- `builder.rs:128` - `value_kinds: HashMap<ValueId, MirValueKind>` - 値種別管理
- `builder.rs:140` - `plugin_method_sigs: HashMap<(String, String), MirType>` - プラグインメソッドシグネチャ
- `builder.rs:144` - `static_method_index: HashMap<String, Vec<(String, usize)>>` - 静的メソッドインデックス
- `builder.rs:151` - `method_tail_index: HashMap<String, Vec<String>>` - メソッドテールインデックス
- `builder.rs:218` - `local_ssa_map: HashMap<(BasicBlockId, ValueId, u8), ValueId>` - ローカル SSA マップ
- `builder.rs:220` - `schedule_mat_map: HashMap<(BasicBlockId, ValueId), ValueId>` - スケジュールマット
- `builder.rs:223` - `pin_slot_names: HashMap<ValueId, String>` - ピンスロット名
- `builder.rs:253` - `static_box_singletons: HashMap<String, ValueId>` - 静的箱シングルトン

#### 最適化パス（順序無関係）

- `passes/cse.rs:20` - `expression_map: HashMap<String, ValueId>` - 共通部分式除去
- `passes/method_id_inject.rs:27` - `origin: HashMap<ValueId, String>` - メソッド ID 注入
- `passes/escape.rs:12` - `analysis: HashMap<String, EscapeInfo>` - エスケープ解析
- `optimizer.rs:241` - `def_map` - 定義マップ（最適化パス内）
- `optimizer.rs:293` - `def_map` - 定義マップ（最適化パス内）

#### Verification / Diagnostics（順序無関係）

- `verification/barrier.rs:9` - `def_map` - バリア検証
- `verification/ssa.rs:9` - `definitions: HashMap<ValueId, (BasicBlockId, usize)>` - SSA 定義検証
- `verification/cfg.rs:42` - `phi_dsts_in_block` - CFG 検証
- `verification/utils.rs:4-32` - 全ユーティリティ関数（predecessors, def_blocks, dominators）
- `optimizer_passes/diagnostics.rs:13` - `def_map` - 診断用

#### Runtime / VM（実行時性能優先）

- `join_ir_runner.rs:70` - `locals: HashMap<VarId, JoinValue>` - JoinIR 実行時ローカル変数
- `join_ir_runner.rs:264` - `locals` - 実行時引数
- `join_ir_runner.rs:403` - `locals` - 変数読み取り
- `join_ir_runner.rs:412` - `locals` - 変数引数
- `function.rs:34` - `blocks: HashMap<BasicBlockId, BasicBlock>` - 関数ブロック（MIR 構造）
- `function.rs:388` - `globals: HashMap<String, ConstValue>` - グローバル定数

#### Type Registry（型システム）

- `builder/type_registry.rs:25` - `origins: HashMap<ValueId, String>` - 型起源
- `builder/type_registry.rs:28` - `types: HashMap<ValueId, MirType>` - 型マップ

#### Misc（その他）

- `region/function_slot_registry.rs:39` - `name_to_slot: HashMap<String, SlotId>` - スロットレジストリ
- `slot_registry.rs:18` - `TYPE_IDS: HashMap<String, BoxTypeId>` - グローバル型 ID
- `slot_registry.rs:22` - `EXPLICIT_SLOTS: HashMap<...>` - グローバルスロット
- `slot_registry.rs:28` - `BUILTIN_SLOTS: HashMap<...>` - ビルトインスロット
- `builder/plugin_sigs.rs:8` - `HashMap<(String, String), MirType>` - プラグインシグネチャ
- `join_ir_vm_bridge/joinir_block_converter.rs:20` - `func_name_to_value_id` - VM ブリッジ
- `builder/lifecycle.rs:165` - `main_static` - ライフサイクル管理
- `builder/decls.rs:14,146,205` - 宣言処理（methods HashMap）
- `trim_pattern_lowerer.rs:171` - `variable_map` - テスト用（Empty HashMap）

---

## 🎯 Phase 222.5-D 実装計画

### Step 1: 優先度高（PHI/ValueId 直接影響）- 13 箇所

1. **exit_binding 系** (3 箇所) - `HashMap<String, ValueId>` → `BTreeMap<String, ValueId>`
2. **carrier_info 系** (2 箇所) - `HashMap<String, ValueId>` → `BTreeMap<String, ValueId>`
3. **pattern_pipeline 系** (1 箇所) - `HashMap<String, UpdateExpr>` → `BTreeMap<String, UpdateExpr>`
4. **loop_update_analyzer 系** (2 箇所) - `HashMap<String, UpdateExpr>` → `BTreeMap<String, UpdateExpr>`
5. **loop with break/continue 系** (2 箇所) - `HashMap<String, UpdateExpr>` → `BTreeMap<String, UpdateExpr>`
6. **pattern4_carrier_analyzer** (1 箇所) - `HashMap<String, UpdateExpr>` → `BTreeMap<String, UpdateExpr>`
7. **condition_env 系** (2 箇所) - `HashMap<String, ValueId>` → `BTreeMap<String, ValueId>`

### Step 2: 優先度中（Merge/Boundary）- 10 箇所

8. **merge 系** (6 箇所) - ValueId/String マップの BTreeMap 化
9. **joinir_id_remapper 系** (2 箇所) - ValueId マップの BTreeMap 化
10. **boundary injector 系** (2 箇所) - ValueId マップの BTreeMap 化

### Step 3: テスト・検証

- `cargo test --release --lib` で全テスト PASS 確認
- PHI 決定性テスト（3 回実行で一貫性確認）
- 既存の Phase 222.5-B/C のテストが通ることを確認

---

## 📝 実装ルール

1. **import 統一**: `use std::collections::BTreeMap;` に変更
2. **初期化統一**: `HashMap::new()` → `BTreeMap::new()`
3. **シグネチャ統一**: 関数引数・戻り値の型も BTreeMap に変更
4. **テスト維持**: 既存テストは全て通すこと
5. **段階的実装**: 優先度高から順に実装し、各段階でテスト

---

## 🎯 期待効果

- **決定性保証**: JoinIR パイプライン全体で ValueId 割り当てが決定的に
- **PHI 安定性**: Phase 25.1 の PHI 決定性と統一
- **デバッグ容易性**: ValueId/carrier 順序が予測可能に
- **テスト安定性**: 非決定的テスト失敗の根絶

---

## 📌 注意事項

- **Category C (Keep HashMap) は変更しない** - 性能・順序無関係な箇所
- **Runtime (join_ir_runner.rs) は変更しない** - 実行時性能優先
- **Verification は変更しない** - 検証ツールは順序無関係
- **Builder 内部状態は変更しない** - キャッシュ・インデックスは順序無関係
Status: Active  
Scope: HashMap 在庫調査（JoinIR/JsonParser ライン）
